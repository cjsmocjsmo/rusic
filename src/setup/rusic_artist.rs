use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

pub fn unique_artistids() -> Vec<String> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT DISTINCT artistid FROM music;")
        .unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut artistids: Vec<String> = Vec::new();
    for row in rows {
        artistids.push(row.unwrap());
    }
    log::info!("artistids: {:?}", artistids.len());

    artistids
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistAlbums {
    pub page: i32,
    pub artistid: String,
    pub albums: String,

}

pub fn albumids_for_artistid(xlist: Vec<String>) -> Vec<ArtistAlbums> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let mut pge = 1;
    let mut index = 1;
    let mut artists_albums_vec = Vec::new();
    for x in xlist {
        log::info!("x: {:#?}", x);
        println!("x: {:#?}", x);
        index += 1;
        if index == 26 {
            pge += 1;
            index = 1;
        }
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT DISTINCT albumid FROM music WHERE artistid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&x]).expect("Unable to query db");

        let mut albumids: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            albumids.push(row.get(0).unwrap());
        }
        let vstring = serde_json::to_string(&albumids).unwrap();
        let artistalbums = ArtistAlbums {
            page: pge,
            artistid: x,
            albums: vstring,

        };
        artists_albums_vec.push(artistalbums);
    }

    // log::info!("artist_albums_vec: {:#?}", artists_albums_vec);
    // println!("artist_albums_vec: {:#?}", artists_albums_vec);

    artists_albums_vec
}

pub fn write_albums_for_artist_to_db(artistsalbumssvec: Vec<ArtistAlbums>) -> Result<()> {
    for art in artistsalbumssvec {
        let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
        let conn = Connection::open(db_path).unwrap();

        conn.execute(
            "INSERT INTO albumsforartist (
                    page,
                    artistid,
                    albums
                )
                VALUES (?1, ?2, ?3)",
            (
                &art.page,
                &art.artistid,
                &art.albums,
            ),
        )?;
    }
    Ok(())
}
