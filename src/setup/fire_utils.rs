use std::env;
use std::path::Path;
use filesize::PathExt;
use md5::{Digest, Md5};

#[derive(Debug)]
pub struct FireUtils {
    pub apath: String
    
}

impl FireUtils {
    pub fn split_base_dir(&self) -> String {
        let mysplit = self.apath.split("/");
        let mut myvec = vec![];
        for my in mysplit {
            myvec.push(my);
        }
        let path = env::var("FIRE_THUMBNAILS").unwrap();
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
        };

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
        
        let artist = String::from(fin[5]);
        let album = String::from(fin[6]);

        (artist, album)
    }
    pub fn image_split_album(&self) -> String {
        let filesplit = self.apath.split("/");
        let mut fspvec = Vec::new();
        for fp in filesplit.clone() {
            fspvec.push(fp);
        };

        let fspcount = fspvec.len();
        let fspidx = fspcount.clone() - 2;

        let fsp = &fspvec[fspidx];
        let alb = format!("{:?}", &fsp);
        let album = alb.replace("\"", "");

        album
    }
    // pub fn music_split_album(&self) -> String {
    //     let filesplit = self.apath.split("/");
    //     let mut filenamevec = vec![];
    //     for file in filesplit {
    //         filenamevec.push(file);
    //     }

    //     let count = &filenamevec.len() - 2;
    //     filenamevec.drain(0..count);
    //     let mut album = "";
    //     for f in filenamevec {
    //         album = f;
    //     }

    //     String::from(album)
    // }
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
    pub fn split_movie_name(&self) -> String {
        let filesplit = self.apath.split("/");
        let mut filenamevec: Vec<String> = vec![];
        for file in filesplit {
            filenamevec.push(file.to_string());
        }
        let raw_fname = filenamevec.pop().unwrap();

        let fsplit = raw_fname.split(" (");
        let mut fsplit_vec = vec![];
        for f in fsplit {
            fsplit_vec.push(f);
        }

        fsplit_vec[0].to_string()
    }
    pub fn split_movie_year(&self) -> String {
        let filesplit = self.apath.split("/");

        let mut filenamevec: Vec<String> = vec![];
        for file in filesplit {
            filenamevec.push(file.to_string());
        }
        let raw_fname = filenamevec.pop().unwrap();

        let fsplit = raw_fname.split(" (");
        let mut fsplit_vec = vec![];
        for f in fsplit {
            fsplit_vec.push(f);
        }

        // println!("this is split_vec{:?}", fsplit_vec.clone());

        let fsplit2 = fsplit_vec[1].split(")");
        let mut fsplit_vec2 = vec![];
        for f2 in fsplit2 {
            fsplit_vec2.push(f2);
        }

        fsplit_vec2[0].to_string()
        // fsplit_vec[0].clone().to_string()

    }
    pub fn split_poster_name(&self) -> String {
        let filesplit = self.apath.split("/");

        let mut filenamevec: Vec<String> = vec![];
        for file in filesplit {
            filenamevec.push(file.to_string());
        }
        let fname = filenamevec.pop().unwrap();

        fname
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
    
    pub fn get_dims(&self) -> (u32, u32) {
        let dims = crate::setup::fire_image::get_image_dims(&self.apath);

        dims
     }
}