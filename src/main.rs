use env_logger::{Builder, Target};
// use std::env;
// use std::sync::mpsc::channel;
use std::time::Instant;
// use threadpool::ThreadPool;

pub mod envvars;
pub mod setup;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    Builder::new().target(Target::Stdout).init();

    log::info!("Rusic setup started");

    let _set_envvars = envvars::set_env_vars();

    let _setup = setup::setup();
    // let _create_tables = setup::rusic_tables::create_tables();
    // let media_lists = setup::rusic_walk_dirs::scan_all_sources();

    // let _rmt = run_music_threads(media_lists.0.clone());
    // let _rmit = run_music_img_threads(media_lists.1.clone());

    // let arids = setup::rusic_artist::unique_artistids();
    // let aalbs = setup::rusic_artist::albumids_for_artistid(arids.clone());
    // let _insert_aalbs = setup::rusic_artist::write_albums_for_artist_to_db(aalbs.clone()).unwrap();

    // let alids = setup::rusic_album::unique_albumids();
    // let sids = setup::rusic_album::songids_for_albumid(alids.clone());
    // let _insert_sids = setup::rusic_album::write_songs_for_album_to_db(sids.clone()).unwrap();

    // println!("music: {}\n", media_lists.0.clone().len());
    // println!("images: {}\n", media_lists.1.clone().len());

    let duration = start.elapsed();
    log::info!("Setup completed in: {} seconds", duration.as_secs());
    println!("Setup completed in: {} seconds", duration.as_secs());

    Ok(())
}

// fn run_music_threads(alist: Vec<String>) -> bool {
//     let pool = ThreadPool::new(num_cpus::get());
//     let (tx, rx) = channel();

//     let mut index = 0;
//     let mut page = 1;
//     let mut page_count = 0;

//     let ofs = env::var("RUSIC_PAGINATION").unwrap();
//     let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

//     for a in alist {
//         index = index + 1;
//         if page_count < offset {
//             page_count = page_count + 1;
//             page = page;
//         } else {
//             page_count = 1;
//             page = page + 1;
//         }
//         let tx = tx.clone();
//         pool.execute(move || {
//             let mfi = crate::setup::rusic_process_music::process_mp3s(
//                 a.clone(),
//                 index.to_string(),
//                 page.to_string(),
//             );
//             tx.send(mfi).expect("Could not send data");
//         });
//     }

//     drop(tx);
//     for t in rx.iter() {
//         let _ifo = t;
//         // println!("this is music_info\n {:?}\n\t", ifo);
//     }

//     true
// }

// fn run_music_img_threads(alist: Vec<String>) -> bool {
//     let pool = ThreadPool::new(num_cpus::get());
//     let (tx, rx) = channel();

//     let mut img_index = 0;
//     for i in alist {
//         img_index = img_index + 1;
//         if i.contains("Music") {
//             let tx = tx.clone();
//             pool.execute(move || {
//                 let img_info =
//                     setup::rusic_process_music_images::process_music_images(i.clone(), img_index);
//                 tx.send(img_info).expect("Could not send data");
//             });
//         }
//     }

//     drop(tx);
//     for t in rx.iter() {
//         // Insert this into db
//         let ifo = t;
//         println!("Processed Music img {:?} files", ifo);
//     }

//     true
// }

