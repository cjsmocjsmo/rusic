// SPDX-FileCopyrightText: 2024 Charlie J Smotherman
//
// SPDX-License-Identifier: GPL-3.0-or-later

package main

import (
	"context"
	"crypto/sha256"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

const transcodeTimeout = 10 * time.Minute

type transcodeJob struct {
	done chan struct{}
	err  error
}

type musicHandler struct {
	root      string
	cache     string
	original  http.Handler
	transcode func(context.Context, string, string) error
	mu        sync.Mutex
	jobs      map[string]*transcodeJob
	slots     chan struct{}
}

func mp3CacheDir() string {
	if dir := os.Getenv("RUSIC_MP3_CACHE_DIR"); dir != "" {
		return dir
	}
	if db := os.Getenv("RUSIC_DB_PATH"); db != "" {
		return filepath.Join(filepath.Dir(db), "mp3-cache")
	}
	return filepath.Join("cache", "mp3")
}

func newMusicHandler(root, cache string) *musicHandler {
	return &musicHandler{
		root:      root,
		cache:     cache,
		original:  http.FileServer(http.Dir(root)),
		transcode: transcodeMP3,
		jobs:      make(map[string]*transcodeJob),
		slots:     make(chan struct{}, 2),
	}
}

func (h *musicHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	formats, specified := r.URL.Query()["format"]
	if specified && (len(formats) != 1 || formats[0] != "mp3") {
		http.Error(w, "unsupported audio format", http.StatusBadRequest)
		return
	}
	if !specified || !strings.EqualFold(filepath.Ext(r.URL.Path), ".flac") {
		h.original.ServeHTTP(w, r)
		return
	}
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		w.Header().Set("Allow", "GET, HEAD")
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	source, info, err := h.source(r.URL.Path)
	if err != nil {
		if os.IsNotExist(err) || os.IsPermission(err) {
			http.NotFound(w, r)
		} else {
			log.Printf("audio source error: %v", err)
			http.Error(w, "unable to access audio", http.StatusInternalServerError)
		}
		return
	}
	// Include encoding settings in the key so a future change does not reuse old output.
	key := fmt.Sprintf("%x", sha256.Sum256(fmt.Appendf(nil, "mp3-192k-v1\x00%s\x00%d\x00%d", source, info.Size(), info.ModTime().UnixNano())))
	cached := filepath.Join(h.cache, key+".mp3")
	if err := h.ensureCached(r.Context(), key, source, cached, info); err != nil {
		if r.Context().Err() != nil {
			return
		}
		if err == errTranscodeBusy {
			w.Header().Set("Retry-After", "5")
			http.Error(w, "audio transcoding is busy; retry later", http.StatusServiceUnavailable)
		} else {
			log.Printf("audio transcoding failed for %q: %v", source, err)
			http.Error(w, "audio transcoding failed", http.StatusInternalServerError)
		}
		return
	}
	f, err := os.Open(cached)
	if err != nil {
		log.Printf("open transcoded audio: %v", err)
		http.Error(w, "unable to open transcoded audio", http.StatusInternalServerError)
		return
	}
	defer f.Close()
	stat, err := f.Stat()
	if err != nil {
		log.Printf("stat transcoded audio: %v", err)
		http.Error(w, "unable to access transcoded audio", http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "audio/mpeg")
	http.ServeContent(w, r, filepath.Base(cached), stat.ModTime(), f)
}

func (h *musicHandler) source(urlPath string) (string, os.FileInfo, error) {
	root, err := filepath.Abs(h.root)
	if err != nil {
		return "", nil, err
	}
	root, err = filepath.EvalSymlinks(root)
	if err != nil {
		return "", nil, err
	}
	// Reject traversal before resolving symlinks; do not let ffmpeg read outside Music.
	for _, part := range strings.Split(urlPath, "/") {
		if part == ".." {
			return "", nil, os.ErrPermission
		}
	}
	path, err := filepath.EvalSymlinks(filepath.Join(root, filepath.FromSlash(strings.TrimPrefix(urlPath, "/"))))
	if err != nil {
		return "", nil, err
	}
	rel, err := filepath.Rel(root, path)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", nil, os.ErrPermission
	}
	info, err := os.Stat(path)
	if err != nil {
		return "", nil, err
	}
	if !info.Mode().IsRegular() {
		return "", nil, os.ErrPermission
	}
	return path, info, nil
}

var errTranscodeBusy = fmt.Errorf("transcode capacity reached")

func (h *musicHandler) ensureCached(ctx context.Context, key, source, cached string, info os.FileInfo) error {
	h.mu.Lock()
	if stat, err := os.Stat(cached); err == nil && stat.Mode().IsRegular() && stat.Size() > 0 {
		h.mu.Unlock()
		return nil
	} else if err != nil && !os.IsNotExist(err) {
		h.mu.Unlock()
		return err
	}
	job := h.jobs[key]
	if job == nil {
		select {
		case h.slots <- struct{}{}:
		default:
			h.mu.Unlock()
			return errTranscodeBusy
		}
		job = &transcodeJob{done: make(chan struct{})}
		h.jobs[key] = job
		go func() {
			start := time.Now()
			log.Printf("transcoding audio %q", source)
			job.err = h.buildCache(source, cached, info)
			log.Printf("transcoding audio %q finished in %s: %v", source, time.Since(start), job.err)
			h.mu.Lock()
			delete(h.jobs, key)
			<-h.slots
			close(job.done)
			h.mu.Unlock()
		}()
	}
	h.mu.Unlock()
	select {
	case <-ctx.Done():
		return ctx.Err()
	case <-job.done:
		return job.err
	}
}

func (h *musicHandler) buildCache(source, cached string, before os.FileInfo) error {
	if err := os.MkdirAll(h.cache, 0755); err != nil {
		return err
	}
	f, err := os.CreateTemp(h.cache, ".transcode-*.mp3")
	if err != nil {
		return err
	}
	temp := f.Name()
	defer os.Remove(temp)
	if err := f.Close(); err != nil {
		return err
	}
	ctx, cancel := context.WithTimeout(context.Background(), transcodeTimeout)
	defer cancel()
	if err := h.transcode(ctx, source, temp); err != nil {
		return err
	}
	after, err := os.Stat(source)
	if err != nil {
		return err
	}
	if !os.SameFile(before, after) || before.Size() != after.Size() || !before.ModTime().Equal(after.ModTime()) {
		return fmt.Errorf("source changed during transcoding")
	}
	output, err := os.Stat(temp)
	if err != nil {
		return err
	}
	if output.Size() == 0 {
		return fmt.Errorf("transcoder produced an empty file")
	}
	return os.Rename(temp, cached)
}

func transcodeMP3(ctx context.Context, source, output string) error {
	cmd := exec.CommandContext(ctx, "ffmpeg", "-nostdin", "-hide_banner", "-loglevel", "error",
		"-i", source, "-vn", "-codec:a", "libmp3lame", "-b:a", "192k", "-f", "mp3", "-y", output)
	if data, err := cmd.CombinedOutput(); err != nil {
		return fmt.Errorf("ffmpeg: %w: %s", err, strings.TrimSpace(string(data)))
	}
	return nil
}
