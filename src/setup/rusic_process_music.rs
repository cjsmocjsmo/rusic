use crate::setup::rusic_utils::RusicUtils;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;
use crate::setup::rusic_utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicInfo {
    pub rusicid: String,
    pub imgurl: String,
    pub artist: String,
    pub artistid: String,
    pub album: String,
    pub albumid: String,
    pub song: String,
    pub basedir: String,
    pub filenameresults: String,
    pub musicartistresults: String,
    pub musicalbumresults: String,
    pub durationresults: String,
    pub fullpath: String,
    pub extension: String,
    pub idx: String,
    pub page: String,
    pub fsizeresults: String,
}

pub fn process_mp3s(x: String, index: String, page: String) -> MusicInfo {
    println!("processing:\n\t {:#?}", x);
    let fu = RusicUtils { apath: x.clone() };

    let tag = RusicUtils::get_tag_info(&fu);

    let art_alb = RusicUtils::split_artist_album(&fu);

    let dirz = RusicUtils::split_base_dir_filename(&fu);


    let music_info = MusicInfo {
        rusicid: rusic_utils::get_md5(x.clone()),
        imgurl: create_thumb_path(
            art_alb.0.clone(),
            art_alb.1.clone(),
        ),
        artist: tag.0.clone(),
        artistid: rusic_utils::get_md5(tag.0.clone()),
        album: tag.1.clone(),
        albumid: rusic_utils::get_md5(tag.1.clone()),
        song: tag.2,
        basedir: dirz.0,
        filenameresults: dirz.1,
        musicartistresults: art_alb.0.clone(),
        musicalbumresults: art_alb.1.clone(),
        durationresults: "0".to_string(),
        fullpath: x.clone(),
        extension: RusicUtils::split_ext(&fu),
        idx: index.clone(),
        page: page.clone(),
        fsizeresults: RusicUtils::get_file_size(&fu).to_string(),
    };
    println!("music_info: {:#?}", music_info);
    let _wm = write_music_to_db(music_info.clone());
    let _wnfo = write_music_nfos_to_file(music_info.clone(), index.clone());

    music_info.clone()
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

    conn.execute(
        "INSERT INTO music (
                rusicid,
                imgurl,
                artist,
                artistid,
                album,
                albumid,
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
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        (
            &music_info.rusicid,
            &music_info.imgurl,
            &music_info.artist,
            &music_info.artistid,
            &music_info.album,
            &music_info.albumid,
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

