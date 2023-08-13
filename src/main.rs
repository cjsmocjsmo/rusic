
use std::env;
use std::sync::mpsc::channel;
use std::time::Instant;
use threadpool::ThreadPool;

pub mod setup;
pub mod envvars;


// fn run_music_threads(alist: Vec<String>) -> bool {
//     let mut index = 0;
//     let mut page = 1;
//     let mut page_count = 0;

//     let ofs = env::var("RUSIC_PAGINATION").unwrap();
//     let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

//     let mut tag_info_vec = Vec::new();
//     for a in alist {
//         index = index + 1;
//         if page_count < offset {
//             page_count = page_count + 1;
//             page = page;
//         } else {
//             page_count = 1;
//             page = page + 1;
//         }

//         let fu = RusicUtils { apath: a.clone() };
//         let rusic_id = RusicUtils::get_md5(&fu);
//         let tags = RusicUtils::get_tag_info(&fu);
//         let artist = tags.0;
//         let album = tags.1;
//         let song = tags.2;
//         // let mut c_art: String = "fuck".to_string();
//         // if let Ok(cover_art) = RusicUtils::extract_coverart(&fu) {
//         //     println!("cover_art: {:#?}", cover_art);
//         //     c_art = cover_art.to_string();
//         // } else {
//         //     println!("No cover art found");
//         //     c_art = "No cover art found".to_string();
//         // };


//         let tinfo = crate::setup::rusic_process_music::TagInfo {
//             id: index.to_string(),
//             rusicid: rusic_id,
//             filename: a.to_string(),
//             artist: artist,
//             album: album,
//             song: song,

//         };
//         println!("tinfo: {:#?}", tinfo);
//         tag_info_vec.push(tinfo);
//     }
//     println!("taginfovec: {:#?}", tag_info_vec);
//     // let _ins_tag_info = insert_tag_info(tag_info_vec);


//     // for tag in tag_info_vec {
//     //     let handle = thread::spawn(move || {
//     //     for i in 1..10 {
//     //         println!("thread number {}", i);
//     //         let _ins_tag_info = insert_tag_info(tag.clone());
//     //     }
//     // });
//     // handle.join().unwrap();
//     // }

//     // let mfi = crate::setup::rusic_process_music::insert_tag_info(
//     //     a.clone(),
//     //     index.to_string(),
//     //     page.to_string(),
//     // );
//     // println!(
//     //     "this is music_info\n {:?}\n\t",
//     //     mfi.expect("Insert failed").clone()
//     // );


//     true
// }


fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let _set_envvars = crate::envvars::set_env_vars();
    let _tables = crate::setup::rusic_tables::create_tables();
    let media_lists = setup::rusic_walk_dirs::scan_all_sources();
    println!("media_lists: {:#?}", media_lists);

    // let _rmt = run_music_threads(media_lists.0.clone());
    let _rmit = run_music_img_threads(media_lists.1.clone());

    // let ab_list = crate::setup::rusic_misc::create_art_alb_list(media_lists.0.clone());
    // let _artist_list = crate::setup::rusic_misc::create_artistids(ab_list.0);
    // let _album_list = crate::setup::rusic_misc::create_albumids(ab_list.1);



    println!("music: {}\n", media_lists.0.clone().len());
    println!("images: {}\n", media_lists.1.clone().len());

    let duration = start.elapsed();
    println!("Setup completed in: {} seconds", duration.as_secs());

    Ok(())
}

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
    };

    true
}

fn run_music_img_threads(alist: Vec<String>) -> bool {
    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();

    let mut img_index = 0;
    for i in alist {
        img_index = img_index + 1;
        if i.contains("Music") {
            // let tx = tx.clone();
            // pool.execute(move || {
                let img_info =
                    setup::rusic_process_music_images::process_music_images(i.clone(), img_index);
            //     tx.send(img_info).expect("Could not send data");
            // });
        }
    }

    // drop(tx);
    // for t in rx.iter() {
    //     // Insert this into db
    //     let ifo = t;
    //     println!("Processed Music img {:?} files", ifo);
    // }

    true
}

