use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
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

    log::info!("albumids: {:?}", albumids.len());

    albumids
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumSongs {
    pub albumid: String,
    pub rusicids: String,
    pub index: String,
    pub page: String,
}

pub fn songids_for_albumid(xlist: Vec<String>) -> Vec<AlbumSongs> {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let mut index = 1;
    let mut page = 1;
    let mut albums_songs_vec = Vec::new();
    for x in xlist {
        index += 1;
        if index == 26 {
            page += 1;
            index = 1;
        }
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
            rusicids: vstring,
            index: String::from(index.to_string()),
            page: String::from(page.to_string()),
        };
        albums_songs_vec.push(albumsongs);

    }

    log::info!("albums_songs_vec: {:#?}", albums_songs_vec);

    albums_songs_vec
}

pub fn write_songs_for_album_to_db(albumsongsvec: Vec<AlbumSongs>) -> Result<()> {
    for alb in albumsongsvec {
        let conn = Connection::open("./db/rusic.db").unwrap();

        conn.execute(
            "INSERT INTO songs_for_album (
                    albumid,
                    songs,
                    index,
                    page
                )
                VALUES (?1, ?2, ?3, ?4)",
            (
                &alb.albumid,
                &alb.rusicids,
                &alb.index,
                &alb.page,
            ),
        )?;


    }
    Ok(())
}