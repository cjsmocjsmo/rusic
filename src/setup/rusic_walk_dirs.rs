use crate::setup::rusic_walk_dirs;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn walk_additional_dir(apath: String) -> (Vec<String>, Vec<String>) {
    let mut musicvec = Vec::new();
    let mut musicimgvec = Vec::new();
    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;
    let ofs = env::var("RUSIC_PAGINATION").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            index = index + 1;
            if page_count < offset {
                page_count = page_count + 1;
                page = page;
            } else {
                page_count = 1;
                page = page + 1;
            }
            let fname = e.path().to_string_lossy().to_string();

            if fname.contains("Music") {
                if fname.ends_with(".mp3") {
                    musicvec.push(fname.clone());
                } else if fname.ends_with(".jpg") {
                    musicimgvec.push(fname.clone());
                } else if fname.ends_with(".png") {
                    musicimgvec.push(fname.clone());
                } else if fname.ends_with(".webp") {
                    musicimgvec.push(fname.clone());
                } else if fname.ends_with(".jpeg") {
                    musicimgvec.push(fname.clone());
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    (musicimgvec.clone(), musicvec.clone())
}

pub fn walk_usb_drives(usb_list: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut add_music_list = Vec::new();
    let mut add_media_img_list = Vec::new();
    for usb in usb_list {
        let media = rusic_walk_dirs::walk_additional_dir(usb);
        for m in media.0 {
            add_media_img_list.push(m);
        }
        for z in media.1 {
            add_music_list.push(z);
        }
    }

    (add_music_list, add_media_img_list)
}

pub fn scan_for_usb_devices() -> Vec<String> {
    let mut usb_devices = Vec::new();
    let path = env::var("RUSIC_USB").expect("$RUSIC_USB is not set");
    let usb_dir_path = Path::new(&path);
    for entry in fs::read_dir(usb_dir_path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let dir_name = entry.path();
            let dir_name = dir_name.to_str().unwrap();
            let dname = dir_name.to_string();
            usb_devices.push(dname);
        }
    };

    usb_devices
}
