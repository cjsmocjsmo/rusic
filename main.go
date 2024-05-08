package main

import (
	// "database/sql"
	"fmt"
	"html/template"
	"io"

	// "log"
	"net/http"
	"os"

	"github.com/joho/godotenv"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	_ "github.com/mattn/go-sqlite3"
)

type Template struct {
	templates *template.Template
}

type MovieStruct struct {
	Name          string
	Year          string
	PosterAddr    string
	Size          string
	Path          string
	Idx           string
	MovId         string
	Catagory      string
	HttpThumbPath string
}

type TvEpiStruct struct {
	TvId     string
	Size     string
	Catagory string
	Name     string
	Season   string
	Episode  string
	Path     string
	Idx      string
}

type TVSeasonStruct struct {
	Cat string
	Sea string
	Epi []TvEpiStruct
}

func checkDBExists() {
	mtvDBPath := os.Getenv("RUS_DB_PATH")
	if _, err := os.Stat(mtvDBPath); os.IsNotExist(err) {
		// file does not exist
		fmt.Println("Database file does not exist\n Please run rusicsetup.")
		os.Exit(1)
	} else if err != nil {
		// other error
		fmt.Println("Error checking for database file: ", err)
		os.Exit(1)
	}
	// file exists
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
	t := &Template{
		templates: template.Must(template.ParseGlob("RusicTemplates/*")),
	}
	e.Renderer = t

	e.GET("/", rus_index)
	e.GET("/randomart", rus_index)
	e.GET("/albumofinterest", album_of_interest)
	e.GET("/songsforalbum/:albumid", songs_for_rand_album)
	e.GET("/artiststartswith", rus_artiststartswith)
	

	e.Static("/assets", "assets")
	e.Logger.Fatal(e.Start(":8080"))
}

func (t *Template) Render(w io.Writer, Name string, data interface{}, c echo.Context) error {
	return t.templates.ExecuteTemplate(w, Name, data)
}

func rus_index(c echo.Context) error {
	randart := RandomArt()
	return c.Render(http.StatusOK, "rus_index", randart)
}

func album_of_interest(c echo.Context) error {
	randart := RandomArt()
	return c.Render(http.StatusOK, "rus_albumofinterest", randart)
}

func songs_for_rand_album(c echo.Context) error {
	albumid := c.Param("albumid")
	println("Album ID: ", albumid)
	songs := SongsForAlbum(albumid)
	return c.Render(http.StatusOK, "rus_songsforalbum", songs)
}

func rus_artiststartswith(c echo.Context) error {
	println("Artist Startswith")
	startswith := ArtistStartsWith()
	return c.Render(http.StatusOK, "rus_artiststartswith", startswith)
}
