package main

import (
	"bytes"
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func testMusicHandler(t *testing.T) (*musicHandler, string) {
	t.Helper()
	root := t.TempDir()
	path := filepath.Join(root, "song.flac")
	if err := os.WriteFile(path, []byte("original flac"), 0644); err != nil {
		t.Fatal(err)
	}
	return newMusicHandler(root, t.TempDir()), path
}

func musicRequest(h http.Handler, method, target string) *httptest.ResponseRecorder {
	w := httptest.NewRecorder()
	h.ServeHTTP(w, httptest.NewRequest(method, target, nil))
	return w
}

func TestMusicFormatsAndCache(t *testing.T) {
	h, source := testMusicHandler(t)
	var calls int
	h.transcode = func(_ context.Context, input, output string) error {
		calls++
		data, err := os.ReadFile(input)
		if err != nil {
			return err
		}
		return os.WriteFile(output, append([]byte("mp3:"), data...), 0600)
	}

	w := musicRequest(h, "GET", "/song.flac")
	if w.Code != http.StatusOK || w.Body.String() != "original flac" || calls != 0 {
		t.Fatalf("original: status=%d body=%q conversions=%d", w.Code, w.Body.String(), calls)
	}
	for _, target := range []string{"/song.flac?format=ogg", "/song.flac?format=", "/song.flac?format=mp3&format=mp3"} {
		if w := musicRequest(h, "GET", target); w.Code != http.StatusBadRequest {
			t.Errorf("%s: status=%d", target, w.Code)
		}
	}
	if err := os.WriteFile(filepath.Join(h.root, "song.mp3"), []byte("already mp3"), 0644); err != nil {
		t.Fatal(err)
	}
	w = musicRequest(h, "GET", "/song.mp3?format=mp3")
	if w.Code != http.StatusOK || w.Body.String() != "already mp3" || calls != 0 {
		t.Fatalf("non-FLAC: status=%d body=%q conversions=%d", w.Code, w.Body.String(), calls)
	}
	for i := 0; i < 2; i++ {
		w = musicRequest(h, "GET", "/song.flac?other=x&format=mp3")
		if w.Code != http.StatusOK || w.Body.String() != "mp3:original flac" || w.Header().Get("Content-Type") != "audio/mpeg" {
			t.Fatalf("converted: status=%d body=%q headers=%v", w.Code, w.Body.String(), w.Header())
		}
	}
	if calls != 1 {
		t.Fatalf("conversions=%d, want 1", calls)
	}
	req := httptest.NewRequest("GET", "/song.flac?format=mp3", nil)
	req.Header.Set("Range", "bytes=0-3")
	w = httptest.NewRecorder()
	h.ServeHTTP(w, req)
	if w.Code != http.StatusPartialContent || w.Body.String() != "mp3:" || w.Header().Get("Content-Range") != "bytes 0-3/17" {
		t.Fatalf("range: status=%d body=%q headers=%v", w.Code, w.Body.String(), w.Header())
	}
	w = musicRequest(h, "HEAD", "/song.flac?format=mp3")
	if w.Code != http.StatusOK || w.Body.Len() != 0 || w.Header().Get("Content-Length") != "17" {
		t.Fatalf("HEAD: status=%d body=%q headers=%v", w.Code, w.Body.String(), w.Header())
	}
	if err := os.WriteFile(source, []byte("changed"), 0644); err != nil {
		t.Fatal(err)
	}
	w = musicRequest(h, "GET", "/song.flac?format=mp3")
	if w.Code != http.StatusOK || w.Body.String() != "mp3:changed" || calls != 2 {
		t.Fatalf("changed: status=%d body=%q conversions=%d", w.Code, w.Body.String(), calls)
	}
}

func TestMusicTranscodeErrorsAndPaths(t *testing.T) {
	h, _ := testMusicHandler(t)
	h.transcode = func(_ context.Context, _, _ string) error { return fmt.Errorf("encoder failed") }
	for i := 0; i < 2; i++ {
		w := musicRequest(h, "GET", "/song.flac?format=mp3")
		if w.Code != http.StatusInternalServerError || strings.Contains(w.Body.String(), "original flac") {
			t.Fatalf("failed conversion: status=%d body=%q", w.Code, w.Body.String())
		}
	}
	entries, err := os.ReadDir(h.cache)
	if err != nil || len(entries) != 0 {
		t.Fatalf("failed conversion left cache entries: %v, %v", entries, err)
	}
	w := musicRequest(h, "GET", "/missing.flac?format=mp3")
	if w.Code != http.StatusNotFound {
		t.Fatalf("missing file: status=%d", w.Code)
	}
	w = musicRequest(h, "POST", "/song.flac?format=mp3")
	if w.Code != http.StatusMethodNotAllowed {
		t.Fatalf("POST: status=%d", w.Code)
	}
	outside := filepath.Join(t.TempDir(), "outside.flac")
	if err := os.WriteFile(outside, []byte("outside"), 0644); err != nil {
		t.Fatal(err)
	}
	if err := os.Symlink(outside, filepath.Join(h.root, "link.flac")); err != nil {
		t.Fatal(err)
	}
	for _, target := range []string{"/link.flac?format=mp3", "/../outside.flac?format=mp3"} {
		w = musicRequest(h, "GET", target)
		if w.Code != http.StatusNotFound {
			t.Errorf("%s: status=%d", target, w.Code)
		}
	}
}

func TestMusicConcurrentRequestsAndSourceChange(t *testing.T) {
	h, source := testMusicHandler(t)
	started := make(chan struct{})
	release := make(chan struct{})
	var calls atomic.Int32
	h.transcode = func(_ context.Context, _, output string) error {
		if calls.Add(1) == 1 {
			close(started)
			<-release
		}
		return os.WriteFile(output, []byte("converted"), 0600)
	}
	var wg sync.WaitGroup
	responses := make(chan int, 8)
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			responses <- musicRequest(h, "GET", "/song.flac?format=mp3").Code
		}()
	}
	<-started
	close(release)
	wg.Wait()
	close(responses)
	for status := range responses {
		if status != http.StatusOK {
			t.Errorf("concurrent status=%d", status)
		}
	}
	if calls.Load() != 1 {
		t.Fatalf("conversions=%d, want 1", calls.Load())
	}

	h2 := newMusicHandler(h.root, t.TempDir())
	h2.transcode = func(_ context.Context, _, output string) error {
		if err := os.WriteFile(source, []byte("changed during conversion"), 0644); err != nil {
			return err
		}
		return os.WriteFile(output, []byte("stale"), 0600)
	}
	w := musicRequest(h2, "GET", "/song.flac?format=mp3")
	if w.Code != http.StatusInternalServerError {
		t.Fatalf("changed during conversion: status=%d", w.Code)
	}
	entries, err := os.ReadDir(h2.cache)
	if err != nil || len(entries) != 0 {
		t.Fatalf("stale cache entries: %v, %v", entries, err)
	}
}

