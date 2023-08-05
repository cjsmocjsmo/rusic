use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use std::time::Instant;

pub mod rusic_image;
pub mod rusic_misc;
pub mod rusic_mp3_info;

pub mod rusic_process_music;
pub mod rusic_process_music_images;
pub mod rusic_utils;
pub mod rusic_walk_dirs;



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
        // Insert this into db
        let ifo = t;
        println!("this is music_info\n {:?}\n\t", ifo.clone());
    };

    true
}

// fn run_video_img_threads(alist: Vec<String>) {
//     let pool = ThreadPool::new(num_cpus::get());
//     let (tx, rx) = channel();

//     let mut img_index = 0;
//     for i in alist {
//         img_index = img_index + 1;
//         if i.contains("MovPosters") {
//             let tx = tx.clone();
//             pool.execute(move || {
//                 let img_info =
//                     RUSIC_process_movie_images::process_movie_posters(i.clone(), img_index);
//                 tx.send(img_info).expect("Could not send data");
//             });
//         }
//     }

//     drop(tx);
//     for t in rx.iter() {
//         // Insert this into db
//         let ifo = t;
//         println!("Processed Movie img {:#?} file", ifo);
//     }
// }

fn run_music_img_threads(alist: Vec<String>) {
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
}

pub fn run_setup() -> bool {
    let start = Instant::now();
        let media_lists = rusic_walk_dirs::scan_all_sources();

        run_music_threads(media_lists.0.clone());
        run_music_img_threads(media_lists.1.clone());


        let ab_list = crate::setup::rusic_misc::create_art_alb_list(media_lists.0.clone());
        let artist_list = crate::setup::rusic_misc::create_artistids(ab_list.0);
        let album_list = crate::setup::rusic_misc::create_albumids(ab_list.1);

        let art_serial = serde_json::to_string(&artist_list).unwrap();
        let alb_serial = serde_json::to_string(&album_list);

        println!("artistid_list; {:#?}\n", art_serial);
        println!("albumid_list; {:#?}\n", alb_serial);

        println!("music: {}\n", media_lists.0.clone().len());
        println!("images: {}\n", media_lists.1.clone().len());




    let duration = start.elapsed();
    println!("Setup completed in: {} seconds", duration.as_secs());


    true
}
