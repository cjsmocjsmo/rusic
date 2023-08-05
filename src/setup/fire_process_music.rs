use crate::setup::fire_mp3_info;
use crate::setup::fire_utils::FireUtils;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicInfo {
    fireid: String,
    imgurl: String,
    artist: String,
    album: String,
    song: String,
    basedir: String,
    filenameresults: String,
    musicartistresults: String,
    musicalbumresults: String,
    durationresults: String,
    fullpath: String,
    extension: String,
    idx: String,
    page: String,
    fsizeresults: String,
}

fn write_music_nfos_to_file(mfo: MusicInfo, index: String) {
    let mus_info = serde_json::to_string(&mfo).unwrap();
    let fire_music_metadata_path = env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");
    let a = format!("{}/", fire_music_metadata_path.as_str());
    let b = format!("Music_Meta_{}.json", index.to_string());
    let outpath = a + &b;
    std::fs::write(outpath, mus_info).unwrap();
}

fn create_thumb_path(art: String, alb: String, ext: String) -> String {
    let myhttpd = env::var("FIRE_HTTP_ADDR").expect("$FIRE_HTTP_ADDR is not set");
    let myport = env::var("FIRE_HTTP_PORT").expect("$FIRE_HTTP_PORT is not set");
    let newpath = myhttpd + &myport + "/thumbnails/" + &art + "_-_" + &alb + &ext;

    newpath
}

pub fn process_mp3s(x: String, index: String, page: String) -> MusicInfo {
    let tags = fire_mp3_info::get_tag_info(&x);
    let artist = tags.0;
    let album = tags.1;
    let song = tags.2;
    let fu = FireUtils { apath: x.clone() };
    let id = FireUtils::get_md5(&fu);
    let duration_results = fire_mp3_info::get_duration(&x);
    let fullpath = &x.to_string();
    let base_dir = FireUtils::split_base_dir(&fu);
    let filename_results = FireUtils::split_filename(&fu);
    let art_alb = FireUtils::music_split_artist(&fu);
    let music_artist_results = art_alb.0;
    let music_album_results = art_alb.1;
    let ext = FireUtils::split_ext(&fu);
    let idx = index.to_string();
    let fsize_results = FireUtils::get_file_size(&fu).to_string();
    let music_info = MusicInfo {
        fireid: id,
        imgurl: create_thumb_path(
            music_artist_results.clone(),
            music_album_results.clone(),
            ext.clone(),
        ),
        artist: artist,
        album: album,
        song: song,
        basedir: base_dir,
        filenameresults: filename_results,
        musicartistresults: music_artist_results.clone(),
        musicalbumresults: music_album_results.clone(),
        durationresults: duration_results,
        fullpath: fullpath.to_string(),
        extension: format!("{:?}", ext.clone()),
        idx: idx,
        page: page.to_string(),
        fsizeresults: fsize_results,
    };
    write_music_nfos_to_file(music_info.clone(), index.clone());
    write_music_to_db(music_info.clone()).expect("Music db insertion failed");

    music_info.clone()
}

fn write_music_to_db(music_info: MusicInfo) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS music (
            id INTEGER PRIMARY KEY,
            fireid TEXT NOT NULL,
            imgurl TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            song TEXT NOT NULL,
            filenameresults TEXT NOT NULL,
            musicartistresults TEXT NOT NULL,
            musicalbumresults TEXT NOT NULL,
            durationresults TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            extension TEXT NOT NULL,
            idx TEXT NOT NULL,
            page TEXT NOT NULL,
            fsizeresults TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO music (
                fireid,
                imgurl, 
                artist, 
                album, 
                song, 
                filenameresults, 
                musicartistresults, 
                musicalbumresults, 
                durationresults, 
                fullpath, 
                extension, 
                idx, 
                page, 
                fsizeresults
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        (
            &music_info.fireid,
            &music_info.imgurl,
            &music_info.artist,
            &music_info.album,
            &music_info.song,
            &music_info.filenameresults,
            &music_info.musicartistresults,
            &music_info.musicalbumresults,
            &music_info.durationresults,
            &music_info.fullpath,
            &music_info.extension,
            &music_info.idx,
            &music_info.page,
            &music_info.fsizeresults,
        ),
    )?;

    Ok(())
}
