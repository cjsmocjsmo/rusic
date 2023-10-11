use crate::server::fragments;
use crate::types;
use actix_web::{get, web, HttpResponse, Responder};
use rusqlite::Connection;
use std::env;

#[get("/test")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}
#[get("/randomcoverart")]
pub async fn randomcoverart() -> impl Responder {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");

    let mut stmt = conn
        .prepare("SELECT * FROM music_images ORDER BY RANDOM() LIMIT 5;")
        .unwrap();

    let mut rows = stmt.query([]).expect("Unable to query db");
    let mut random_cover_art_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let random_cover_art = types::MusicImageInfo {
            rusicid: row.get(1).unwrap(),
            width: row.get(2).unwrap(),
            height: row.get(3).unwrap(),
            artist: row.get(4).unwrap(),
            artistid: row.get(5).unwrap(),
            album: row.get(6).unwrap(),
            albumid: row.get(7).unwrap(),
            filesize: row.get(8).unwrap(),
            fullpath: row.get(9).unwrap(),
            thumbpath: row.get(10).unwrap(),
            idx: row.get(11).unwrap(),
            page: row.get(12).unwrap(),
            httpthumbpath: row.get(13).unwrap(),
        };

        random_cover_art_vec.push(random_cover_art);
    };

    println!("random_cover_art_vec: {:?}", random_cover_art_vec.clone());
    let json = serde_json::to_string(&random_cover_art_vec).unwrap();

    HttpResponse::Ok().body(json)
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

#[get("/deleteplaylist/{playlistid}")]
pub async fn deleteplaylist(x: web::Path<String>) -> impl Responder {
    let x = x.into_inner();
    let delete_playlist = fragments::delete_playlist(x.clone());
    let json = serde_json::to_string(&delete_playlist).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("addsongtomylikes/{rusicid}")]
pub async fn addsongtomylikes(x: web::Path<String>) -> impl Responder {
    let x = x.into_inner();
    let add_song_to_my_likes = fragments::add_song_to_my_likes(x.clone());
    let json = serde_json::to_string(&add_song_to_my_likes).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("addsongtoplaylist/{playlistid}/{songid}")]
pub async fn addsongtoplaylist(x: web::Path<(String, String)>) -> impl Responder {
    let (playlistid, songid) = x.into_inner();
    println!("playlistid: {}", playlistid.clone());
    println!("songid: {}", songid.clone());
    let add_song_to_playlist = fragments::add_song_to_playlist(playlistid.clone(), songid.clone());
    let json = serde_json::to_string(&add_song_to_playlist).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("getplaylistdata/{playlistid}")]
pub async fn getplaylistdata(x: web::Path<String>) -> impl Responder {
    let playlistid = x.into_inner();
    let get_playlist_data = fragments::get_playlist_data(playlistid.clone());
    let json = serde_json::to_string(&get_playlist_data).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("delsongfromplaylist/{playlistid}/{songid}")]
pub async fn delsongfromplaylist(x: web::Path<(String, String)>) -> impl Responder {
    let (playlistid, songid) = x.into_inner();
    let del_song_from_playlist = fragments::del_song_from_playlist(playlistid.clone(), songid.clone());
    let json = serde_json::to_string(&del_song_from_playlist).unwrap();

    HttpResponse::Ok().body(json)
}