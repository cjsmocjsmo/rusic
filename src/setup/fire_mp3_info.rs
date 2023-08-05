use id3::{Tag, TagLike};
use mp3_duration;
use std::path::Path;
use std::time::Duration;

pub fn get_tag_info(x: &String) -> (String, String, String) {
    let tag = Tag::read_from_path(x).expect(x);
    let artist = tag.artist().expect(x);
    let album = tag.album().expect(x);
    let song = tag.title().expect(x);

    (artist.to_string(), album.to_string(), song.to_string())
}

fn mp3_duration_extract(x: String) -> Duration {
    let path = Path::new(&x);
    let dur_sec_res = mp3_duration::from_path(&path);
    let dur_sec = match dur_sec_res {
        Ok(d) => d,
        Err(_) => Duration::new(0, 0),
    };

    dur_sec.clone()
}

pub fn get_duration(x: &String) -> String {
    let dur_sec = mp3_duration_extract(x.to_string());
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

