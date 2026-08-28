// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

package rusic

import (
	"crypto/md5"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"os"
	"strconv"
	"time"

	_ "github.com/mattn/go-sqlite3"
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

// openDB opens the rusic db with foreign_keys enabled on every pooled connection
// (required for ON DELETE CASCADE across albums/songs/album_images/playlist_songs).
func openDB() (*sql.DB, error) {
	db_path := os.Getenv("RUSIC_DB_PATH")
	return sql.Open("sqlite3", db_path+"?_foreign_keys=on")
}

// songSelectQuery joins songs to their album/artist names since the new schema
// no longer stores artist/album name columns directly on songs.
const songSelectQuery = `SELECT s.rusicid, s.imgurl, s.playpath, ar.name, ar.artistid, al.name, al.albumid,
	s.title, s.fullpath, s.extension, s.idx, s.page, s.fsizeresults, s.duration
FROM songs s
JOIN albums al ON al.albumid = s.albumid
JOIN artists ar ON ar.artistid = al.artistid
`

func nullStringValue(value sql.NullString) string {
	if !value.Valid {
		return ""
	}
	return value.String
}

func scanMusicInfo(rows *sql.Rows) (MusicInfo, error) {
	var song MusicInfo
	var (
		rusicID, imgURL, playPath, artist, artistID, album, albumID, songTitle, fullPath, extension, idx, page, fsizeResults, duration sql.NullString
	)

	err := rows.Scan(&rusicID, &imgURL, &playPath, &artist, &artistID, &album, &albumID, &songTitle, &fullPath, &extension, &idx, &page,
		&fsizeResults, &duration)
	if err != nil {
		return song, err
	}

	song.RusicId = nullStringValue(rusicID)
	song.ImgUrl = nullStringValue(imgURL)
	song.PlayPath = nullStringValue(playPath)
	song.Artist = nullStringValue(artist)
	song.Artistid = nullStringValue(artistID)
	song.Album = nullStringValue(album)
	song.Albumid = nullStringValue(albumID)
	song.Song = nullStringValue(songTitle)
	song.Fullpath = nullStringValue(fullPath)
	song.Extension = nullStringValue(extension)
	song.Idx = nullStringValue(idx)
	song.Page = nullStringValue(page)
	song.FsizeResults = nullStringValue(fsizeResults)
	song.Duration = nullStringValue(duration)
	return song, nil
}

func RandomArt() []RandomArtStruct {
	// Open log file
	logPath := os.Getenv("RUSIC_LOG_PATH")
	logFile, err := os.OpenFile(logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Fatal(err)
	}
	defer logFile.Close()

	// Set the output log file
	log.SetOutput(logFile)

	log.Println("RandomArt() called")
	db, err := openDB()
	if err != nil {
		log.Println("Error opening database: ", err)
		return []RandomArtStruct{}
	}
	defer db.Close()

	rows, err := db.Query("SELECT id FROM album_images")
	if err != nil {
		log.Println("Error opening database: ", err)
		return []RandomArtStruct{}
	}
	defer rows.Close()

	idxlist := []int{}
	for rows.Next() {
		var idx int
		if err := rows.Scan(&idx); err != nil {
			log.Println("Error scanning row: %w", err)
		}
		idxlist = append(idxlist, idx)
	}

	if err := rows.Err(); err != nil {
		log.Println("Error iterating over rows: %w", err)
	}

	thumbPaths := []RandomArtStruct{}
	if len(idxlist) == 0 {
		log.Println("No rows found in album_images, returning empty result")
		return thumbPaths
	}

	rand.Seed(time.Now().UnixNano())

	randomNumbers := []int{}
	for i := 0; i < 5; i++ {
		randomIndex := rand.Intn(len(idxlist))
		randomNumbers = append(randomNumbers, idxlist[randomIndex])
	}

	for _, idx := range randomNumbers {
		rows, err := db.Query("SELECT httpthumbpath, albumid FROM album_images WHERE id=?", idx)
		if err != nil {
			log.Println("Error executing query: %w", err)
			continue
		}
		defer rows.Close()

		for rows.Next() {
			var thumbpath, albumid string
			if err := rows.Scan(&thumbpath, &albumid); err != nil {
				log.Println("Error scanning row: %w", err)
			}

			RA := RandomArtStruct{AlbumId: albumid, HttpThumbPath: thumbpath}
			thumbPaths = append(thumbPaths, RA)
		}

		if err := rows.Err(); err != nil {
			log.Println("Error iterating over rows: %w", err)
		}
	}

	log.Println(thumbPaths)

	return thumbPaths
}

type MusicInfo struct {
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
	Duration     string
}

func SongsForAlbum(albumId string) []MusicInfo {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(songSelectQuery+"WHERE s.albumid = ? ORDER BY CAST(s.idx AS INTEGER)", albumId)
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	songs := []MusicInfo{}

	for rows.Next() {
		song, err := scanMusicInfo(rows)
		if err != nil {
			fmt.Println("SongsForAlbum Error scanning row: ", err)
			continue
		}
		songs = append(songs, song)
	}

	return songs
}

type SongCountStruct struct {
	ID    int
	Alpha string
	Count int
}

func ArtistStartsWith() []SongCountStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()
	rows, err := db.Query("SELECT first_letter, COUNT(*) FROM artists GROUP BY first_letter ORDER BY first_letter")
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []SongCountStruct{}
	}
	defer rows.Close()
	results := []SongCountStruct{}
	for rows.Next() {
		var startsWith SongCountStruct
		if err := rows.Scan(&startsWith.Alpha, &startsWith.Count); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		startsWith.ID = len(results) + 1
		results = append(results, startsWith)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}
	return results
}