func TestMusicGzipDoesNotCorruptRange(t *testing.T) {
	h, _ := testMusicHandler(t)
	h.transcode = func(_ context.Context, _, output string) error {
		return os.WriteFile(output, []byte("converted"), 0600)
	}
	mux := http.NewServeMux()
	mux.Handle("/Music/", http.StripPrefix("/Music/", h))
	req := httptest.NewRequest("GET", "/Music/song.flac?format=mp3", nil)
	req.Header.Set("Range", "bytes=0-3")
	req.Header.Set("Accept-Encoding", "gzip")
	w := httptest.NewRecorder()
	withGzip(mux).ServeHTTP(w, req)
	if w.Code != http.StatusPartialContent || w.Body.String() != "conv" || w.Header().Get("Content-Encoding") != "" {
		t.Fatalf("gzip range: status=%d body=%q headers=%v", w.Code, w.Body.String(), w.Header())
	}
}

func TestFFmpegConversion(t *testing.T) {
	if _, err := exec.LookPath("ffmpeg"); err != nil {
		t.Skip("ffmpeg is not installed")
	}
	if _, err := exec.LookPath("ffprobe"); err != nil {
		t.Skip("ffprobe is not installed")
	}
	h, source := testMusicHandler(t)
	cmd := exec.Command("ffmpeg", "-nostdin", "-hide_banner", "-loglevel", "error", "-f", "lavfi",
		"-i", "sine=frequency=440:duration=1", "-y", source)
	if output, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("generate FLAC: %v: %s", err, output)
	}
	w := musicRequest(h, "GET", "/song.flac?format=mp3")
	if w.Code != http.StatusOK {
		t.Fatalf("conversion: status=%d body=%q", w.Code, w.Body.String())
	}
	mp3 := filepath.Join(t.TempDir(), "result.mp3")
	if err := os.WriteFile(mp3, w.Body.Bytes(), 0600); err != nil {
		t.Fatal(err)
	}
	cmd = exec.Command("ffprobe", "-v", "error", "-select_streams", "a:0", "-show_entries", "stream=codec_name",
		"-of", "default=noprint_wrappers=1:nokey=1", mp3)
	output, err := cmd.Output()
	if err != nil || !bytes.Equal(bytes.TrimSpace(output), []byte("mp3")) {
		t.Fatalf("probe result: %q, %v", output, err)
	}
}

func TestMusicBusyAndCancelledRequest(t *testing.T) {
	h, _ := testMusicHandler(t)
	started := make(chan struct{}, 2)
	release := make(chan struct{})
	h.transcode = func(_ context.Context, _, output string) error {
		started <- struct{}{}
		<-release
		return os.WriteFile(output, []byte("converted"), 0600)
	}
	for _, name := range []string{"second.flac", "third.flac"} {
		if err := os.WriteFile(filepath.Join(h.root, name), []byte(name), 0644); err != nil {
			t.Fatal(err)
		}
	}
	done := make(chan struct{}, 2)
	for _, name := range []string{"song.flac", "second.flac"} {
		go func(name string) {
			musicRequest(h, "GET", "/"+name+"?format=mp3")
			done <- struct{}{}
		}(name)
	}
	<-started
	<-started
	w := musicRequest(h, "GET", "/third.flac?format=mp3")
	if w.Code != http.StatusServiceUnavailable || w.Header().Get("Retry-After") != "5" {
		t.Errorf("busy status=%d headers=%v", w.Code, w.Header())
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	req := httptest.NewRequest("GET", "/song.flac?format=mp3", nil).WithContext(ctx)
	h.ServeHTTP(httptest.NewRecorder(), req)
	close(release)
	for i := 0; i < 2; i++ {
		select {
		case <-done:
		case <-time.After(5 * time.Second):
			t.Fatal("conversion did not finish")
		}
	}
}
