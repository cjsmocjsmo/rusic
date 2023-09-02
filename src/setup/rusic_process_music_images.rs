use crate::setup::rusic_utils;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicImageInfo {
    rusicid: String,
    width: String,
    height: String,
    basedir: String,
    filename: String,
    extension: String,
    artist: String,
    artistid: String,
    album: String,
    albumid: String,
    filesize: String,
    fullpath: String,
    thumbpath: String,
    idx: String,
}

use crate::setup::rusic_utils::RusicUtils;

pub fn process_music_images(x: String, index: i32) -> i32 {
    if x.ends_with("webp") {
        println!(".webp found please convert to jpg: {:?}", x);
    } else if x.ends_with("jpeg") {
        println!(".jpeg found please convert to jpg: {:?}", x);
    } else if x.ends_with("png") {
        println!(".png found please convert to jpg: {:?}", x);
    } else if x.ends_with("gif") {
        println!(".gif found please convert to jpg: {:?}", x);
    } else {
        let foo2 = RusicUtils { apath: x.clone() };
        let id = rusic_utils::get_md5(x.clone());
        let dims = RusicUtils::get_dims(&foo2);
        let bdfn = RusicUtils::split_base_dir_filename(&foo2);
        let basedir = bdfn.0;
        let filename = bdfn.1;
        let artalb = RusicUtils::split_artist_album(&foo2);
        let artist1 = artalb.0;
        let album1 = artalb.1;

        if dims != (0, 0) {
            let newdims = crate::setup::rusic_utils::normalize_music_image(dims);
            let width_r = newdims.0.to_string();
            let height_r = newdims.1.to_string();
            let base_dir = basedir;
            let file_name = filename;
            let ext = RusicUtils::split_ext(&foo2);
            let fsize_results = RusicUtils::get_file_size(&foo2).to_string();
            let full_path = &x.to_string();
            let thumb_path = create_music_thumbnail(&x, artist1.clone(), album1.clone());

            let music_img_info = MusicImageInfo {
                rusicid: id,
                width: width_r,
                height: height_r,
                basedir: base_dir,
                filename: file_name,
                extension: ext,
                artist: artist1.clone(),
                artistid: rusic_utils::get_md5(artist1.clone()),
                album: album1.clone(),
                albumid: rusic_utils::get_md5(album1.clone()),
                filesize: fsize_results,
                fullpath: full_path.to_string(),
                thumbpath: thumb_path,
                idx: index.to_string(),
            };
            write_music_img_to_file(music_img_info.clone(), index);
            write_music_img_to_db(music_img_info.clone()).expect("music image db insertion failed")
        };
    }

    index
}

fn create_music_thumbnail(x: &String, art: String, alb: String) -> String {
    let rusic_music_metadata_path = env::var("RUSIC_THUMBS").expect("$RUSIC_THUMBS is not set");
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    println!("new_fname: {:?}", new_fname);
    let out_fname = rusic_music_metadata_path + &new_fname;
    println!("out_fname: {:?}", out_fname);
    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");
    out_fname.to_string()
}

fn write_music_img_to_file(miinfo: MusicImageInfo, index: i32) {
    let mii = serde_json::to_string(&miinfo).unwrap();
    let rusic_music_metadata_path = env::var("RUSIC_NFOS").expect("$RUSIC_NFOS is not set");
    let outpath = format!(
        "{}/Music_Image_Meta_{}.json",
        rusic_music_metadata_path.as_str(),
        &index
    );
    std::fs::write(outpath, mii.clone()).unwrap();
}

fn write_music_img_to_db(music_img_info: MusicImageInfo) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS music_images (
            id INTEGER PRIMARY KEY,
            rusicid TEXT NOT NULL,
            width TEXT NOT NULL,
            height TEXT NOT NULL,
            basedir TEXT NOT NULL,
            filename TEXT NOT NULL,
            extension TEXT NOT NULL,
            artist TEXT NOT NULL,
            artistid TEXT NOT NULL,
            album TEXT NOT NULL,
            albumid TEXT NOT NULL,
            filesize TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO music_images (
                rusicid,
                width,
                height,
                basedir,
                filename,
                extension,
                artist,
                artistid,
                album,
                albumid,
                filesize,
                fullpath,
                thumbpath,
                idx
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        (
            &music_img_info.rusicid,
            &music_img_info.width,
            &music_img_info.height,
            &music_img_info.basedir,
            &music_img_info.filename,
            &music_img_info.extension,
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
