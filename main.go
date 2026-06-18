// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

package main

import (
	"compress/gzip"
	"encoding/json"
	"fmt"
	"github.com/cjsmo/cjsmo/rusic/rusic"
	_ "github.com/mattn/go-sqlite3"
	"log"
	"net/http"
	"os"
	"strings"
)

func init() {
	logPath := os.Getenv("RUSIC_LOG_PATH")
	logFile, err := os.OpenFile(logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println("Error opening log file: ", err)
	} else {
		log.SetOutput(logFile)
	}

	log.Println("Initializing...")
}

func main() {
	log.Println("Starting main...")
	r := newRouter()
	r.GET("/", rus_index)
	r.GET("/main", rus_main)
	r.GET("/randomart", rus_index2)
	r.GET("/albumofinterest", album_of_interest)
	r.GET("/songsforalbum/:albumid", songs_for_rand_album)
	r.GET("/artiststartswith", rus_artist_starts_with)
	r.GET("/albumstartswith", rus_album_starts_with)
	r.GET("/currentPlayingImg/:albumid", rus_current_playing_img)
	r.GET("/artistforalpha/:alpha", rus_artist_for_alpha)
	r.GET("/albumforalpha/:alpha", rus_album_for_alpha)
	r.GET("/albumsforartist/:artistid", rus_albums_for_artist)
	r.GET("/albumsforartistsongs/:albumid", rus_albums_for_artist_songs)
	r.GET("/songpages", rus_song_pages)
	r.GET("/songsforpage/:page", rus_songs_for_page)
	r.GET("/playlistcheck", rus_playlist_check)
	r.GET("/createemptyplaylist/:plname", rus_create_empty_playlist)
	r.GET("/createrandomplaylist/:plname/:count", rus_create_random_playlist)
	r.GET("/allplaylists", rus_all_playlists)
	r.GET("/editplaylistpage/:rusicid", rus_edit_playlist)
	r.GET("/addsongtoplaylist/:playlistid/:songid", rus_add_song_to_playlist)
	r.GET("/removesongfromplaylist/:playlistid/:songid", rus_remove_song_from_playlist)
	r.GET("/deleteplaylist/:rusicid", rus_delete_playlist)
	r.GET("/coverartfromplaypath/:playpath", rus_cover_art_from_playpath)
	r.GET("/playmusic/:songid", rus_PlayMusic)
	r.GET("/playplaylist/:rusicid", rus_PlayPlayList)

	mux := http.NewServeMux()
	mux.Handle("/thumbs/", http.StripPrefix("/thumbs/", http.FileServer(http.Dir("thumbs"))))
	mux.Handle("/Music/", http.StripPrefix("/Music/", http.FileServer(http.Dir("Music"))))
	mux.Handle("/", r)

	h := withRecover(withCORS(withGzip(mux)))
	// hex := os.Getenv("RUSIC_RAW_HTTP")
	port := os.Getenv("RUSIC_PORT")
	addr := fmt.Sprintf("%s:%s", "0.0.0.0", port)
	// addr := os.Getenv("RUSIC_ADDR")
	// if addr == "" {
	// 	addr = "0.0.0.0:8080"
	// }
	log.Fatal(http.ListenAndServe(addr, h))
}

type routeHandler func(http.ResponseWriter, *http.Request, map[string]string)

type route struct {
	method  string
	pattern string
	handler routeHandler
}

type router struct {
	routes []route
}

func newRouter() *router {
	return &router{routes: []route{}}
}

func (r *router) GET(pattern string, handler routeHandler) {
	r.routes = append(r.routes, route{method: http.MethodGet, pattern: pattern, handler: handler})
}

func (r *router) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	pathMatched := false
	for _, rt := range r.routes {
		params, matched := matchPath(rt.pattern, req.URL.Path)
		if !matched {
			continue
		}
		pathMatched = true
		if req.Method != rt.method {
			continue
		}
		rt.handler(w, req, params)
		return
	}

	if pathMatched {
		http.Error(w, http.StatusText(http.StatusMethodNotAllowed), http.StatusMethodNotAllowed)
		return
	}
	http.NotFound(w, req)
}

func matchPath(pattern, path string) (map[string]string, bool) {
	patternParts := splitPath(pattern)
	pathParts := splitPath(path)
	if len(patternParts) != len(pathParts) {
		return nil, false
	}

	params := make(map[string]string)
	for i := range patternParts {
		p := patternParts[i]
		v := pathParts[i]
		if strings.HasPrefix(p, ":") {
			params[strings.TrimPrefix(p, ":")] = v
			continue
		}
		if p != v {
			return nil, false
		}
	}
	return params, true
}

func splitPath(p string) []string {
	trimmed := strings.Trim(p, "/")
	if trimmed == "" {
		return []string{}
	}
	return strings.Split(trimmed, "/")
}

func withCORS(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")
		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}
		next.ServeHTTP(w, r)
	})
}

func withRecover(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if rec := recover(); rec != nil {
				log.Printf("panic recovered: %v", rec)
				http.Error(w, http.StatusText(http.StatusInternalServerError), http.StatusInternalServerError)
			}
		}()
		next.ServeHTTP(w, r)
	})
}

