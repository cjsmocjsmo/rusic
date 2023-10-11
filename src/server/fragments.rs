use crate::rusicdb;
use crate::setup::rusic_utils::get_md5;
use crate::types;
use rand::Rng;
use rusqlite::Connection;
use std::env;
use std::path::Path;

pub fn create_empty_playlist(x: String) -> bool {
    println!("x: {}", x.clone());
    let plinfo = types::PlayList {
        rusicid: get_md5(x.clone()),
        name: x.clone(),
        songs: "None".to_string(),
        numsongs: "0".to_string(),
    };

    println!("plinfo: {:#?}", plinfo.clone());

    let _insert_pl = rusicdb::db_main::post_playlist_to_db(plinfo);

    true
}

fn get_rand_idx_vec(offset: String) -> Vec<String> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");

    let mut stmt = conn.prepare("SELECT songcount FROM stats").unwrap();

    let mut song_count: String = "0".to_string();
    let mut rows = stmt.query([]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        song_count = row.get(0).unwrap();
    }
    let song_count_i64 = song_count.parse::<i64>().unwrap();
    let offset_i64 = offset.parse::<i64>().unwrap();

    let mut random_numbers = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..offset_i64 {
        random_numbers.push(rng.gen_range(0..song_count_i64));
    }

    let mut random_idx_numbers = Vec::new();
    for random in random_numbers {
        let randy = random.to_string();
        random_idx_numbers.push(randy);
    }

    random_idx_numbers
}

pub fn create_random_playlist(x: String, offset: String) -> bool {
    let randidxvec = get_rand_idx_vec(offset.clone());
    let mut new_song_rusicid_vec = Vec::new();
    for idx in randidxvec {
        let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");

        let mut stmt = conn
            .prepare("SELECT rusicid FROM music WHERE idx = ?1")
            .unwrap();
        let mut newsongsidvec = Vec::new();
        let mut rows = stmt.query(&[&idx]).expect("Unable to query db");
        while let Some(row) = rows.next().unwrap() {
            let song_rusicid: String = row.get(0).unwrap();
            newsongsidvec.push(song_rusicid);
        }
        new_song_rusicid_vec.push(newsongsidvec);
    }

    let rusicid_json = serde_json::to_string(&new_song_rusicid_vec).unwrap();

    let plinfo = types::PlayList {
        rusicid: get_md5(x.clone()),
        name: x.clone(),
        songs: rusicid_json.clone(),
        numsongs: offset.clone(),
    };

    let _insert_pl = rusicdb::db_main::post_playlist_to_db(plinfo).unwrap();

    true
}

pub fn fetch_songs_for_album(x: String) -> Vec<types::MusicInfo> {
    let mut song_vec = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM music WHERE albumid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&x]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let fpath: String = row.get(8).unwrap();
        let fupath = split_path(fpath);

        let song_info = types::MusicInfo {
            rusicid: row.get(1).unwrap(),
            imgurl: row.get(2).unwrap(),
            artist: row.get(3).unwrap(),
            artistid: row.get(4).unwrap(),
            album: row.get(5).unwrap(),
            albumid: row.get(6).unwrap(),
            song: row.get(7).unwrap(),
            fullpath: fupath,
            extension: row.get(9).unwrap(),
            idx: row.get(10).unwrap(),
            page: row.get(11).unwrap(),
            fsizeresults: row.get(12).unwrap(),
        };
        song_vec.push(song_info);
    }

    song_vec
}

pub fn split_path(path: String) -> String {
    let path = Path::new(&path);
    let components = path.components();
    let mut components_vec = Vec::new();
    for component in components {
        let foo = component.as_os_str().to_str().unwrap();
        components_vec.push(foo.to_string());
    }
    components_vec.drain(0..4);

    let ffile = components_vec.join("/");

    let http_addr = env::var("RUSIC_HTTP_ADDR").expect("RUSIC_HTTP_ADDR not set");
    let http_port = env::var("RUSIC_PORT").expect("RUSIC_PORT not set");
    let http_addr_port = http_addr + &http_port + "/" + &ffile;

    http_addr_port
}

