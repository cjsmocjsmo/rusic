use rusqlite::Connection;
use serde_json;

pub fn unique_albumids() -> Vec<String> {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT DISTINCT albumid FROM music;")
        .unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut albumids: Vec<String> = Vec::new();
    for row in rows {
        albumids.push(row.unwrap());
    }
    // println!("albumids: {:?}", albumids.len());

    albumids
}

#[derive(Debug)]
pub struct AlbumSongs {
    pub albumid: String,
    pub songids: String,
}

pub fn songids_for_albumid(xlist: Vec<String>) -> Vec<AlbumSongs> {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let mut albums_songs_vec = Vec::new();
    for x in xlist {
        let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT rusicid FROM music WHERE albumid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&x]).expect("Unable to query db");

        let mut songids: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            songids.push(row.get(0).unwrap());
        };
        let vstring = serde_json::to_string(&songids).unwrap();
        let albumsongs = AlbumSongs {
            albumid: x,
            songids: vstring,
        };
        albums_songs_vec.push(albumsongs);

    }

    println!("albums_songs_vec: {:#?}", albums_songs_vec);

    albums_songs_vec
}