func AlbumStartsWith() []SongCountStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()
	rows, err := db.Query("SELECT first_letter, COUNT(*) FROM albums GROUP BY first_letter ORDER BY first_letter")
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []SongCountStruct{}
	}
	defer rows.Close()
	results := []SongCountStruct{}
	for rows.Next() {
		var startsWith SongCountStruct
		if err := rows.Scan(&startsWith.Alpha, &startsWith.Count); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		startsWith.ID = len(results) + 1
		results = append(results, startsWith)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}
	return results
}

func SongForId(rusicId string) MusicInfo {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(songSelectQuery+"WHERE s.rusicid = ?", rusicId)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return MusicInfo{}
	}
	defer rows.Close()

	song := MusicInfo{}

	for rows.Next() {
		var err error
		song, err = scanMusicInfo(rows)
		if err != nil {
			fmt.Println("song for id Error scanning row: ", err)
			continue
		}
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return song
}

type MusicImgInfo struct {
	Id            int
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
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	query := `SELECT ai.id, ai.width, ai.height, ar.name, ar.artistid, al.name, al.albumid,
		ai.filesize, ai.fullpath, ai.thumbpath, ai.idx, ai.page, ai.httpthumbpath
	FROM album_images ai
	JOIN albums al ON al.albumid = ai.albumid
	JOIN artists ar ON ar.artistid = al.artistid
	WHERE ai.albumid = ?`
	rows, err := db.Query(query, albid)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return MusicImgInfo{}
	}
	defer rows.Close()

	img := MusicImgInfo{}

	for rows.Next() {
		if err := rows.Scan(&img.Id, &img.Width, &img.Height, &img.Artist, &img.Artistid, &img.Album,
			&img.Albumid, &img.Filesize, &img.Fullpath, &img.Thumbpath, &img.Idx, &img.Page,
			&img.HttpThumbPath); err != nil {
			fmt.Println("getcurrentplayingimg Error scanning row: ", err)
			continue
		}
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return img
}

type ArtistForAlphaStruct struct {
	Artist   string
	Artistid string
}

func ArtistForAlpha(alpha string) []ArtistForAlphaStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	artist := []ArtistForAlphaStruct{}

	rows, err := db.Query("SELECT name, artistid FROM artists WHERE first_letter = ? ORDER BY name COLLATE NOCASE", alpha)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return artist
	}
	defer rows.Close()
	for rows.Next() {
		var startswith ArtistForAlphaStruct
		if err := rows.Scan(&startswith.Artist, &startswith.Artistid); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		artist = append(artist, startswith)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

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
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	albums := []AlbumForAlphaStruct{}

	rows, err := db.Query("SELECT name, albumid FROM albums WHERE first_letter = ? ORDER BY name COLLATE NOCASE", alpha)
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
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}
	rows.Close()

	albumList := []AlbumStruct{}
	for _, alb := range albums {
		rows, err := db.Query("SELECT httpthumbpath FROM album_images WHERE albumid = ? ORDER BY CAST(idx AS INTEGER) LIMIT 1", alb.Albumid)
		if err != nil {
			fmt.Println("Error executing query: ", err)
			continue
		}
		for rows.Next() {
			var thumbpath string
			if err := rows.Scan(&thumbpath); err != nil {
				fmt.Println("Error scanning row: ", err)
				continue
			}
			albumList = append(albumList, AlbumStruct{Album: alb.Album, Albumid: alb.Albumid, HttpThumbPath: thumbpath})
		}
		if err := rows.Err(); err != nil {
			fmt.Println("Error iterating over rows: ", err)
		}
		rows.Close()
	}

	return albumList
}

type AlbumsForArtistAlbumStruct struct {
	Albumid       string
	Album         string
	HttpThumbPath string
}