pub fn fetch_albforart(artid: String) -> Vec<types::AlbAlbidInfo> {
    println!("artid: {}", artid.clone());
    let mut albumidvec = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT distinct albumid FROM music WHERE artistid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&artid]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let album_id: String = row.get(0).unwrap();
        albumidvec.push(album_id);
    }

    println!("albumidvec: {:#?}", albumidvec.clone());

    let mut album_info_list = Vec::new();
    let mut album_info_vec = Vec::new();
    for albumid in albumidvec {
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT * FROM albalbid WHERE albumid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&albumid]).expect("Unable to query db");
        while let Some(row) = rows.next().expect("Unable to get next row") {
            let album_info = types::AlbAlbidInfo {
                rusticid: row.get(1).unwrap(),
                imageurl: row.get(2).unwrap(),
                albumid: row.get(3).unwrap(),
            };

            println!("album_info: {:#?}", album_info.clone());
            album_info_vec.push(album_info.clone());
        }
    }

    for alb in album_info_vec {
        let foo = alb.imageurl.clone();
        let bar = alb.albumid.clone();
        let baz = (foo, bar);
        album_info_list.push(baz);
    }

    album_info_list.sort();
    album_info_list.dedup();

    println!("album_info: {:?}", album_info_list.clone());

    let mut new_album_info_list = Vec::new();
    let mut count = 0;
    for album in album_info_list.clone() {
        count += 1;
        let stringcount = count.to_string();

        let albuminfo = types::AlbAlbidInfo {
            rusticid: stringcount.clone(),
            imageurl: album.0.clone(),
            albumid: album.1.to_string(),
        };
        new_album_info_list.push(albuminfo);
    }

    println!("new_album_info_list: {:#?}", new_album_info_list.clone());

    new_album_info_list
}

pub fn fetch_artist_count_by_alpha(alpha: String) -> Vec<types::ArtArtidInfo> {
    println!("alpha: {}", alpha.clone());
    //get artistid from startswith db
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut id_list = Vec::new();

    let mut stmt = conn
        .prepare("SELECT DISTINCT artistid FROM startswith WHERE artist_first_letter = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&alpha]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let mediaid: String = row.get(0).unwrap();
        id_list.push(mediaid);
    }

    println!("id_list: {:?}", id_list.clone());

    let mut artist_info_list = Vec::new();
    let mut art_vec = Vec::new();
    for artid in id_list {
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT * FROM artartid WHERE artistid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&artid]).expect("Unable to query db");
        while let Some(row) = rows.next().expect("Unable to get next row") {
            let artist_info = types::ArtArtidInfo {
                rusticid: row.get(1).unwrap(),
                artist: row.get(2).unwrap(),
                artistid: row.get(3).unwrap(),
            };

            art_vec.push(artist_info);
        }
    }

    for art in art_vec {
        let foo = art.artist.clone();
        let bar = art.artistid.clone();
        let baz = (foo, bar);
        artist_info_list.push(baz);
    }

    artist_info_list.sort();
    artist_info_list.dedup();

    let mut new_artist_info_list = Vec::new();
    let mut count = 0;
    for artist in artist_info_list.clone() {
        count += 1;
        let stringcount = count.to_string();

        let artistinfo = types::ArtArtidInfo {
            rusticid: stringcount.clone(),
            artist: artist.0.clone(),
            artistid: artist.1.to_string(),
        };
        new_artist_info_list.push(artistinfo);
    }

    println!("new_artist_info: {:#?}", new_artist_info_list.clone());

    new_artist_info_list
}

