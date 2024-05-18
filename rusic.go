package main

import (
	"crypto/md5"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	"math/rand"
	"os"
	"strconv"
	"time"
)

type RandomArtStruct struct {
	AlbumId       string
	HttpThumbPath string
}

type SongStruct struct {
	Idx     string
	Path    string
	MusicId string
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

type MusicInfo struct {
	Id           int    // 1
	RusicId      string // 2
	ImgUrl       string // 3
	PlayPath     string // 4
	Artist       string // 5
	Artistid     string // 6
	Album        string // 7
	Albumid      string // 8
	Song         string // 9
	Fullpath     string // 10
	Extension    string // 11
	Idx          string // 12
	Page         string // 13
	FsizeResults string // 14
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
		if err := rows.Scan(&song.Id, &song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist, &song.Artistid, &song.Album,
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

		results = append(results, startsWith)
	}
	println(results)
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
		if err := rows.Scan(&song.Id, &song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist, &song.Artistid, &song.Album,
			&song.Albumid, &song.Song, &song.Fullpath, &song.Extension, &song.Idx, &song.Page,
			&song.FsizeResults); err != nil {
			fmt.Println("song for id Error scanning row: ", err)
			continue
		}
	}

	return song
}

type MusicImgInfo struct {
	Id            int
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

func GetCurrentPlayingImg(albid string) MusicImgInfo {
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
		if err := rows.Scan(&img.Id, &img.RusicId, &img.Width, &img.Height, &img.Artist, &img.Artistid, &img.Album,
			&img.Albumid, &img.Filesize, &img.Fullpath, &img.Thumbpath, &img.Idx, &img.Page,
			&img.HttpThumbPath); err != nil {
			fmt.Println("getcurrentplayingimg Error scanning row: ", err)
			continue
		}
	}
	fmt.Println(img)

	return img
}

type ArtistForAlphaStruct struct {
	Artist   string
	Artistid string
}

func ArtistForAlpha(alpha string) []ArtistForAlphaStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	artist := []ArtistForAlphaStruct{}

	rows, _ := db.Query(fmt.Sprintf("SELECT DISTINCT artist, artistid FROM startswith WHERE artist_first_letter='%s'", alpha))

	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	for rows.Next() {
		var startswith ArtistForAlphaStruct
		if err := rows.Scan(&startswith.Artist, &startswith.Artistid); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		artist = append(artist, startswith)
	}
	fmt.Println(artist)

	return artist

}

type AlbumForAlphaStruct struct {
	Album   string
	Albumid string
}

type AlbumStruct struct {
	Album         string
	Albumid       string
	HttpThumbPath string
}

func AlbumForAlpha(alpha string) []AlbumStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	albums := []AlbumForAlphaStruct{}

	rows, _ := db.Query(fmt.Sprintf("SELECT DISTINCT album, albumid FROM startswith WHERE album_first_letter='%s'", alpha))
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return nil
	}
	for rows.Next() {
		var startswith AlbumForAlphaStruct
		if err := rows.Scan(&startswith.Album, &startswith.Albumid); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		albums = append(albums, startswith)
	}
	fmt.Println(albums)

	albumList := []AlbumStruct{}
	for _, alb := range albums {
		rows, _ := db.Query(fmt.Sprintf("SELECT DISTINCT album, albumid, httpthumbpath FROM music_images WHERE albumid='%s'", alb.Albumid))
		if err != nil {
			fmt.Println("Error executing query: ", err)
			return nil
		}
		for rows.Next() {
			var album AlbumStruct
			if err := rows.Scan(&album.Album, &album.Albumid, &album.HttpThumbPath); err != nil {
				fmt.Println("Error scanning row: ", err)
				continue
			}
			fmt.Println(album)
			albumList = append(albumList, album)
		}
	}

	return albumList
}

type AlbumsForArtistAlbumStruct struct {
	Albumid       string
	Album         string
	HttpThumbPath string
}

func AlbumsForArtist(artid string) []AlbumsForArtistAlbumStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	albums := []AlbumsForArtistAlbumStruct{}

	rows, _ := db.Query(fmt.Sprintf("SELECT DISTINCT albumid, album, httpthumbpath FROM music_images WHERE artistid='%s'", artid))
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return nil
	}
	for rows.Next() {
		var album AlbumsForArtistAlbumStruct
		if err := rows.Scan(&album.Albumid, &album.Album, &album.HttpThumbPath); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		albums = append(albums, album)
	}

	return albums

}

func AlbumsForArtistSongs(albid string) []MusicInfo {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, _ := db.Query(fmt.Sprintf("SELECT * FROM music WHERE albumid='%s'", albid))
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}

	songs := []MusicInfo{}
	for rows.Next() {
		var song MusicInfo
		if err := rows.Scan(&song.Id, &song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist,
			&song.Artistid, &song.Album, &song.Albumid, &song.Song, &song.Fullpath, &song.Extension,
			&song.Idx, &song.Page, &song.FsizeResults); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		fmt.Println(song)
		songs = append(songs, song)
	}

	return songs
}

