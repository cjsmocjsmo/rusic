// use rusqlite::{Connection, Result};
use std::env;
use std::sync::mpsc::channel;
use std::time::Instant;
use threadpool::ThreadPool;

pub mod envvars;
pub mod setup;


fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let _set_envvars = crate::envvars::set_env_vars();
    let _tables = crate::setup::rusic_tables::create_tables();
    let media_lists = setup::rusic_walk_dirs::scan_all_sources();
    // println!("media_lists: {:#?}", media_lists.0);

    let _rmt = run_music_threads(media_lists.0.clone());
    let _aids = setup::rusic_artist::unique_artistids();
    let _alids = setup::rusic_album::unique_albumids();
    // let _rmit = run_music_img_threads(media_lists.1.clone());




    // get artist pages together

    // get album pagets together

    println!("music: {}\n", media_lists.0.clone().len());
    println!("images: {}\n", media_lists.1.clone().len());

    // THIS RUNS EXTREMELY SLOW EVEN WITH THREADS
    // let _rdt = run_duration_threads(media_lists.0.clone());

    let duration = start.elapsed();
    println!("Setup completed in: {} seconds", duration.as_secs());

    Ok(())
}

// #[derive(Debug)]
// struct DurationInfo {
//     rusicid: String,
//     duration: String,
//     path: String,
// }

// fn run_duration_threads(alist: Vec<String>) -> bool {
//     let pool = ThreadPool::new(num_cpus::get());
//     let (tx, rx) = channel();

//     for a in alist {
//         let tx = tx.clone();
//         pool.execute(move || {
//             let fu = setup::rusic_utils::RusicUtils { apath: a.clone() };
//             let dur = setup::rusic_utils::RusicUtils::get_duration(&fu);
//             tx.send(dur).expect("Could not send data");
//         });
//     }

//     drop(tx);
//     for t in rx.iter() {
//         // Insert this into db
//         let ifo = t;
//         println!("rusicid\n {:?}\n\t", ifo.0);
//         println!("duration {:?}\n\t", ifo.1);
//         println!("path\n {:?}\n\t", ifo.2);
//         let dinfo = DurationInfo {
//             rusicid: ifo.0,
//             duration: ifo.1,
//             path: ifo.2,
//         };
//         let _wdt = write_duration_to_db(dinfo).unwrap();
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
        // Insert this into db
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
                    setup::rusic_process_music_images::process_music_images(i.clone(), img_index);
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

// fn write_duration_to_db(duration_info: DurationInfo) -> Result<()> {
//     let conn = Connection::open("./db/rusic.db").unwrap();

//     conn.execute(
//         "INSERT INTO duration (
//                 rusicid,
//                 duration,
//                 path
//             )
//             VALUES (?1, ?2, ?3)",
//         (
//             &duration_info.rusicid,
//             &duration_info.duration,
//             &duration_info.path,
//         ),
//     )?;

//     Ok(())
// }