pub fn fetch_album_count_by_alpha(alpha: String) -> Vec<types::AlbAlbidInfo> {
    println!("alpha: {}", alpha.clone());
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut id_list = Vec::new();
    let mut stmt = conn
        .prepare("SELECT DISTINCT albumid FROM startswith WHERE album_first_letter = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&alpha]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let mediaid: String = row.get(0).unwrap();
        id_list.push(mediaid);
    }
    println!("id_list: {:?}", id_list.clone());

    let mut album_info_list = Vec::new();
    let mut alb_vec = Vec::new();
    for albid in id_list {
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT * FROM albalbid WHERE albumid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&albid]).expect("Unable to query db");
        while let Some(row) = rows.next().expect("Unable to get next row") {
            let album_info = types::AlbAlbidInfo {
                rusticid: row.get(1).unwrap(),
                imageurl: row.get(2).unwrap(),
                albumid: row.get(3).unwrap(),
            };

            alb_vec.push(album_info);
        }
    }

    for alb in alb_vec {
        let foo = alb.imageurl.clone();
        let bar = alb.albumid.clone();
        let baz = (foo, bar);
        album_info_list.push(baz);
    }

    album_info_list.sort();
    album_info_list.dedup();

    let mut new_album_info_list = Vec::new();
    for album in album_info_list.clone() {
        // count += 1;
        let albpath = "/songsforalbum/".to_string() + &album.1.to_string();

        let albuminfo = types::AlbAlbidInfo {
            rusticid: albpath.clone(),
            imageurl: album.0.clone(),
            albumid: album.1.to_string(),
        };
        new_album_info_list.push(albuminfo);
    }

    println!("new_album_info_list: {:?}", new_album_info_list.clone());

    new_album_info_list
}

pub fn fetch_all_playlists() -> Vec<types::PlayList> {
    let mut pl_vec = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn.prepare("SELECT * FROM playlists").unwrap();
    let mut rows = stmt.query([]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let plinfo = types::PlayList {
            rusicid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            songs: row.get(3).unwrap(),
            numsongs: row.get(4).unwrap(),
        };
        pl_vec.push(plinfo);
    }

    pl_vec
}

pub fn delete_playlist(x: String) -> bool {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");

    let mut stmt = conn
        .prepare("DELETE FROM playlists WHERE rusicid = ?1")
        .unwrap();
    let _rows = stmt.execute(&[&x]).expect("Unable to query db");

    true
}

pub fn add_song_to_my_likes(rid: String) -> bool {
    let old = rusicdb::db_playlist::get_mylikes_oldsongs();
    let oldsongs = old.0;
    let oldnumsongs = old.1;

    if oldsongs == "None" {
        let newsongvec = vec![rid.clone()];
        let newsongvec_json = serde_json::to_string(&newsongvec).unwrap();
        let numsongs = "1".to_string();
        let update_mylikes_result = rusicdb::db_playlist::update_mylikes(
            newsongvec_json.clone(),
            numsongs.clone(),
            "mylikes".to_string(),
        );

        return update_mylikes_result;
    } else {
        let mut oldsongvec: Vec<String> = serde_json::from_str(&oldsongs).unwrap();
        oldsongvec.push(rid.clone());
        let newsongvec_json = serde_json::to_string(&oldsongvec).unwrap();
        let oldnumsongs_i64 = oldnumsongs.parse::<i64>().unwrap();
        let newnumsongs_i64 = oldnumsongs_i64 + 1;
        let newnumsongs = newnumsongs_i64.to_string();
        let update_mylikes_result = rusicdb::db_playlist::update_mylikes(
            newsongvec_json.clone(),
            newnumsongs.clone(),
            "mylikes".to_string(),
        );

        return update_mylikes_result;
    };
}

