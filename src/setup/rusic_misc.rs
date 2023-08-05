use std::env;
use byte_unit::Byte;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use rusqlite::{Connection, Result};
use std::clone::Clone;

pub fn media_total_size(addr: String) -> String {
    let total_size = WalkDir::new(addr)
        .min_depth(1)
        .max_depth(5)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    let btos = total_size.to_string();
    let result = Byte::from_str(btos).unwrap();
    let size = result.get_appropriate_unit(true).to_string();

    size
}

pub fn create_art_alb_list(alist: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut art_vec = Vec::new();
    let mut alb_vec = Vec::new();

    for a in alist {
        let tags = crate::setup::rusic_mp3_info::get_tag_info(&a);
        let artist = tags.0;
        let album = tags.1;
        art_vec.push(artist);
        alb_vec.push(album)
    };
    art_vec.sort();
    alb_vec.sort();

    art_vec.dedup();
    alb_vec.dedup();

    (art_vec, alb_vec)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtId {
    id: String,
    artist: String,
    artistid: String,
}

pub fn create_artistids(alist: Vec<String>) -> Vec<ArtId> {

    let mut artid_list = Vec::new();
    let mut count = 0;
    for a in alist {
        count = count + 1;

        let af = crate::setup::rusic_utils::RusicUtils {
            apath: a.to_string()
        };

        let artistid = crate::setup::rusic_utils::RusicUtils::get_md5(&af);
        let artidstruc = ArtId {
            id: count.clone().to_string(),
            artist: a.clone(),
            artistid: artistid.clone()
        };

        write_artist_ids_to_db(artidstruc.clone()).expect("artistids insert has failed");
        artid_list.push(artidstruc.clone());
    };

    let artidlistserial = serde_json::to_string(&artid_list).unwrap();
    println!("{:#?}", artidlistserial);

    let fire_nfo_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");
    let a = format!("{}/", fire_nfo_path.as_str());
    let b = format!("Artist_ID_List.json");
    let outpath = a + &b;
    std::fs::write(outpath, artidlistserial).unwrap();

    artid_list
}

fn write_artist_ids_to_db(artidstruc: ArtId) -> Result<()> {
    let conn = Connection::open("rusic.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS artistids (
            id INTEGER PRIMARY KEY,
            artist TEXT NOT NULL,
            artistid TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO artistids (
                artist,
                artistid
            )
            VALUES (?1, ?2)",
        (
            &artidstruc.artist,
            &artidstruc.artistid
        ),
    )?;

    Ok(())
}

fn write_album_ids_to_db(albidstruc: AlbId) -> Result<()> {
    let conn = Connection::open("rusic.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albumids (
            id INTEGER PRIMARY KEY,
            album TEXT NOT NULL,
            albumid TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO albumids (
                album,
                albumid
            )
            VALUES (?1, ?2)",
        (
            &albidstruc.album,
            &albidstruc.albumid
        ),
    )?;

    Ok(())
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbId {
    id: String,
    album: String,
    albumid: String
}

pub fn create_albumids(alist: Vec<String>) -> Vec<AlbId> {
    let mut albid_list = Vec::new();
    let mut count = 0;
    for a in alist {
        count = count + 1;

        let af = crate::setup::rusic_utils::RusicUtils {
            apath: a.to_string()
        };

        let albumid = crate::setup::rusic_utils::RusicUtils::get_md5(&af);
        let albidstruc = AlbId {
            id: count.clone().to_string(),
            album: a.clone(),
            albumid: albumid.clone()
        };
        write_album_ids_to_db(albidstruc.clone()).expect("albumid db insertion has failed");
        albid_list.push(albidstruc);

    };
    let albidlistserial = serde_json::to_string(&albid_list).unwrap();
    println!("{:#?}", albidlistserial);

    let fire_nfo_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");

    let a = format!("{}/", fire_nfo_path.as_str());
    let b = format!("Album_ID_List.json");
    let outpath = a + &b;
    std::fs::write(outpath, albidlistserial).unwrap();

    albid_list
}

