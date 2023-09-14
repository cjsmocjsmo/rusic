use actix_web::{get, web, HttpResponse, Responder};
use std::path::Path;

// use actix_web::web::Json;
use rusqlite::Connection;
// use serde::{Deserialize, Serialize};
use std::env;
// use anyhow::Error;
use crate::types;

#[get("/test")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}

#[get("/artistcount")]
pub async fn artistcount() -> impl Responder {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn.prepare("SELECT * FROM artistcount;").unwrap();
    let mut rows = stmt.query([]).expect("Unable to query db");
    let mut artist_count_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {

        let artist_count = types::ArtistCount {
            alpha: row.get(1).unwrap(),
            count: row.get(2).unwrap(),
        };
        artist_count_vec.push(artist_count);
    }

    println!("artist_count_vec: {:?}", artist_count_vec.clone());

    // THIS IS NEED BY SVELTE DONT DELETE
    let json = serde_json::to_string(&artist_count_vec).unwrap();
    HttpResponse::Ok().body(json)

    // let frag  = crate::server::fragments::frag_artiscount(artist_count_vec);
    // HttpResponse::Ok().body(frag)
}

#[get("/albumcount")]
pub async fn albumcount() -> impl Responder {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut stmt = conn.prepare("SELECT * FROM albumcount;").unwrap();
    let mut rows = stmt.query([]).expect("Unable to query db");
    let mut album_count_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let album_count = types::AlbumCount {
            alpha: row.get(1).unwrap(),
            count: row.get(2).unwrap(),
        };
        album_count_vec.push(album_count);
    }

    println!("album_count_vec: {:?}", album_count_vec.clone());

    // THIS IS NEED BY SVELTE DONT DELETE
    let json = serde_json::to_string(&album_count_vec).unwrap();
    HttpResponse::Ok().body(json)

//     let frag  = crate::server::fragments::frag_albumcount(album_count_vec);
//     HttpResponse::Ok().body(frag)
}

#[get("/artistforalpha/{alpha}")]
pub async fn artistalpha(a: web::Path<String>) -> impl Responder {
    let alpha = a.into_inner();
    println!("alpha: {}", alpha.clone());
    let artist_info_list = fetch_artist_count_by_alpha(alpha);
    let json = serde_json::to_string(&artist_info_list).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/albumforalpha/{alpha}")]
pub async fn albumalpha(a: web::Path<String>) -> impl Responder {
    let alpha = a.into_inner();
    let album_info_list = fetch_album_count_by_alpha(alpha);
    let json = serde_json::to_string(&album_info_list).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/albforart/{artistid}")]
pub async fn albforart(a: web::Path<String>) -> impl Responder {
    let artistid = a.into_inner();
    let alb_for_art = fetch_albforart(artistid);
    let json = serde_json::to_string(&alb_for_art).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/songsforalbum/{albumid}")]
pub async fn songsforalbum(a: web::Path<String>) -> impl Responder {
    let albumid = a.into_inner();
    let songs_for_album = fetch_songs_for_album(albumid);
    let json = serde_json::to_string(&songs_for_album).unwrap();

    HttpResponse::Ok().body(json)
}

fn fetch_songs_for_album(x: String) -> Vec<types::MusicInfo> {
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

    };

    song_vec
}

fn split_path(path: String) -> String {
    let path = Path::new(&path);
    let components = path.components();
    let mut components_vec = Vec::new();
    for component in components {
        let foo = component.as_os_str().to_str().unwrap();
        components_vec.push(foo.to_string());
    };
    components_vec.drain(0..4);

    let ffile = components_vec.join("/");

    let http_addr = env::var("RUSIC_HTTP_ADDR").expect("RUSIC_HTTP_ADDR not set");
    let http_port = env::var("RUSIC_PORT").expect("RUSIC_PORT not set");
    let http_addr_port = http_addr + &http_port + "/" + &ffile;
    // let http_addr_port_file = http_addr_port;

    http_addr_port
  }

fn fetch_albforart(artid: String) -> Vec<types::AlbAlbidInfo> {
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
    };

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
    };

    println!("new_album_info_list: {:#?}", new_album_info_list.clone());

    new_album_info_list

}

fn fetch_artist_count_by_alpha(alpha: String) -> Vec<types::ArtArtidInfo> {
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

    //get artist info for each artistid and return json
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
    };

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
    };


    // THIS IS NEED BY SVELTE DONT DELETE
    println!("new_artist_info: {:#?}", new_artist_info_list.clone());
    new_artist_info_list


    // let frag  = crate::server::fragments::frag_artist_for_alpha(new_artist_info_list);
    // frag
}

pub fn fetch_album_count_by_alpha(alpha: String) -> Vec<(String, String)> {
    println!("alpha: {}", alpha.clone());
    //get artistid from startswith db
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

    //get artist info for each artistid and return json
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
    println!("artist_info: {:?}", album_info_list.clone());

    album_info_list
}
