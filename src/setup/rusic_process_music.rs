use crate::setup::rusic_utils;
use crate::setup::rusic_utils::RusicUtils;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicInfo {
    pub rusicid: String,
    pub imgurl: String,
    pub artist: String,
    pub artistid: String,
    pub album: String,
    pub albumid: String,
    pub song: String,
    pub fullpath: String,
    pub extension: String,
    pub idx: String,
    pub page: String,
    pub fsizeresults: String,
}

pub fn process_mp3s(x: String, index: String, page: String) -> MusicInfo {
    // println!("processing:\n\t {:#?}", x);
    let fu = RusicUtils { apath: x.clone() };
    let rusic_id = rusic_utils::get_md5(x.clone());
    let art_alb = RusicUtils::split_artist_album(&fu);
    let tag = RusicUtils::get_tag_info(&fu);
    let tag_artist = tag.0.clone();
    let tag_album = tag.1.clone();
    let artist_id = rusic_utils::get_md5(tag.0.clone());
    let album_id = rusic_utils::get_md5(tag.1.clone());

    // let dirz = RusicUtils::split_base_dir_filename(&fu);

    let music_info = MusicInfo {
        rusicid: rusic_id.clone(),
        imgurl: create_thumb_path(art_alb.0.clone(), art_alb.1.clone()),
        artist: tag_artist.clone(),
        artistid: artist_id.clone(),
        album: tag_album.clone(),
        albumid: album_id.clone(),
        song: tag.2,
        fullpath: x.clone(),
        extension: RusicUtils::split_ext(&fu),
        idx: index.clone(),
        page: page.clone(),
        fsizeresults: RusicUtils::get_file_size(&fu).to_string(),
    };
    let _wm = write_music_to_db(music_info.clone());
    let _wnfo = write_music_nfos_to_file(music_info.clone(), index.clone());

    // let artist_starts_with = RusicUtils::artist_starts_with(&fu);
    // let album_starts_with = RusicUtils::album_starts_with(&fu);

    let _insert_first_letter = rusic_utils::gen_first_letter_db(x.clone()).unwrap();
    let _insert_art_artid = write_art_artid_to_db(rusic_id.clone(), tag_artist.clone(), artist_id.clone()).unwrap();
    let _insert_alb_albid = write_alb_albid_to_db(rusic_id.clone(), tag_album.clone(), album_id.clone()).unwrap();

    music_info.clone()
}

fn write_art_artid_to_db(rusid: String, art: String, artid: String) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "INSERT INTO artists (
                rusicid,
                artist,
                artistid
            )
            VALUES (?1, ?2, ?3)",
        (
            &rusid,
            &art,
            &artid,
        ),
    )?;

    Ok(())
}


fn write_alb_albid_to_db(rusid: String, alb: String, albid: String) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "INSERT INTO albums (
                rusicid,
                album,
                albumid
            )
            VALUES (?1, ?2)",
        (
            &rusid,
            &alb,
            &albid,
        ),
    )?;

    Ok(())
}

fn write_music_nfos_to_file(mfo: MusicInfo, index: String) {
    let mus_info = serde_json::to_string(&mfo).unwrap();
    let rusic_music_metadata_path = env::var("RUSIC_NFOS").expect("$RUSIC_NFOS is not set");
    let a = format!("{}/", rusic_music_metadata_path.as_str());
    let b = format!("Music_Meta_{}.json", index.to_string());
    let outpath = a + &b;
    std::fs::write(outpath, mus_info).unwrap();
}

fn create_thumb_path(art: String, alb: String) -> String {
    let myhttpd = env::var("RUSIC_HTTP_ADDR").expect("$RUSIC_HTTP_ADDR is not set");
    let myport = env::var("RUSIC_PORT").expect("$RUSIC_PORT is not set");
    let newpath = myhttpd + &myport + "/thumbnails/" + &art + "_-_" + &alb + ".jpg";

    newpath
}

fn write_music_to_db(music_info: MusicInfo) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();
    println!("writing to db: {:#?}", music_info);

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
