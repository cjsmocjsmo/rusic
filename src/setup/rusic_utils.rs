use filesize::PathExt;
use id3::{Tag, TagLike};
use md5::{Digest, Md5};
use std::env;
use std::path::Path;
use anyhow::anyhow;
use std::time::Duration;
use image::{self};

#[derive(Debug)]
pub struct RusicUtils {
    pub apath: String,
}

impl RusicUtils {
    pub fn split_base_dir_filename(&self) -> (String, String){
        let path = Path::new(&self.apath);

        let dir_path = path.parent().unwrap();
        let filename = path.file_name().unwrap();

        (dir_path.to_string_lossy().to_string(), filename.to_string_lossy().to_string())
    }
    pub fn split_base_dir(&self) -> String {
        let mysplit = self.apath.split("/");
        let mut myvec = vec![];
        for my in mysplit {
            myvec.push(my);
        }
        let path = env::var("RUSIC_THUMBS").unwrap();
        let envsplit = path.split("/");
        let mut envvec = vec![];
        for env in envsplit {
            envvec.push(env);
        }
        let count = envvec.len() - 2;
        myvec.drain(0..count);
        myvec.pop();
        let base_dir = myvec.join("/");
        base_dir
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
    pub fn image_split_artist(&self) -> String {
        let filesplit = self.apath.split("/");
        let mut fspvec = Vec::new();
        for fp in filesplit.clone() {
            fspvec.push(fp);
        }

        let fspcount = fspvec.len();
        let fspidx = fspcount.clone() - 3;

        let fsp = &fspvec[fspidx];
        let art = format!("{:?}", &fsp);
        let artist = art.replace("\"", "");

        artist
    }
    pub fn music_split_artist(&self) -> (String, String) {
        let filesplit = self.apath.split("/");
        let mut filenamevec = vec![];
        for file in filesplit {
            filenamevec.push(file);
        }

        let mut fin = vec![];
        for f in filenamevec {
            fin.push(f);
        }

        let artist = String::from(fin[7]);
        let album = String::from(fin[8]);

        (artist, album)
    }
    pub fn image_split_album(&self) -> String {
        let filesplit = self.apath.split("/");
        let mut fspvec = Vec::new();
        for fp in filesplit.clone() {
            fspvec.push(fp);
        }

        let fspcount = fspvec.len();
        let fspidx = fspcount.clone() - 2;

        let fsp = &fspvec[fspidx];
        let alb = format!("{:?}", &fsp);
        let album = alb.replace("\"", "");

        album
    }
    pub fn split_filename(&self) -> String {
        let filesplit = self.apath.split("/");
        let mut filenamevec = vec![];
        for file in filesplit {
            filenamevec.push(file);
        }

        let count = &filenamevec.len() - 1;
        filenamevec.drain(0..count);
        let mut finalvec = "";
        for f in filenamevec {
            finalvec = f;
        }

        let fname = finalvec.split(".");
        let mut svec = vec![];
        for f in fname {
            svec.push(f);
        }
        svec.pop();

        let filename = svec.get(0).unwrap();

        filename.to_string()
    }


    pub fn get_tag_info(&self) -> (String, String, String) {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let artist = tag.artist().expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        let song = tag.title().expect(&self.apath);

        (artist.to_string(), album.to_string(), song.to_string())
    }

    pub fn extract_coverart(&self) -> anyhow::Result<String> {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let artist = tag.artist().expect("artist has fucked up");
        let album = tag.album().expect("album has fucked up");
        let mut coverart_path = env::var("RUSIC_THUMBS").unwrap();
        coverart_path.push_str(artist);
        coverart_path.push_str("_-_");
        coverart_path.push_str(album);
        coverart_path.push_str(".jpg");
        let c_path = Path::new(&coverart_path);

        if c_path.exists() {
            Ok(coverart_path)
        } else {
            let first_picture = tag.pictures().next();
            if let Some(p) = first_picture {
                match image::load_from_memory(&p.data) {
                    Ok(image) => {
                        image.save(&coverart_path).map_err(|e| {
                            anyhow!("Couldn't write image file {:?}: {}", coverart_path, e)
                        })?;
                    }
                    Err(e) => return Err(anyhow!("Couldn't load image: {}", e)),
                };

                Ok(coverart_path)
            } else {
                Err(anyhow!("No image found in music file"))
            }
        }
    }

    pub fn get_file_size(&self) -> String {
        let path = Path::new(&self.apath);

        path.size_on_disk().unwrap().to_string()
    }
    pub fn get_md5(&self) -> String {
        let mut hasher2 = Md5::new();
        hasher2.update(&self.apath);
        let a_id = hasher2.finalize();
        let foo = format!("{:x}", a_id);

        foo
    }

    pub fn get_duration(&self) -> String {
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
            return duration.clone();
        } else {
            let new_dur = Duration::new(0, 0);
            let duration = format!("{:?}", new_dur);
            return duration;
        };
    }

    pub fn get_dims(&self) -> (u32, u32) {
        let dims = get_image_dims(&self.apath);

        dims
    }
}

fn get_image_dims(x: &String) -> (u32, u32) {
    let dims_rs = image::image_dimensions(&x);
    let dims = match dims_rs {
        Ok(d) => d,
        Err(_) => (0, 0)
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
