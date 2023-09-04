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
        };
        for m in media.1 {
            add_music_list.push(m);
        };
    }

    (add_media_img_list, add_music_list)
}

fn scan_usb1() -> (Vec<String>, Vec<String>) {
    let usb1 = env::var("RUSIC_USB1").expect("$RUSIC_USB1 is not set");
    let add_media = rusic_walk_dirs::walk_additional_dir(usb1);

    let add_media_img_list = add_media.0;
    let add_music_list = add_media.1;

    (add_music_list, add_media_img_list)
}

fn scan_usb2() -> (Vec<String>, Vec<String>) {
    let usb2 = env::var("RUSIC_USB2").expect("$RUSIC_USB2 is not set");
    let add_media = rusic_walk_dirs::walk_additional_dir(usb2);
    let add_media_img_list = add_media.0;
    let add_music_list = add_media.1;

    (add_music_list, add_media_img_list)
}

fn scan_usb3() -> (Vec<String>, Vec<String>) {
    let usb3 = env::var("RUSIC_USB3").expect("$RUSIC_USB3 is not set");
    let add_media = rusic_walk_dirs::walk_additional_dir(usb3);
    let add_media_img_list = add_media.0;
    let add_music_list = add_media.1;

    (add_music_list, add_media_img_list)
}

fn scan_usb4() -> (Vec<String>, Vec<String>) {
    let usb4 = env::var("RUSIC_USB4").expect("$RUSIC_USB4 is not set");
    let add_media = rusic_walk_dirs::walk_additional_dir(usb4);
    let add_media_img_list = add_media.0;
    let add_music_list = add_media.1;

    (add_music_list, add_media_img_list)
}

pub fn scan_all_sources() -> (Vec<String>, Vec<String>) {
    let mut master_music_list = Vec::new();
    let mut master_img_list = Vec::new();

    let usb1_var = env::var("RUSIC_USB1").expect("Not set");
    if usb1_var != "None".to_string() {
        let usb1 = scan_usb1();
        let mut usb1_music_list = usb1.0;
        let mut usb1_media_images = usb1.1;
        master_music_list.append(&mut usb1_music_list);
        master_img_list.append(&mut usb1_media_images);
    }

    let usb2_var = env::var("RUSIC_USB2").expect("not set");
    if usb2_var != "None".to_string() {
        let usb2 = scan_usb2();
        let mut usb2_music_list = usb2.0;
        let mut usb2_media_iamges = usb2.1;
        master_music_list.append(&mut usb2_music_list);
        master_img_list.append(&mut usb2_media_iamges);
    }

    let usb3_var = env::var("RUSIC_USB3").expect("not set");
    if usb3_var != "None".to_string() {
        let usb3 = scan_usb3();
        let mut usb3_music_list = usb3.0;
        let mut usb3_media_iamges = usb3.1;
        master_music_list.append(&mut usb3_music_list);
        master_img_list.append(&mut usb3_media_iamges);
    }

    let usb4_var = env::var("RUSIC_USB4").expect("not set");
    if usb4_var != "None".to_string() {
        let usb4 = scan_usb4();
        let mut usb4_music_list = usb4.0;
        let mut usb4_media_iamges = usb4.1;
        master_music_list.append(&mut usb4_music_list);
        master_img_list.append(&mut usb4_media_iamges);
    }

    (master_music_list, master_img_list)
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
    }
    println!("Found USB device: {:?}", usb_devices);

    usb_devices
}
