package main

import (
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	"math/rand"
	"os"
	"time"
)

type RandomArtStruct struct {
	AlbumId string
	HttpThumbPath string
}

type SongStruct struct {
	Idx string
	Path string
	MovId string
}

type MusicInfo struct{
	id int
    RusicId string
    ImgUrl string
    Artist string
    Artistid string
    Album string	 
	Albumid string
    Song string
    Fullpath string
    Extension string
    Idx string
    Page string
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
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	idxlist := []int{}
	for rows.Next() {
		var idx int
		if err := rows.Scan(&idx); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		// fmt.Println("Index:", idx)
		idxlist = append(idxlist, idx)
	}

	// fmt.Printf("Index list: %v\n", idxlist)

	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	rand.Seed(time.Now().UnixNano())

	randomNumbers := []int{}
	for i := 0; i < 5; i++ {
		randomIndex := rand.Intn(len(idxlist))
		randomNumbers = append(randomNumbers, idxlist[randomIndex])
	}

	// fmt.Printf("Random numbers: %v\n", randomNumbers)

	thumbPaths := []RandomArtStruct{}
	for _, idx := range randomNumbers {
		rows, err := db.Query(fmt.Sprintf("SELECT httpthumbpath, albumid FROM music_images WHERE idx=%d", idx))
		if err != nil {
			fmt.Println("Error executing query: ", err)
			continue
		}
		defer rows.Close()

		for rows.Next() {
			var thumbpath, albumid string
			if err := rows.Scan(&thumbpath, &albumid); err != nil {
				fmt.Println("Error scanning row: ", err)
				continue
			}

			RA := RandomArtStruct{AlbumId: albumid, HttpThumbPath: thumbpath}
			thumbPaths = append(thumbPaths, RA)
		}

		if err := rows.Err(); err != nil {
			fmt.Println("Error iterating over rows: ", err)
		}
	}
	// fmt.Printf("Thumb paths: %v\n", thumbPaths)

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
		if err := rows.Scan(&song.id, &song.RusicId, &song.ImgUrl, &song.Artist, &song.Artistid, &song.Album, 
			&song.Albumid, &song.Song, &song.Fullpath, &song.Extension, &song.Idx, &song.Page, 
			&song.FsizeResults); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		songs = append(songs, song)
	}

	return songs
		
}
// type MusicInfo struct{
//     RusicId string
//     ImgUrl string
//     Artist string
//     Artistid string
//     Album string	 
// 	Albumid string
//     Song string
//     Fullpath string
//     Extension string
//     Idx string
//     Page string
//     FsizeResults string
// }