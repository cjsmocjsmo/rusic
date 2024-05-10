package main

import (
	"database/sql"
	"fmt"
	"math/rand"
	"os"
	"time"

	_ "github.com/mattn/go-sqlite3"
	// "encoding/json"
)

type RandomArtStruct struct {
	AlbumId       string
	HttpThumbPath string
}

type SongStruct struct {
	Idx   string
	Path  string
	MovId string
}

type MusicInfo struct {
	Id		     int
	RusicId      string
	ImgUrl       string
	PlayPath     string
	Artist       string
	Artistid     string
	Album        string
	Albumid      string
	Song         string
	Fullpath     string
	Extension    string
	Idx          string
	Page         string
	FsizeResults string
}

func RandomArt() []RandomArtStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query("SELECT idx FROM music_images")
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer rows.Close()

	idxlist := []int{}
	for rows.Next() {
		var idx int
		if err := rows.Scan(&idx); err != nil {
			fmt.Println("Error scanning row: %w", err)
		}
		idxlist = append(idxlist, idx)
	}

	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: %w", err)
	}

	rand.Seed(time.Now().UnixNano())

	randomNumbers := []int{}
	for i := 0; i < 5; i++ {
		randomIndex := rand.Intn(len(idxlist))
		randomNumbers = append(randomNumbers, idxlist[randomIndex])
	}

	thumbPaths := []RandomArtStruct{}
	for _, idx := range randomNumbers {
		rows, err := db.Query(fmt.Sprintf("SELECT httpthumbpath, albumid FROM music_images WHERE idx=%d", idx))
		if err != nil {
			fmt.Println("Error executing query: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			var thumbpath, albumid string
			if err := rows.Scan(&thumbpath, &albumid); err != nil {
				fmt.Println("Error scanning row: %w", err)
			}

			RA := RandomArtStruct{AlbumId: albumid, HttpThumbPath: thumbpath}
			thumbPaths = append(thumbPaths, RA)
		}

		if err := rows.Err(); err != nil {
			fmt.Println("Error iterating over rows: %w", err)
		}
	}
	// fmt.Println(thumbPaths)
	
	if err != nil {
		fmt.Println("Error marshaling data to JSON: %w", err)
	}

	return thumbPaths
}

func SongsForAlbum(albumId string) []MusicInfo {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(fmt.Sprintf("SELECT * FROM music WHERE albumid='%s'", albumId))
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	songs := []MusicInfo{}

	for rows.Next() {
		song := MusicInfo{}
		if err := rows.Scan(&song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist, &song.Artistid, &song.Album,
			&song.Albumid, &song.Song, &song.Fullpath, &song.Extension, &song.Idx, &song.Page,
			&song.FsizeResults); err != nil {
			fmt.Println("SongsForAlbum Error scanning row: ", err)
			continue
		}
		songs = append(songs, song)
	}

	// fmt.Println(songs)

	return songs
}

type SongCountStruct struct {
	ID    int
	Alpha string
	Count int
}

func ArtistStartsWith() []SongCountStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()
	rows, err := db.Query("SELECT * FROM artistcount")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()
	results := []SongCountStruct{}
	for rows.Next() {
		var startsWith SongCountStruct
		if err := rows.Scan(&startsWith.ID, &startsWith.Alpha, &startsWith.Count); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		println(startsWith.Alpha, startsWith.Count)
		results = append(results, startsWith)
	}
	return results
}

func AlbumStartsWith() []SongCountStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()
	rows, err := db.Query("SELECT * FROM albumcount")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()
	results := []SongCountStruct{}
	for rows.Next() {
		var startsWith SongCountStruct
		if err := rows.Scan(&startsWith.ID, &startsWith.Alpha, &startsWith.Count); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		println(startsWith.Alpha, startsWith.Count)
		results = append(results, startsWith)
	}
	return results
}

func SongForId(rusicId string) MusicInfo {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(fmt.Sprintf("SELECT * FROM music WHERE rusicid='%s'", rusicId))
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	song := MusicInfo{}

	for rows.Next() {
		if err := rows.Scan(&song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist, &song.Artistid, &song.Album,
			&song.Albumid, &song.Song, &song.Fullpath, &song.Extension, &song.Idx, &song.Page,
			&song.FsizeResults); err != nil {
			fmt.Println("song for id Error scanning row: ", err)
			continue
		}
	}

	return song
}

type MusicImgInfo struct {
	RusicId       string
	Width         string
	Height        string
	Artist        string
	Artistid      string
	Album         string
	Albumid       string
	Filesize      string
	Fullpath      string
	Thumbpath     string
	Idx           string
	Page          string
	HttpThumbPath string
}

func get_currentPlayingImg(albid string) MusicImgInfo {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(fmt.Sprintf("SELECT * FROM music_images WHERE albumid='%s'", albid))
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	img := MusicImgInfo{}

	for rows.Next() {
		if err := rows.Scan(&img.RusicId, &img.Width, &img.Height, &img.Artist, &img.Artistid, &img.Album,
			&img.Albumid, &img.Filesize, &img.Fullpath, &img.Thumbpath, &img.Idx, &img.Page,
			&img.HttpThumbPath); err != nil {
			fmt.Println("getcurrentplayingimg Error scanning row: ", err)
			continue
		}
	}
	fmt.Println(img)

	return img
}