use filesize::PathExt;
use id3::{Tag, TagLike};
use image::{self};
use md5::{Digest, Md5};
use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub struct RusicUtils {
    pub apath: String,
}

impl RusicUtils {
    pub fn split_base_dir_filename(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let dir_path = path.parent().unwrap();
        let filename = path.file_name().unwrap();

        (
            dir_path.to_string_lossy().to_string(),
            filename.to_string_lossy().to_string(),
        )
    }

    pub fn split_artist_album(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let basedir = path.parent().unwrap();
        let basedirpath = Path::new(&basedir);
        let album = basedirpath.file_name().unwrap();
        let basedirpath2 = basedirpath.parent().unwrap();
        let bdp3 = Path::new(&basedirpath2);
        let artist = bdp3.file_name().unwrap();

        (
            artist.to_string_lossy().to_string(),
            album.to_string_lossy().to_string(),
        )
    }

    pub fn get_tag_info(&self) -> (String, String, String) {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let artist = tag.artist().expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        let song = tag.title().expect(&self.apath);

        (artist.to_string(), album.to_string(), song.to_string())
    }

    pub fn split_ext(&self) -> String {
        let path = Path::new(&self.apath);
        let boo_results = path.extension();
        let boo = match boo_results {
            Some(b) => b.to_string_lossy().to_string(),
            None => "split_ext did not work".to_string(),
        };

        let ext = ".".to_string() + boo.as_str();

        ext
    }

    pub fn get_file_size(&self) -> String {
        let path = Path::new(&self.apath);

        path.size_on_disk().unwrap().to_string()
    }

    pub fn get_duration(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let dur_sec_res = mp3_duration::from_path(&path);
        let dur_sec = match dur_sec_res {
            Ok(d) => d,
            Err(_) => Duration::new(0, 0),
        };
        if dur_sec != Duration::new(0, 0) {
            let dur_min = dur_sec.div_f32(60.0);
            let dur_str = format!("{:?}", dur_min);
            let mut durvec = vec![];
            for i in dur_str.chars() {
                durvec.push(i);
            }

            let mut newvec = vec![];
            let mut count: i32 = 0;
            for c in durvec {
                count = count + 1;
                if count < 5 {
                    newvec.push(c);
                } else {
                    break;
                };
            }

            let duration: String = newvec.into_iter().collect();
            return (duration.clone(), self.apath.clone());
        } else {
            let new_dur = Duration::new(0, 0);
            let duration = format!("{:?}", new_dur);
            return (duration.clone(), self.apath.clone());
        };
    }

    pub fn get_dims(&self) -> (u32, u32) {
        let dims = get_image_dims(&self.apath);

        dims
    }
}

pub fn get_md5(z: String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(&z);
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

fn get_image_dims(x: &String) -> (u32, u32) {
    let dims_rs = image::image_dimensions(&x);
    let dims = match dims_rs {
        Ok(d) => d,
        Err(_) => (0, 0),
    };

    dims
}

pub fn normalize_music_image(dims: (u32, u32)) -> (u32, u32) {
    let largest: u32;

    if dims.0 == dims.1 {
        largest = dims.0;
    } else if dims.0 > dims.1 {
        largest = dims.0;
    } else {
        largest = dims.1;
    }

    let resizetup: (u32, u32);
    if largest < 100 {
        resizetup = (100, 100);
    } else if largest < 200 {
        resizetup = (200, 200);
    } else if largest < 300 {
        resizetup = (300, 300);
    } else {
        resizetup = (300, 300);
    }

    resizetup
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