pub fn get_playlist_oldsongs(playlistid: String) -> (String, String) {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    // let mylikes = "mylikes".to_string();
    let mut stmt = conn
        .prepare("SELECT * FROM playlists WHERE rusicid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&playlistid]).expect("Unable to query db");

    let mut oldsongs = String::new();
    let mut oldnumsongs = String::new();
    while let Some(row) = rows.next().unwrap() {
        let oldplinfo = types::PlayList {
            rusicid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            songs: row.get(3).unwrap(),
            numsongs: row.get(4).unwrap(),
        };
        oldsongs = oldplinfo.songs;
        oldnumsongs = oldplinfo.numsongs;
    }

    (oldsongs, oldnumsongs)
}

pub fn update_playlist(playlistid: String, songs: String, numsongs: String) -> bool {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");

    let mut stmt = conn
        .prepare("UPDATE playlists SET songs = ?1, numsongs = ?2 WHERE rusicid = ?3")
        .unwrap();
    let _rows = stmt
        .execute(&[&songs, &numsongs, &playlistid])
        .expect("Unable to query db");

    true
}

pub fn add_song_to_playlist(playlistid: String, songid: String) -> bool {
    let oldies = get_playlist_oldsongs(playlistid.clone());
    let oldsongs = oldies.0;
    let oldnumsongs = oldies.1;
    println!("oldsongs: {}", oldsongs.clone());
    println!("oldnumsongs: {}", oldnumsongs.clone());

    if oldsongs == "None" {
        let newsongvec = vec![songid.clone()];
        let newsongvec_json = serde_json::to_string(&newsongvec).unwrap();
        let numsongs = "1".to_string();
        let update_playlist_result = update_playlist(
            playlistid.clone(),
            newsongvec_json.clone(),
            numsongs.clone(),
        );
        println!("update_playlist_result: {}", update_playlist_result.clone());

        return update_playlist_result;
    } else {
        let mut oldsongvec: Vec<String> = serde_json::from_str(&oldsongs).unwrap();
        oldsongvec.push(songid.clone());
        let newsongvec_json = serde_json::to_string(&oldsongvec).unwrap();
        let oldnumsongs_i64 = oldnumsongs.parse::<i64>().unwrap();
        let newnumsongs_i64 = oldnumsongs_i64 + 1;
        let newnumsongs = newnumsongs_i64.to_string();
        let update_playlist_result = update_playlist(
            playlistid.clone(),
            newsongvec_json.clone(),
            newnumsongs.clone(),
        );
        println!("update_playlist_result: {}", update_playlist_result.clone());

        return update_playlist_result;
    };
}

fn playlist_raw_data(plid: String) -> Vec<types::PlayList> {
    let mut newdbvec = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM playlists WHERE rusicid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&plid]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let oldplinfo = types::PlayList {
            rusicid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            songs: row.get(3).unwrap(),
            numsongs: row.get(4).unwrap(),
        };
        newdbvec.push(oldplinfo);
    }

    newdbvec
}

fn fetch_song_by_rusicid(rusicid: String) -> types::MusicInfo {
    let mut mfovec = Vec::new();
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT * FROM music WHERE rusicid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&rusicid]).expect("Unable to query db");
    while let Some(row) = rows.next().unwrap() {
        let song_info = types::MusicInfo {
            rusicid: row.get(1).unwrap(),
            imgurl: row.get(2).unwrap(),
            artist: row.get(3).unwrap(),
            artistid: row.get(4).unwrap(),
            album: row.get(5).unwrap(),
            albumid: row.get(6).unwrap(),
            song: row.get(7).unwrap(),
            fullpath: row.get(8).unwrap(),
            extension: row.get(9).unwrap(),
            idx: row.get(10).unwrap(),
            page: row.get(11).unwrap(),
            fsizeresults: row.get(12).unwrap(),
        };
        println!("song_info: {:#?}", song_info.clone());
        mfovec.push(song_info);
    };

    mfovec[0].clone()
}

pub fn get_playlist_data(playlistid: String) -> bool {
    let mut pl_name = String::new();
    let mut pl_raw_songs = String::new();
    let pl_raw_data = playlist_raw_data(playlistid.clone());
    for pl in pl_raw_data {
        pl_name = pl.name;
        pl_raw_songs = pl.songs;

    };
    let plsongs: Vec<Vec<String>> = serde_json::from_str(&pl_raw_songs).unwrap();
    println!("pl_name: {}", pl_name.clone());
    println!("pl_raw_songs: {}", pl_raw_songs.clone());

    for rid in plsongs.clone() {
        println!("this is rid: {:#?}", rid.clone());
    }

    // let songs_rusicid_vec: Vec<String> = serde_json::from_str(&pl_raw_songs).unwrap();
    // println!("songs_rusicid_vec: {:?}", songs_rusicid_vec.clone());
    // let mut songs_info_vec = Vec::new();
    // for rusicid in pl_raw_data {
    //     let song_info = fetch_song_by_rusicid(rusicid);
    //     songs_info_vec.push(song_info);
    // };

    // println!("songs_info_vec: {:?}", songs_info_vec.clone());

    // songs_info_vec

    true

}