func SongPages() []string {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, _ := db.Query("SELECT DISTINCT page FROM music")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}

	pages := []string{}
	for rows.Next() {
		var page string
		if err := rows.Scan(&page); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		fmt.Println(page)
		pages = append(pages, page)
	}

	return pages
}

func SongsForPage(page string) []MusicInfo {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, _ := db.Query(fmt.Sprintf("SELECT * FROM music WHERE page='%s'", page))
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}

	songs := []MusicInfo{}
	for rows.Next() {
		var song MusicInfo
		if err := rows.Scan(&song.Id, &song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist,
			&song.Artistid, &song.Album, &song.Albumid, &song.Song, &song.Fullpath, &song.Extension,
			&song.Idx, &song.Page, &song.FsizeResults); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		fmt.Println(song)
		songs = append(songs, song)
	}

	return songs
}

func PlaylistCheck() bool {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
		return false
	}
	defer db.Close()

	rows, err := db.Query("SELECT * FROM playlists")
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return false
	}
	defer rows.Close()

	for rows.Next() {
		return true
	}

	return false

}

type PlaylistStruct struct {
	Id       int
	RusicId  string
	Name     string
	Songs    string
	NumSongs string
}

func create_md5_hash(aname string) string {
	hasher := md5.New()
	hasher.Write([]byte(aname))
	return hex.EncodeToString(hasher.Sum(nil))
}

func CreateEmptyPlaylist(plname string) PlaylistStruct {
	rusicid := create_md5_hash(plname)
	name := plname
	songs := "None"
	numsongs := "0"

	pllist := PlaylistStruct{
		RusicId:  rusicid,
		Name:     name,
		Songs:    songs,
		NumSongs: numsongs,
	}

	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	_, err = db.Exec("INSERT INTO playlists (rusicid, name, songs, numsongs) VALUES (?, ?, ?, ?)", pllist.RusicId, pllist.Name, pllist.Songs, pllist.NumSongs)
	if err != nil {
		fmt.Println("Error inserting playlist: ", err)
	}

	return pllist

}

func CreateRandomPlaylist(plname string, count string) PlaylistStruct {
	rusicid := create_md5_hash(plname)
	name := plname
	numSongs, err := strconv.Atoi(count)
	if err != nil {
		fmt.Println("Error converting count to integer: ", err)
	}

	rand.Seed(time.Now().UnixNano())
	randomNumbers := make([]int, numSongs)
	for i := range randomNumbers {
		randomNumbers[i] = rand.Intn(numSongs + 1) // Intn returns a number in the range [0, n)
	}

	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	songs := []MusicInfo{}

	for _, idx := range randomNumbers {
		rows, err := db.Query(fmt.Sprintf("SELECT * FROM music WHERE idx=%d", idx))
		if err != nil {
			fmt.Println("Error executing query: ", err)
		}
		defer rows.Close()

		for rows.Next() {
			var song MusicInfo
			if err := rows.Scan(&song.Id, &song.RusicId, &song.ImgUrl, &song.PlayPath, &song.Artist,
				&song.Artistid, &song.Album, &song.Albumid, &song.Song, &song.Fullpath, &song.Extension,
				&song.Idx, &song.Page, &song.FsizeResults); err != nil {
				fmt.Println("Error scanning row: ", err)
				continue
			}
			songs = append(songs, song)
		}
	}

	songsJSON, err := json.Marshal(songs)
	if err != nil {
		fmt.Println("Error marshaling songslist[0] to JSON: ", err)
	}

	songsString := string(songsJSON)

	playlistinfo := PlaylistStruct{
		RusicId:  rusicid,
		Name:     name,
		Songs:    songsString,
		NumSongs: count,
	}

	fmt.Println(playlistinfo)

	_, err = db.Exec("INSERT INTO playlists (rusicid, name, songs, numsongs) VALUES (?, ?, ?, ?)", playlistinfo.RusicId, playlistinfo.Name, playlistinfo.Songs, playlistinfo.NumSongs)
	if err != nil {
		fmt.Println("Error inserting playlist: ", err)
	}

	return playlistinfo

}

func AllPlaylists() []PlaylistStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query("SELECT * FROM playlists")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	allplaylist := []PlaylistStruct{}

	for rows.Next() {
		var pl PlaylistStruct
		if err := rows.Scan(&pl.Id, &pl.RusicId, &pl.Name, &pl.Songs, &pl.NumSongs); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		fmt.Println(pl)
		allplaylist = append(allplaylist, pl)
	}

	return allplaylist

}

type NewPlayListStruct struct {
	Id       int
	RusicId  string
	Name     string
	Songs    []MusicInfo
	NumSongs string
}

