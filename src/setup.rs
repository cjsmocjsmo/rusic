pub mod rusic_tables;
pub mod rusic_process_music;
pub mod rusic_process_music_images;
pub mod rusic_utils;
pub mod rusic_walk_dirs;
pub mod rusic_artist;
pub mod rusic_album;

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