func AlbumsForArtist(artid string) []AlbumsForArtistAlbumStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	albums := []AlbumsForArtistAlbumStruct{}

	query := `SELECT al.albumid, al.name, ai.httpthumbpath
	FROM albums al
	JOIN album_images ai ON ai.albumid = al.albumid
	WHERE al.artistid = ?
	GROUP BY al.albumid
	ORDER BY al.name COLLATE NOCASE`
	rows, err := db.Query(query, artid)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return nil
	}
	defer rows.Close()
	for rows.Next() {
		var album AlbumsForArtistAlbumStruct
		if err := rows.Scan(&album.Albumid, &album.Album, &album.HttpThumbPath); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		albums = append(albums, album)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return albums

}

func AlbumsForArtistSongs(albid string) []MusicInfo {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(songSelectQuery+"WHERE s.albumid = ? ORDER BY CAST(s.idx AS INTEGER)", albid)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []MusicInfo{}
	}
	defer rows.Close()

	songs := []MusicInfo{}
	for rows.Next() {
		song, err := scanMusicInfo(rows)
		if err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		songs = append(songs, song)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return songs
}

func SongPages() []string {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query("SELECT DISTINCT page FROM songs ORDER BY CAST(page AS INTEGER)")
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []string{}
	}
	defer rows.Close()

	pages := []string{}
	for rows.Next() {
		var page string
		if err := rows.Scan(&page); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		pages = append(pages, page)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return pages
}

func SongsForPage(page string) []MusicInfo {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(songSelectQuery+"WHERE s.page = ? ORDER BY CAST(s.idx AS INTEGER)", page)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []MusicInfo{}
	}
	defer rows.Close()

	songs := []MusicInfo{}
	for rows.Next() {
		song, err := scanMusicInfo(rows)
		if err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		songs = append(songs, song)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return songs
}

func PlaylistCheck() bool {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
		return false
	}
	defer db.Close()

	rows, err := db.Query("SELECT 1 FROM playlists LIMIT 1")
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return false
	}
	defer rows.Close()

	for rows.Next() {
		return true
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	return false

}

// PlaylistStruct's Songs field stays a JSON-encoded string for API compatibility,
// even though the new schema stores membership in playlist_songs, not as a blob.
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

// getPlaylistID resolves the new integer playlists.id from the opaque rusicid.
func getPlaylistID(db *sql.DB, rusicid string) (int, error) {
	var id int
	err := db.QueryRow("SELECT id FROM playlists WHERE rusicid = ?", rusicid).Scan(&id)
	return id, err
}

// fetchPlaylistSongs loads a playlist's songs, joined to album/artist names, in position order.
func fetchPlaylistSongs(db *sql.DB, playlistID int) []MusicInfo {
	query := songSelectQuery + `JOIN playlist_songs ps ON ps.song_rusicid = s.rusicid
WHERE ps.playlist_id = ?
ORDER BY ps.position`
	rows, err := db.Query(query, playlistID)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []MusicInfo{}
	}
	defer rows.Close()

	songs := []MusicInfo{}
	for rows.Next() {
		song, err := scanMusicInfo(rows)
		if err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		songs = append(songs, song)
	}
	return songs
}

func CreateEmptyPlaylist(plname string) PlaylistStruct {
	rusicid := create_md5_hash(plname)
	name := plname

	pllist := PlaylistStruct{
		RusicId:  rusicid,
		Name:     name,
		Songs:    "None",
		NumSongs: "0",
	}

	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	_, err = db.Exec("INSERT INTO playlists (rusicid, name) VALUES (?, ?)", pllist.RusicId, pllist.Name)
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

	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	_, err = db.Exec("INSERT INTO playlists (rusicid, name) VALUES (?, ?)", rusicid, name)
	if err != nil {
		fmt.Println("Error inserting playlist: ", err)
	}

	playlistID, err := getPlaylistID(db, rusicid)
	if err != nil {
		fmt.Println("Error looking up playlist id: ", err)
	}

	rand.Seed(time.Now().UnixNano())
	rows, err := db.Query("SELECT rusicid FROM songs ORDER BY RANDOM() LIMIT ?", numSongs)
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return PlaylistStruct{}
	}
	defer rows.Close()

	songRusicIds := []string{}
	for rows.Next() {
		var rid string
		if err := rows.Scan(&rid); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		songRusicIds = append(songRusicIds, rid)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	songs := []MusicInfo{}
	for i, rid := range songRusicIds {
		_, err := db.Exec("INSERT INTO playlist_songs (playlist_id, song_rusicid, position) VALUES (?, ?, ?)", playlistID, rid, i+1)
		if err != nil {
			fmt.Println("Error inserting playlist_songs row: ", err)
			continue
		}
		songs = append(songs, SongForId(rid))
	}

	songsJSON, err := json.Marshal(songs)
	if err != nil {
		fmt.Println("Error marshaling songslist[0] to JSON: ", err)
	}

	playlistinfo := PlaylistStruct{
		RusicId:  rusicid,
		Name:     name,
		Songs:    string(songsJSON),
		NumSongs: strconv.Itoa(len(songs)),
	}

	return playlistinfo

}

