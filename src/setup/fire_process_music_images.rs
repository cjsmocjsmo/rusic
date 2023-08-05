use std::env;
use std::clone::Clone;
use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result};

fn create_music_thumbnail(x: &String, art: String, alb: String) -> String {
    let fire_music_metadata_path = env::var("FIRE_THUMBNAILS").expect("$FIRE_THUMBNAILS is not set");
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let out_fname = fire_music_metadata_path + &new_fname;
    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");
    out_fname.to_string()
}

fn write_music_img_to_file(miinfo: MusicImageInfo, index: i32) {
    let mii = serde_json::to_string(&miinfo).unwrap();
    let fire_music_metadata_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");
    let a = format!("{}/", fire_music_metadata_path.as_str());
    let b = format!("Music_Image_Meta_{}.json", &index);
    let outpath = a + &b;
    std::fs::write(outpath, mii.clone()).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicImageInfo {
    fireid: String,
    width: String,
    height: String,
    basedir: String,
    filename: String,
    extension: String,
    artist: String,
    album: String,
    filesize: String,
    fullpath: String,
    thumbpath: String,
    idx: String,
}

use crate::setup::fire_utils::FireUtils;
use crate::setup::fire_image;


pub fn process_music_images(x: String, index: i32) -> i32 {
    let foo2 = FireUtils {apath: x.clone()};
    let id = FireUtils::get_md5(&foo2);
    let dims = FireUtils::get_dims(&foo2);

    if dims != (0, 0) {
        let newdims = fire_image::normalize_music_image(dims);
        let width_r = newdims.0.to_string();
        let height_r = newdims.1.to_string();
        let base_dir = FireUtils::split_base_dir(&foo2);
        let file_name = FireUtils::split_filename(&foo2);
        let ext = FireUtils::split_ext(&foo2);
        let artist_results = FireUtils::image_split_artist(&foo2);
        let album_results = FireUtils::image_split_album(&foo2);
        let fsize_results = FireUtils::get_file_size(&foo2).to_string();
        let full_path = &x.to_string();
        let thumb_path = create_music_thumbnail(&x, artist_results.clone(), album_results.clone());

        let music_img_info = MusicImageInfo {
            fireid: id,
            width: width_r,
            height: height_r,
            basedir: base_dir,
            filename: file_name,
            extension: ext,
            artist: artist_results,
            album: album_results,
            filesize: fsize_results,
            fullpath: full_path.to_string(),
            thumbpath: thumb_path,
            idx: index.to_string(),
        };
        write_music_img_to_file(music_img_info.clone(), index);
        write_music_img_to_db(music_img_info.clone()).expect("music image db insertion failed")
    };

    index
}

fn write_music_img_to_db(music_img_info: MusicImageInfo) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS music_images (
            id INTEGER PRIMARY KEY,
            fireid TEXT NOT NULL,
            width TEXT NOT NULL,
            height TEXT NOT NULL,
            basedir TEXT NOT NULL,
            filename TEXT NOT NULL,
            extension TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            filesize TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO music_images (
                fireid,
                width, 
                height,
                basedir,
                filename,
                extension,
                artist,
                album,
                filesize,
                fullpath,
                thumbpath,
                idx
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        (
            &music_img_info.fireid,
            &music_img_info.width,
            &music_img_info.height,
            &music_img_info.basedir,
            &music_img_info.filename,
            &music_img_info.extension,
            &music_img_info.artist,
            &music_img_info.album,
            &music_img_info.filesize,
            &music_img_info.fullpath,
            &music_img_info.thumbpath,
            &music_img_info.idx
        ),
    )?;


    Ok(())
}


