package rusic

import (
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestScanMusicInfoNormalizesNullStrings(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("open sqlite: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE artists (
			artistid TEXT,
			name TEXT
		);
		CREATE TABLE albums (
			albumid TEXT,
			artistid TEXT,
			name TEXT
		);
		CREATE TABLE songs (
			rusicid TEXT,
			imgurl TEXT,
			playpath TEXT,
			albumid TEXT,
			title TEXT,
			fullpath TEXT,
			extension TEXT,
			idx TEXT,
			page TEXT,
			fsizeresults TEXT,
			duration TEXT
		);
	`)
	if err != nil {
		t.Fatalf("create schema: %v", err)
	}

	_, err = db.Exec(`
		INSERT INTO artists (artistid, name) VALUES ('artist-1', 'Artist One');
		INSERT INTO albums (albumid, artistid, name) VALUES ('album-1', 'artist-1', 'Album One');
		INSERT INTO songs (
			rusicid, imgurl, playpath, albumid, title, fullpath, extension, idx, page, fsizeresults, duration
		) VALUES (
			'song-1', NULL, 'http://example.com/stream.mp3', 'album-1', 'Song One', '/music/song-one.mp3', '.mp3', '1', '1', '1024', '300'
		);
	`)
	if err != nil {
		t.Fatalf("insert data: %v", err)
	}

	rows, err := db.Query(`
		SELECT s.rusicid, s.imgurl, s.playpath, ar.name, ar.artistid, al.name, al.albumid,
			s.title, s.fullpath, s.extension, s.idx, s.page, s.fsizeresults, s.duration
		FROM songs s
		JOIN albums al ON al.albumid = s.albumid
		JOIN artists ar ON ar.artistid = al.artistid
		WHERE s.rusicid = 'song-1'
	`)
	if err != nil {
		t.Fatalf("query row: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("expected a row")
	}

	song, err := scanMusicInfo(rows)
	if err != nil {
		t.Fatalf("scanMusicInfo: %v", err)
	}

	if song.ImgUrl != "" {
		t.Fatalf("ImgUrl = %q, want empty string for NULL", song.ImgUrl)
	}
	if song.PlayPath != "http://example.com/stream.mp3" {
		t.Fatalf("PlayPath = %q, want http://example.com/stream.mp3", song.PlayPath)
	}
	if song.Artist != "Artist One" {
		t.Fatalf("Artist = %q, want Artist One", song.Artist)
	}
	if song.Song != "Song One" {
		t.Fatalf("Song = %q, want Song One", song.Song)
	}
}
