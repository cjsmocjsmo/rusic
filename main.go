package main

import (
	"fmt"
	"net/http"
	"os"

	"github.com/joho/godotenv"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	_ "github.com/mattn/go-sqlite3"
)

func checkDBExists() {
	mtvDBPath := os.Getenv("RUS_DB_PATH")
	if _, err := os.Stat(mtvDBPath); os.IsNotExist(err) {
		fmt.Println("Database file does not exist\n Please run rusicsetup.")
		os.Exit(1)
	} else if err != nil {
		fmt.Println("Error checking for database file: ", err)
		os.Exit(1)
	}
	fmt.Println("Database file exists.")
}

func init() {
	godotenv.Load("rus.env")
	checkDBExists()
}

func main() {

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
	e.GET("/playmusic/:songid", rus_PlayMusic)
	e.GET("/playplaylist/:rusicid", rus_PlayPlayList)
	e.Static("/assets", "assets")
	e.Logger.Fatal(e.Start(":8080"))
}

func rus_index(c echo.Context) error {
	randart := RandomArt()
	return c.JSON(http.StatusOK, randart)
}

func rus_index2(c echo.Context) error {
	randart := RandomArt()
	return c.JSON(http.StatusOK, randart)
}

func rus_main(c echo.Context) error {
	randart := RandomArt()
	return c.JSON(http.StatusOK, randart)
}

func album_of_interest(c echo.Context) error {
	randart := RandomArt()
	return c.JSON(http.StatusOK, randart)
}

func songs_for_rand_album(c echo.Context) error {
	albumid := c.Param("albumid")
	println("Album ID: ", albumid)
	songs := SongsForAlbum(albumid)
	return c.JSON(http.StatusOK, songs)
}

func rus_artist_starts_with(c echo.Context) error {
	println("Artist Startswith")
	startswith := ArtistStartsWith()
	return c.JSON(http.StatusOK, startswith)
}

func rus_album_starts_with(c echo.Context) error {
	println("album Startswith")
	startswith := AlbumStartsWith()
	return c.JSON(http.StatusOK, startswith)
}

func rus_current_playing_img(c echo.Context) error {
	albumid := c.Param("albumid")
	println("AlbumID: ", albumid)
	currentPlaying := GetCurrentPlayingImg(albumid)
	return c.JSON(http.StatusOK, currentPlaying)
}

func rus_artist_for_alpha(c echo.Context) error {
	alphastr := c.Param("alpha")
	println("Alpha: ", alphastr)
	artists := ArtistForAlpha(alphastr)
	return c.JSON(http.StatusOK, artists)
}

func rus_album_for_alpha(c echo.Context) error {
	alphastr := c.Param("alpha")
	println("Alpha: ", alphastr)
	albums := AlbumForAlpha(alphastr)
	return c.JSON(http.StatusOK, albums)
}

func rus_albums_for_artist(c echo.Context) error {
	artistid := c.Param("artistid")
	println("Artist ID: ", artistid)
	albums := AlbumsForArtist(artistid)
	return c.JSON(http.StatusOK, albums)
}

func rus_albums_for_artist_songs(c echo.Context) error {
	albid := c.Param("albumid")
	println("Album ID: ", albid)
	songs := AlbumsForArtistSongs(albid)
	return c.JSON(http.StatusOK, songs)
}

func rus_song_pages(c echo.Context) error {
	println("Song Pages")
	songpages := SongPages()
	return c.JSON(http.StatusOK, songpages)
}

func rus_songs_for_page(c echo.Context) error {
	page := c.Param("page")
	println("Page: ", page)
	songs := SongsForPage(page)
	return c.JSON(http.StatusOK, songs)
}

func rus_playlist_check(c echo.Context) error {
	println("Playlist Check")
	playlist := PlaylistCheck()
	return c.JSON(http.StatusOK, playlist)
}

func rus_create_empty_playlist(c echo.Context) error {
	plname := c.Param("plname")
	println("Create Empty Playlist")
	playlist := CreateEmptyPlaylist(plname)
	return c.JSON(http.StatusOK, playlist)
}

func rus_create_random_playlist(c echo.Context) error {
	plname := c.Param("plname")
	count := c.Param("count")
	println("Create Random Playlist")
	playlist := CreateRandomPlaylist(plname, count)
	return c.JSON(http.StatusOK, playlist)
}

func rus_all_playlists(c echo.Context) error {
	println("All Playlists")
	playlists := AllPlaylists()
	return c.JSON(http.StatusOK, playlists)
}

func rus_edit_playlist(c echo.Context) error {
	rusicid := c.Param("rusicid")
	println("Edit Playlist")
	playlist := SongsForPlaylist(rusicid)
	return c.JSON(http.StatusOK, playlist)
}

func rus_add_song_to_playlist(c echo.Context) error {
	rusicid := c.Param("playlistid")
	songid := c.Param("songid")
	println("Add Song to Playlist")
	playlist := AddSongToPlaylist(rusicid, songid)
	return c.JSON(http.StatusOK, playlist)
}

func rus_remove_song_from_playlist(c echo.Context) error {
	rusicid := c.Param("playlistid")
	songid := c.Param("songid")
	println("Remove Song from Playlist")
	playlist := RemoveSongFromPlaylist(rusicid, songid)
	return c.JSON(http.StatusOK, playlist)
}

func rus_delete_playlist(c echo.Context) error {
	rusicid := c.Param("rusicid")
	println("Delete Playlist")
	playlists := DeletePlaylist(rusicid)
	return c.JSON(http.StatusOK, playlists)
}

func rus_PlayMusic(c echo.Context) error {
	rusicid := c.Param("rusicid")
	println("Song ID: ", rusicid)
	song := SongForId(rusicid)
	return c.Render(http.StatusOK, "rus_playmusic", song)
}

func rus_PlayPlayList(c echo.Context) error {
	rusicid := c.Param("rusicid")
	println("Song ID: ", rusicid)
	plsongs :=  PlayPlaylist(rusicid)
	return c.Render(http.StatusOK, "rus_playmusic", plsongs)
}
