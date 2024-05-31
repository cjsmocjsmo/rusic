package main

import (
	"fmt"
	"net/http"

	"github.com/cjsmo/cjsmo/rusic/rusic"
	"github.com/joho/godotenv"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	_ "github.com/mattn/go-sqlite3"

	"log"
	"os"
)

// func checkDBExists() {
// 	mtvDBPath := os.Getenv("RUS_DB_PATH")
// 	if _, err := os.Stat(mtvDBPath); os.IsNotExist(err) {
// 		fmt.Println("Database file does not exist\n Please run rusicsetup.")
// 		os.Exit(1)
// 	} else if err != nil {
// 		fmt.Println("Error checking for database file: ", err)
// 		os.Exit(1)
// 	}
// 	fmt.Println("Database file exists.")
// }

	

func init() {
    logFile, err := os.OpenFile("/usr/share/rusic/rusic/log.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
    if err != nil {
        fmt.Println("Error opening log file: ", err)
    }
    log.SetOutput(logFile)

    log.Println("Initializing...")

    err = godotenv.Load()
    if err != nil {
        log.Println("Error loading .env file: ", err)
    }
    log.Println("Initialization complete.")
}

func main() {
	log.Println("Starting main...")
	e := echo.New()
	e.Use(middleware.CORS())
	e.Use(middleware.Gzip())
	// e.Use(middleware.Recover())
	e.GET("/", rus_index)
	e.GET("/main", rus_main)
	e.GET("/randomart", rus_index2)
	e.GET("/albumofinterest", album_of_interest)
	e.GET("/songsforalbum/:albumid", songs_for_rand_album)
	e.GET("/artiststartswith", rus_artist_starts_with)
	e.GET("/albumstartswith", rus_album_starts_with)
	e.GET("/currentPlayingImg/:albumid", rus_current_playing_img)
	e.GET("/artistforalpha/:alpha", rus_artist_for_alpha)
	e.GET("/albumforalpha/:alpha", rus_album_for_alpha)
	e.GET("/albumsforartist/:artistid", rus_albums_for_artist)
	e.GET("/albumsforartistsongs/:albumid", rus_albums_for_artist_songs)
	e.GET("/songpages", rus_song_pages)
	e.GET("/songsforpage/:page", rus_songs_for_page)
	e.GET("/playlistcheck", rus_playlist_check)
	e.GET("/createemptyplaylist/:plname", rus_create_empty_playlist)
	e.GET("/createrandomplaylist/:plname/:count", rus_create_random_playlist)
	e.GET("/allplaylists", rus_all_playlists)
	e.GET("/editplaylistpage/:rusicid", rus_edit_playlist)
	e.GET("/addsongtoplaylist/:playlistid/:songid", rus_add_song_to_playlist)
	e.GET("/removesongfromplaylist/:playlistid/:songid", rus_remove_song_from_playlist)
	e.GET("/deleteplaylist/:rusicid", rus_delete_playlist)
	e.GET("/coverartfromplaypath/:playpath", rus_cover_art_from_playpath)
	e.GET("/playmusic/:songid", rus_PlayMusic)
	e.GET("/playplaylist/:rusicid", rus_PlayPlayList)
	e.Static("/thumbnails", "thumbnails")
	e.Static("/Music", "Music")
	e.Logger.Fatal(e.Start("0.0.0.0:8080"))
}

func rus_index(c echo.Context) error {
    log.Println("Entering rus_index...")
    message := map[string]string{"message": "Hello from rusic"}
    log.Println("Message: ", message)
    return c.JSON(http.StatusOK, message)
}

func rus_index2(c echo.Context) error {
	randart := rusic.RandomArt()
	log.Println("Random Art: ", randart)
	return c.JSON(http.StatusOK, randart)
}

func rus_main(c echo.Context) error {
	randart := rusic.RandomArt()
	log.Println("Random Art: ", randart)
	return c.JSON(http.StatusOK, randart)
}

func album_of_interest(c echo.Context) error {
	randart := rusic.RandomArt()
	return c.JSON(http.StatusOK, randart)
}

func songs_for_rand_album(c echo.Context) error {
	albumid := c.Param("albumid")
	songs := rusic.SongsForAlbum(albumid)
	return c.JSON(http.StatusOK, songs)
}

func rus_artist_starts_with(c echo.Context) error {
	startswith := rusic.ArtistStartsWith()
	return c.JSON(http.StatusOK, startswith)
}

func rus_album_starts_with(c echo.Context) error {
	startswith := rusic.AlbumStartsWith()
	return c.JSON(http.StatusOK, startswith)
}

func rus_current_playing_img(c echo.Context) error {
	albumid := c.Param("albumid")
	currentPlaying := rusic.GetCurrentPlayingImg(albumid)
	return c.JSON(http.StatusOK, currentPlaying)
}

func rus_artist_for_alpha(c echo.Context) error {
	alphastr := c.Param("alpha")
	artists := rusic.ArtistForAlpha(alphastr)
	return c.JSON(http.StatusOK, artists)
}

func rus_album_for_alpha(c echo.Context) error {
	alphastr := c.Param("alpha")
	albums := rusic.AlbumForAlpha(alphastr)
	return c.JSON(http.StatusOK, albums)
}

func rus_albums_for_artist(c echo.Context) error {
	artistid := c.Param("artistid")
	albums := rusic.AlbumsForArtist(artistid)
	return c.JSON(http.StatusOK, albums)
}

func rus_albums_for_artist_songs(c echo.Context) error {
	albid := c.Param("albumid")
	songs := rusic.AlbumsForArtistSongs(albid)
	return c.JSON(http.StatusOK, songs)
}

func rus_song_pages(c echo.Context) error {
	songpages := rusic.SongPages()
	return c.JSON(http.StatusOK, songpages)
}

func rus_songs_for_page(c echo.Context) error {
	page := c.Param("page")
	songs := rusic.SongsForPage(page)
	return c.JSON(http.StatusOK, songs)
}

func rus_playlist_check(c echo.Context) error {
	playlist := rusic.PlaylistCheck()
	return c.JSON(http.StatusOK, playlist)
}

func rus_create_empty_playlist(c echo.Context) error {
	plname := c.Param("plname")
	playlist := rusic.CreateEmptyPlaylist(plname)
	return c.JSON(http.StatusOK, playlist)
}

func rus_create_random_playlist(c echo.Context) error {
	plname := c.Param("plname")
	count := c.Param("count")
	playlist := rusic.CreateRandomPlaylist(plname, count)
	return c.JSON(http.StatusOK, playlist)
}

func rus_all_playlists(c echo.Context) error {
	playlists := rusic.AllPlaylists()
	return c.JSON(http.StatusOK, playlists)
}

func rus_edit_playlist(c echo.Context) error {
	rusicid := c.Param("rusicid")
	playlist := rusic.SongsForPlaylist(rusicid)
	return c.JSON(http.StatusOK, playlist)
}

func rus_add_song_to_playlist(c echo.Context) error {
	rusicid := c.Param("playlistid")
	songid := c.Param("songid")
	playlist := rusic.AddSongToPlaylist(rusicid, songid)
	return c.JSON(http.StatusOK, playlist)
}

func rus_remove_song_from_playlist(c echo.Context) error {
	rusicid := c.Param("playlistid")
	songid := c.Param("songid")
	playlist := rusic.RemoveSongFromPlaylist(rusicid, songid)
	return c.JSON(http.StatusOK, playlist)
}

func rus_delete_playlist(c echo.Context) error {
	rusicid := c.Param("rusicid")
	playlists := rusic.DeletePlaylist(rusicid)
	return c.JSON(http.StatusOK, playlists)
}

func rus_cover_art_from_playpath(c echo.Context) error {
	playpath := c.Param("playpath")
	coverart := rusic.CoverArtFromPlayPath(playpath)
	return c.JSON(http.StatusOK, coverart)
}

func rus_PlayMusic(c echo.Context) error {
	rusicid := c.Param("rusicid")
	song := rusic.SongForId(rusicid)
	return c.Render(http.StatusOK, "rus_playmusic", song)
}

// need a function to get coverart from Playpath

func rus_PlayPlayList(c echo.Context) error {
	rusicid := c.Param("rusicid")
	plsongs := rusic.PlayPlaylist(rusicid)
	return c.JSON(http.StatusOK, plsongs)
}
