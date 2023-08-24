use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json;

pub fn unique_artistids() -> Vec<String> {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
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
    pub artistid: String,
    pub albums: String,
    // pub index: String,
    // pub page: String
}

pub fn albumids_for_artistid(xlist: Vec<String>) -> Vec<ArtistAlbums> {
    let mut page = 1;
    let mut index = 1;
    let mut artists_albums_vec = Vec::new();
    for x in xlist {
        log::info!("x: {:#?}", x);
        println!("x: {:#?}", x);
        index += 1;
        if index == 26 {
            page += 1;
            index = 1;
        }
        let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
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
            artistid: x,
            albums: vstring,
            // index: String::from(index.to_string()),
            // page: String::from(page.to_string()),
        };
        artists_albums_vec.push(artistalbums);
    }

    log::info!("artist_albums_vec: {:#?}", artists_albums_vec);
    println!("artist_albums_vec: {:#?}", artists_albums_vec);

    artists_albums_vec
}

pub fn write_albums_for_artist_to_db(artistsalbumssvec: Vec<ArtistAlbums>) -> Result<()> {
    for art in artistsalbumssvec {
        log::info!("art: {:#?}", art);
        println!("art: {:#?}", art);
        let conn = Connection::open("./db/rusic.db").unwrap();

        conn.execute(
            "INSERT INTO albums_for_artist (
                    artistid,
                    albums

                )
                VALUES (?1, ?2)",
            (
                &art.artistid,
                &art.albums),
        )?;
    }
    Ok(())
}
