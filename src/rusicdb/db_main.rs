use crate::types;
use rusqlite::{Connection, Result};
use std::env;
// use serde::{Deserialize, Serialize};

pub fn write_music_to_db(music_info: types::MusicInfo) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();
    println!("writing to db: {:#?}", music_info.rusicid.clone());

    conn.execute(
        "INSERT INTO music (
                rusicid,
                imgurl,
                artist,
                artistid,
                album,
                albumid,
                song,
                fullpath,
                extension,
                idx,
                page,
                fsizeresults
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        (
            &music_info.rusicid,
            &music_info.imgurl,
            &music_info.artist,
            &music_info.artistid,
            &music_info.album,
            &music_info.albumid,
            &music_info.song,
            &music_info.fullpath,
            &music_info.extension,
            &music_info.idx,
            &music_info.page,
            &music_info.fsizeresults,
        ),
    )?;

    Ok(())
}

pub fn write_songs_for_album_to_db(albumsongsvec: Vec<types::AlbumSongs>) -> Result<()> {
    for alb in albumsongsvec {
        let conn = Connection::open("./db/rusic.db").unwrap();

        conn.execute(
            "INSERT INTO songsforalbum (
                    page,
                    albumid,
                    songs
                )
                VALUES (?1, ?2, ?3)",
            (&alb.page, &alb.albumid, &alb.rusicids),
        )?;
    }
    Ok(())
}

pub fn insert_first_letter(first_letter_info: types::FirstLetterInfo) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "INSERT INTO startswith (
                rusicid,
                artist,
                album,
                artistid,
                albumid,
                artist_first_letter,
                album_first_letter
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &first_letter_info.rusicid,
            &first_letter_info.artist,
            &first_letter_info.album,
            &first_letter_info.artistid,
            &first_letter_info.albumid,
            &first_letter_info.artist_first_letter,
            &first_letter_info.album_first_letter,
        ),
    )?;

    Ok(())
}

pub fn write_music_img_to_db(music_img_info: types::MusicImageInfo) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "INSERT INTO music_images (
                rusicid,
                width,
                height,
                artist,
                artistid,
                album,
                albumid,
                filesize,
                fullpath,
                thumbpath,
                idx
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            &music_img_info.rusicid,
            &music_img_info.width,
            &music_img_info.height,
            &music_img_info.artist,
            &music_img_info.artistid,
            &music_img_info.album,
            &music_img_info.albumid,
            &music_img_info.filesize,
            &music_img_info.fullpath,
            &music_img_info.thumbpath,
            &music_img_info.idx,
        ),
    )?;

    Ok(())
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct ArtistCount {
//     artist_first_letter: String,
//     count: i64,
// }

pub fn post_artist_count_by_alpha(alpha: String) -> (String, String) {
    let mut distinct_artistid_list_for_alpha = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();


    let mut stmt = conn
        .prepare("SELECT DISTINCT artistid FROM startswith WHERE artist_first_letter = ?")
        .unwrap();
    let mut rows = stmt.query(&[&alpha]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let artistid: String = row.get(0).unwrap();
        distinct_artistid_list_for_alpha.push(artistid);
    }

    let count = distinct_artistid_list_for_alpha.len().to_string();

    let alphacount = (alpha.clone(), count.clone());

    println!("this is artist alpha count {:#?}", alphacount.clone());

    let foo = types::ArtistCount {
        alpha: alpha.clone(),
        count: count.clone().parse::<i64>().unwrap(),
    };

    conn.execute(
        "INSERT INTO artistcount (
                alpha,
                count
            )
            VALUES (?1, ?2)",
        (
            &foo.alpha,
            &foo.count,
        ),
    ).unwrap();



    //PUT ALPHA COUNT INTO DB

    alphacount
}

pub fn post_album_count_by_alpha(alpha: String) -> (String, String) {
    let mut distinct_albumid_list_for_alpha = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();


    let mut stmt = conn
        .prepare("SELECT DISTINCT albumid FROM startswith WHERE album_first_letter = ?")
        .unwrap();
    let mut rows = stmt.query(&[&alpha]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let albumid: String = row.get(0).unwrap();
        distinct_albumid_list_for_alpha.push(albumid);
    }

    let count = distinct_albumid_list_for_alpha.len().to_string();

    let alphacount = (alpha.clone(), count.clone());

    println!("this is album alpha count {:#?}", alphacount.clone());
    let fu = types::AlbumCount {
        alpha: alpha.clone(),
        count: count.clone().parse::<i64>().unwrap(),
    };

    conn.execute(
        "INSERT INTO albumcount (
                alpha,
                count
            )
            VALUES (?1, ?2)",
        (
            &fu.alpha,
            &fu.count,
        ),
    ).unwrap();

    //PUT ALPHA COUNT INTO DB

    alphacount
}