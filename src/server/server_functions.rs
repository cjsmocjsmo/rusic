use actix_web::{get, post, web, HttpResponse, Responder};

// use actix_web::web::Json;
use rusqlite::Connection;
use std::env;
use serde::{Deserialize, Serialize};
// use anyhow::Error;

// use crate::setup::rusic_process_music;

#[get("/test")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/artist/{alpha}")]
pub async fn artistalpha(a: web::Path<String>) -> impl Responder {
    let alpha = a.into_inner();
    println!("alpha: {}", alpha.clone());
    let op = "artist";
    let artist_info_list = fetch_media_by_alpha(alpha, op);
    let json = serde_json::to_string(&artist_info_list).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/album/{alpha}")]
pub async fn albumalpha(a: web::Path<String>) -> impl Responder {
    let alpha = a.into_inner();
    let op = "album";
    let album_info_list = fetch_media_by_alpha(alpha, op);
    let json = serde_json::to_string(&album_info_list).unwrap();

    HttpResponse::Ok().body(json)
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtArtidInfo {
    pub rusticid: String,
    pub artist: String,
    pub artistid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbAlbidInfo {
    pub rusticid: String,
    pub album: String,
    pub albumid: String,
}


pub fn fetch_media_by_alpha(alpha: String, op: &str) -> Vec<ArtArtidInfo> {
    println!("alpha: {}, {}", alpha.clone(), op.clone());
    //get artistid from startswith db
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mut id_list = Vec::new();
    if op == "artist" {
        let mut stmt = conn
            .prepare("SELECT DISTINCT artistid FROM startswith WHERE artist_first_letter = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&alpha]).expect("Unable to query db");
        while let Some(row) = rows.next().unwrap() {
            let mediaid: String = row.get(0).unwrap();
            id_list.push(mediaid);
        }
    } else if op == "album" {
        let mut stmt = conn
            .prepare("SELECT DISTINCT artistid FROM startswith WHERE album_first_letter = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&alpha]).expect("Unable to query db");
        while let Some(row) = rows.next().unwrap() {
            let mediaid: String = row.get(0).unwrap();
            id_list.push(mediaid);
        }
    };
    println!("id_list: {:?}", id_list.clone());

    //get artist info for each artistid and return json
    let mut artist_info_list = Vec::new();
    for artid in id_list {
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT * FROM artartid WHERE artistid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&artid]).expect("Unable to query db");
        while let Some(row) = rows.next().expect("Unable to get next row") {
            let artist_info = ArtArtidInfo {
                rusticid: row.get(1).unwrap(),
                artist: row.get(2).unwrap(),
                artistid: row.get(3).unwrap(),
            };
            println!("artist_info: {:?}", artist_info.clone());

            artist_info_list.push(artist_info);
        }
    }

    artist_info_list
}