func AllPlaylists() []PlaylistStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query("SELECT id, rusicid, name FROM playlists")
	if err != nil {
		fmt.Println("Error executing query: ", err)
		return []PlaylistStruct{}
	}
	defer rows.Close()

	type playlistRow struct {
		id      int
		rusicid string
		name    string
	}
	basics := []playlistRow{}
	for rows.Next() {
		var pr playlistRow
		if err := rows.Scan(&pr.id, &pr.rusicid, &pr.name); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		basics = append(basics, pr)
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	allplaylist := []PlaylistStruct{}
	for _, pr := range basics {
		songs := fetchPlaylistSongs(db, pr.id)
		songsJSON, err := json.Marshal(songs)
		if err != nil {
			fmt.Println("Error marshaling songs to JSON: ", err)
		}
		allplaylist = append(allplaylist, PlaylistStruct{
			Id:       pr.id,
			RusicId:  pr.rusicid,
			Name:     pr.name,
			Songs:    string(songsJSON),
			NumSongs: strconv.Itoa(len(songs)),
		})
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
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	id, err := getPlaylistID(db, rusicid)
	if err != nil {
		fmt.Println("Error looking up playlist id: ", err)
		return []NewPlayListStruct{}
	}

	var name string
	if err := db.QueryRow("SELECT name FROM playlists WHERE id = ?", id).Scan(&name); err != nil {
		fmt.Println("Error scanning row: ", err)
		return []NewPlayListStruct{}
	}

	songs := fetchPlaylistSongs(db, id)

	return []NewPlayListStruct{{
		Id:       id,
		RusicId:  rusicid,
		Name:     name,
		Songs:    songs,
		NumSongs: strconv.Itoa(len(songs)),
	}}
}

func DeletePlaylist(rusicid string) []PlaylistStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	// playlist_songs rows cascade automatically since openDB enables foreign_keys.
	_, err = db.Exec("DELETE FROM playlists WHERE rusicid = ?", rusicid)
	if err != nil {
		fmt.Println("Error deleting playlist: ", err)
	}

	return AllPlaylists()
}

func RemoveSongFromPlaylist(playlistid string, songid string) []NewPlayListStruct {
	fmt.Println("PlaylistID: ", playlistid)

	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	id, err := getPlaylistID(db, playlistid)
	if err != nil {
		fmt.Println("Error looking up playlist id: ", err)
		return SongsForPlaylist(playlistid)
	}

	_, err = db.Exec("DELETE FROM playlist_songs WHERE playlist_id = ? AND song_rusicid = ?", id, songid)
	if err != nil {
		fmt.Println("Error updating playlist: ", err)
	}

	return SongsForPlaylist(playlistid)

}

func AddSongToPlaylist(playlistid string, songid string) []NewPlayListStruct {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	id, err := getPlaylistID(db, playlistid)
	if err != nil {
		fmt.Println("Error looking up playlist id: ", err)
		return SongsForPlaylist(playlistid)
	}

	var maxPos sql.NullInt64
	if err := db.QueryRow("SELECT MAX(position) FROM playlist_songs WHERE playlist_id = ?", id).Scan(&maxPos); err != nil {
		fmt.Println("Error scanning row: ", err)
	}
	nextPos := 1
	if maxPos.Valid {
		nextPos = int(maxPos.Int64) + 1
	}

	_, err = db.Exec("INSERT INTO playlist_songs (playlist_id, song_rusicid, position) VALUES (?, ?, ?)", id, songid, nextPos)
	if err != nil {
		fmt.Println("Error updating playlist: ", err)
	}

	return SongsForPlaylist(playlistid)

}

func CoverArtFromPlayPath(playpath string) []string {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query(songSelectQuery+"WHERE s.playpath = ?", playpath)
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	var results []string

	for rows.Next() {
		song, err := scanMusicInfo(rows)
		if err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		results = append(results, song.Artist)
		results = append(results, song.Song)
		results = append(results, song.ImgUrl)
	}

	return results

}

func PlayPlaylist(plid string) []MusicInfo {
	db, err := openDB()
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	id, err := getPlaylistID(db, plid)
	if err != nil {
		fmt.Println("Error looking up playlist id: ", err)
		return []MusicInfo{}
	}

	return fetchPlaylistSongs(db, id)
}
