// use env_logger::{Builder, Target};
// use std::time::Instant;
use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use crate::rusicdb;

pub mod rusic_album;
pub mod rusic_artist;
pub mod rusic_process_music;
pub mod rusic_process_music_images;
pub mod rusic_utils;
pub mod rusic_walk_dirs;

pub fn setup() -> String {
    let _create_tables = rusicdb::db_tables::create_tables();

    let usb_drives = rusic_walk_dirs::scan_for_usb_devices();

    let media_lists = rusic_walk_dirs::walk_usb_drives(usb_drives.clone());

    let mp3_count = media_lists.0.clone().len();
    let img_count = media_lists.1.clone().len();


//NEED ARTIST COUNT FOR ALPHA
//NEED ALBUM COUNT FOR ALPHA



    let _rmt = run_music_threads(media_lists.0.clone());

    let _gen_artist_count_by_alpha = rusic_utils::artist_album_count_by_alpha();


    let human_total_size = rusic_utils::mp3_total_size(media_lists.0.clone());

    let _rmit = run_music_img_threads(media_lists.1.clone());

    let arids = rusic_artist::unique_artistids();
    let aalbs = rusic_artist::albumids_for_artistid(arids.clone());
    let _insert_aalbs = rusic_artist::write_albums_for_artist_to_db(aalbs.clone()).unwrap();

    let alids = rusic_album::unique_albumids();
    let sids = rusic_album::songids_for_albumid(alids.clone());
    let insert_sids_result = rusicdb::db_main::write_songs_for_album_to_db(sids.clone());
    let insert_sids = match insert_sids_result {
        Ok(_) => String::from("Exit 0"),
        Err(_) => String::from("Exit 1"),
    };
    let _gen_db_check_file = rusic_utils::gen_db_check_file();
    println!("this is image count {:?}", img_count);
    println!("this is mp3 count {:?}", mp3_count);
    println!("\n\nFound {:?} USB devices", usb_drives.len());
    println!("Found {:?} usb devices", usb_drives);
    println!("Processed {} Mp3 files", media_lists.0.clone().len());
    println!("Processed {} Jpg files", media_lists.1.clone().len());
    println!("Mp3 size on disk {}", human_total_size);
    insert_sids
    // "fuck".to_string()
}

// fn run_first_letter_threads(alist: Vec<String>) -> bool {
//     let pool = ThreadPool::new(num_cpus::get());
//     let (tx, rx) = channel();


//     for i in alist {


//             let tx = tx.clone();
//             pool.execute(move || {
//                 let _fl_info =
//                     rusic_utils::gen_first_letter_db(i.clone());
//                 tx.send(i).expect("Could not send data");
//             });

//     }

//     drop(tx);
//     for t in rx.iter() {
//         // Insert this into db
//         let ifo = t;
//         println!("Processed first letter {:?} files", ifo);
//     }

//     true
// }

fn run_music_threads(alist: Vec<String>) -> bool {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;

    let ofs = env::var("RUSIC_PAGINATION").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    for a in alist {
        index = index + 1;
        if page_count < offset {
            page_count = page_count + 1;
            page = page;
        } else {
            page_count = 1;
            page = page + 1;
        }
        let tx = tx.clone();
        pool.execute(move || {
            let mfi = crate::setup::rusic_process_music::process_mp3s(
                a.clone(),
                index.to_string(),
                page.to_string(),
            );
            tx.send(mfi).expect("Could not send data");
        });
    }

    drop(tx);
    for t in rx.iter() {
        let _ifo = t;
        // println!("this is music_info\n {:?}\n\t", ifo);
    }

    true
}

fn run_music_img_threads(alist: Vec<String>) -> bool {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut img_index = 0;
    for i in alist {
        img_index = img_index + 1;
        if i.contains("Music") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info =
                    rusic_process_music_images::process_music_images(i.clone(), img_index);
                tx.send(img_info).expect("Could not send data");
            });
        }
    }

    drop(tx);
    for t in rx.iter() {
        // Insert this into db
        let ifo = t;
        println!("Processed Music img {:?} files", ifo);
    }

    true
}

// pub fn save_coverart(x: String, coverart_path: String) -> Result<(), E> {
//         let tag = Tag::read_from_path(x.clone()).expect(&x);
//         let mut first_picture = tag.pictures().next();
//         if let Some(p) = first_picture {
//             match image::load_from_memory(&p.data) {
//                 Ok(image) => {
//                     image.save(&coverart_path).map_err(|e| {
//                         anyhow!("Couldn't write image file {:?}: {}", coverart_path, e)
//                     })?;
//                 }
//                 Err(e) => return Err(anyhow!("Couldn't load image: {}", e)),
//             };

//             Ok(())
//         } else {
//             Err(anyhow!("No image found in music file"))
//         }
//     }

// pub fn media_total_size(addr: String) -> String {
//     let total_size = WalkDir::new(addr)
//         .min_depth(1)
//         .max_depth(5)
//         .into_iter()
//         .filter_map(|entry| entry.ok())
//         .filter_map(|entry| entry.metadata().ok())
//         .filter(|metadata| metadata.is_file())
//         .fold(0, |acc, m| acc + m.len());

//     let btos = total_size.to_string();
//     let result = Byte::from_str(btos).unwrap();
//     let size = result.get_appropriate_unit(true).to_string();

//     size
// }
