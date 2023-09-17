use crate::server::fragments;
use crate::types;
use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;

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
    let json = serde_json::to_string(&artist_count_vec).unwrap();

    HttpResponse::Ok().body(json)
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
    let json = serde_json::to_string(&album_count_vec).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/artistforalpha/{alpha}")]
pub async fn artistalpha(a: web::Path<String>) -> impl Responder {
    let alpha = a.into_inner();
    println!("alpha: {}", alpha.clone());
    let artist_info_list = fragments::fetch_artist_count_by_alpha(alpha);
    let json = serde_json::to_string(&artist_info_list).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/albumforalpha/{alpha}")]
pub async fn albumalpha(a: web::Path<String>) -> impl Responder {
    let alpha = a.into_inner();
    let album_info_list = fragments::fetch_album_count_by_alpha(alpha);
    let json = serde_json::to_string(&album_info_list).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/albforart/{artistid}")]
pub async fn albforart(a: web::Path<String>) -> impl Responder {
    let artistid = a.into_inner();
    let alb_for_art = fragments::fetch_albforart(artistid);
    let json = serde_json::to_string(&alb_for_art).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/songsforalbum/{albumid}")]
pub async fn songsforalbum(a: web::Path<String>) -> impl Responder {
    let albumid = a.into_inner();
    let songs_for_album = fragments::fetch_songs_for_album(albumid);
    let json = serde_json::to_string(&songs_for_album).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/createemptyplaylist/{playlistname}")]
pub async fn createemptyplaylist(x: web::Path<String>) -> impl Responder {
    let x = x.into_inner();
    let empty_playlist = fragments::create_empty_playlist(x.clone());
    let json = serde_json::to_string(&empty_playlist).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/createrandomplaylist/{playlistname}/{offset}")]
pub async fn createrandomplaylist(x: web::Path<(String, String)>) -> impl Responder {
    let (playlistname, offset) = x.into_inner();
    let random_playlist = fragments::create_random_playlist(playlistname.clone(), offset.clone());
    let json = serde_json::to_string(&random_playlist).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/allplaylists")]
pub async fn allplaylists() -> impl Responder {
    let all_playlists = fragments::fetch_all_playlists();
    let json = serde_json::to_string(&all_playlists).unwrap();

    HttpResponse::Ok().body(json)
}