func withGzip(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !strings.Contains(r.Header.Get("Accept-Encoding"), "gzip") {
			next.ServeHTTP(w, r)
			return
		}

		gw := gzip.NewWriter(w)
		defer gw.Close()

		w.Header().Set("Content-Encoding", "gzip")
		w.Header().Add("Vary", "Accept-Encoding")
		gzw := &gzipResponseWriter{ResponseWriter: w, writer: gw}
		next.ServeHTTP(gzw, r)
	})
}

type gzipResponseWriter struct {
	http.ResponseWriter
	writer *gzip.Writer
}

func (w *gzipResponseWriter) Write(data []byte) (int, error) {
	return w.writer.Write(data)
}

func writeJSON(w http.ResponseWriter, status int, payload any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(payload); err != nil {
		log.Printf("failed to encode JSON response: %v", err)
	}
}

func rus_index(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	log.Println("Entering rus_index...")
	message := map[string]string{"message": "Hello from rusic"}
	log.Println("Message: ", message)
	writeJSON(w, http.StatusOK, message)
}

func rus_index2(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	randart := rusic.RandomArt()
	log.Println("Random Art: ", randart)
	writeJSON(w, http.StatusOK, randart)
}

func rus_main(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	randart := rusic.RandomArt()
	log.Println("Random Art: ", randart)
	writeJSON(w, http.StatusOK, randart)
}

func album_of_interest(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	randart := rusic.RandomArt()
	writeJSON(w, http.StatusOK, randart)
}

func songs_for_rand_album(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	albumid := params["albumid"]
	songs := rusic.SongsForAlbum(albumid)
	writeJSON(w, http.StatusOK, songs)
}

func rus_artist_starts_with(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	startswith := rusic.ArtistStartsWith()
	writeJSON(w, http.StatusOK, startswith)
}

func rus_album_starts_with(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	startswith := rusic.AlbumStartsWith()
	writeJSON(w, http.StatusOK, startswith)
}

func rus_current_playing_img(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	albumid := params["albumid"]
	currentPlaying := rusic.GetCurrentPlayingImg(albumid)
	writeJSON(w, http.StatusOK, currentPlaying)
}

func rus_artist_for_alpha(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	alphastr := params["alpha"]
	artists := rusic.ArtistForAlpha(alphastr)
	writeJSON(w, http.StatusOK, artists)
}

func rus_album_for_alpha(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	alphastr := params["alpha"]
	albums := rusic.AlbumForAlpha(alphastr)
	writeJSON(w, http.StatusOK, albums)
}

func rus_albums_for_artist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	artistid := params["artistid"]
	albums := rusic.AlbumsForArtist(artistid)
	writeJSON(w, http.StatusOK, albums)
}

func rus_albums_for_artist_songs(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	albid := params["albumid"]
	songs := rusic.AlbumsForArtistSongs(albid)
	writeJSON(w, http.StatusOK, songs)
}

func rus_song_pages(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	songpages := rusic.SongPages()
	writeJSON(w, http.StatusOK, songpages)
}

func rus_songs_for_page(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	page := params["page"]
	songs := rusic.SongsForPage(page)
	writeJSON(w, http.StatusOK, songs)
}

func rus_playlist_check(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	playlist := rusic.PlaylistCheck()
	writeJSON(w, http.StatusOK, playlist)
}

func rus_create_empty_playlist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	plname := params["plname"]
	playlist := rusic.CreateEmptyPlaylist(plname)
	writeJSON(w, http.StatusOK, playlist)
}

func rus_create_random_playlist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	plname := params["plname"]
	count := params["count"]
	playlist := rusic.CreateRandomPlaylist(plname, count)
	writeJSON(w, http.StatusOK, playlist)
}

func rus_all_playlists(w http.ResponseWriter, _ *http.Request, _ map[string]string) {
	playlists := rusic.AllPlaylists()
	writeJSON(w, http.StatusOK, playlists)
}

func rus_edit_playlist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	rusicid := params["rusicid"]
	playlist := rusic.SongsForPlaylist(rusicid)
	writeJSON(w, http.StatusOK, playlist)
}

func rus_add_song_to_playlist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	rusicid := params["playlistid"]
	songid := params["songid"]
	playlist := rusic.AddSongToPlaylist(rusicid, songid)
	writeJSON(w, http.StatusOK, playlist)
}

func rus_remove_song_from_playlist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	rusicid := params["playlistid"]
	songid := params["songid"]
	playlist := rusic.RemoveSongFromPlaylist(rusicid, songid)
	writeJSON(w, http.StatusOK, playlist)
}

func rus_delete_playlist(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	rusicid := params["rusicid"]
	playlists := rusic.DeletePlaylist(rusicid)
	writeJSON(w, http.StatusOK, playlists)
}

func rus_cover_art_from_playpath(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	playpath := params["playpath"]
	coverart := rusic.CoverArtFromPlayPath(playpath)
	writeJSON(w, http.StatusOK, coverart)
}

func rus_PlayMusic(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	songid := params["songid"]
	song := rusic.SongForId(songid)
	writeJSON(w, http.StatusOK, song)
}

// need a function to get coverart from Playpath

func rus_PlayPlayList(w http.ResponseWriter, _ *http.Request, params map[string]string) {
	rusicid := params["rusicid"]
	plsongs := rusic.PlayPlaylist(rusicid)
	writeJSON(w, http.StatusOK, plsongs)
}