func SongsForPlaylist(rusicid string) []NewPlayListStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(fmt.Sprintf("SELECT * FROM playlists WHERE rusicid='%s'", rusicid))
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	newPlaylist := []NewPlayListStruct{}

	for rows.Next() {
		var pl PlaylistStruct
		if err := rows.Scan(&pl.Id, &pl.RusicId, &pl.Name, &pl.Songs, &pl.NumSongs); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		fmt.Println(pl)

		var songs []MusicInfo
		err := json.Unmarshal([]byte(pl.Songs), &songs)
		if err != nil {
			fmt.Println("Error unmarshalling songs this is an empty playlist: ", err)
			new_Play_List := NewPlayListStruct{
				Id:       pl.Id,
				RusicId:  pl.RusicId,
				Name:     pl.Name,
				Songs:    []MusicInfo{},
				NumSongs: pl.NumSongs,
			}
			newPlaylist = append(newPlaylist, new_Play_List)
		}

		newplaylist2 := NewPlayListStruct{
			Id:       pl.Id,
			RusicId:  pl.RusicId,
			Name:     pl.Name,
			Songs:    songs,
			NumSongs: pl.NumSongs,
		}

		newPlaylist = append(newPlaylist, newplaylist2)
	}
	fmt.Println(newPlaylist)

	return newPlaylist
}

func DeletePlaylist(rusicid string) []PlaylistStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	_, err = db.Exec(fmt.Sprintf("DELETE FROM playlists WHERE rusicid='%s'", rusicid))
	if err != nil {
		fmt.Println("Error deleting playlist: ", err)
	}

	rows, err := db.Query("SELECT * FROM playlists")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	allplaylist := []PlaylistStruct{}

	for rows.Next() {
		var pl PlaylistStruct
		if err := rows.Scan(&pl.Id, &pl.RusicId, &pl.Name, &pl.Songs, &pl.NumSongs); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		fmt.Println(pl)
		allplaylist = append(allplaylist, pl)
	}

	return allplaylist
}

func RemoveSongFromPlaylist(playlistid string, songid string) []NewPlayListStruct {
	playlist := SongsForPlaylist(playlistid)
	songs := playlist[0].Songs

	for i, song := range songs {
		if song.RusicId == songid {
			songs = append(songs[:i], songs[i+1:]...)
			break
		}
	}

	numsongs := len(songs)

	songsJSON, err := json.Marshal(songs)
	if err != nil {
		fmt.Println("Error marshaling songslist[0] to JSON: ", err)
	}

	songsString := string(songsJSON)

	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	_, err = db.Exec(fmt.Sprintf("UPDATE playlists SET songs='%s' WHERE rusicid='%s'", songsString, playlistid))
	if err != nil {
		fmt.Println("Error updating playlist: ", err)
	}

	_, err = db.Exec(fmt.Sprintf("UPDATE playlists SET numsongs='%d' WHERE rusicid='%s'", numsongs, playlistid))
	if err != nil {
		fmt.Println("Error updating playlist: ", err)
	}

	return SongsForPlaylist(playlistid)

}

func AddSongToPlaylist(playlistid string, songid string) []NewPlayListStruct {
	playlist := SongsForPlaylist(playlistid)
	songs := playlist[0].Songs

	song := SongForId(songid)
	songs = append(songs, song)
	numsongs := len(songs)

	songsJSON, err := json.Marshal(songs)
	if err != nil {
		fmt.Println("Error marshaling songslist[0] to JSON: ", err)
	}

	songsString := string(songsJSON)

	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	_, err = db.Exec(fmt.Sprintf("UPDATE playlists SET songs='%s' WHERE rusicid='%s'", songsString, playlistid))
	if err != nil {
		fmt.Println("Error updating playlist: ", err)
	}

	_, err = db.Exec(fmt.Sprintf("UPDATE playlists SET numsongs='%d' WHERE rusicid='%s'", numsongs, playlistid))
	if err != nil {
		fmt.Println("Error updating playlist: ", err)
	}

	return SongsForPlaylist(playlistid)

}

type PlaylistPlaySonglistStruct struct {
	PlayPath string
	ImgUrl  string
}

func PlayPlaylist(plid string) []PlaylistPlaySonglistStruct {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query("SELECT * FROM playlists where rusicid='plid'")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	var infolist []PlaylistPlaySonglistStruct

	for rows.Next() {
		
		var pl PlaylistStruct
		if err := rows.Scan(&pl.Id, &pl.RusicId, &pl.Name, &pl.Songs, &pl.NumSongs); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}

		fmt.Println(&pl.Songs)

		// var songs []MusicInfo
		// if err := json.Unmarshal([]byte(pl.Songs), &songs); err != nil {
		// 	fmt.Println("Error decoding JSON: ", err)
		// 	continue
		// }

		// fmt.Println(songs)

		

		// for _, song := range songs {
		// 	var info PlaylistPlaySonglistStruct
		// 	info.PlayPath = song.PlayPath
		// 	info.ImgUrl = song.ImgUrl
		// 	fmt.Println(info)
		// 	infolist = append(infolist, info)
		// }
		// Use the decoded songs variable here
		
	}
	return infolist
}
