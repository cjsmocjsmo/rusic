use rusqlite::Connection;
// use serde::{Deserialize, Serialize};

pub fn unique_albumids() -> Vec<String> {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT DISTINCT albumid FROM music;")
        .unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut albumids: Vec<String> = Vec::new();
    for row in rows {
        albumids.push(row.unwrap());
    }
    println!("albumids: {:?}", albumids.len());

    albumids
}

pub fn songids_for_albumid(xid: String) -> (String, Vec<String>) {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT rusicid FROM music WHERE albumid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&xid]).expect("Unable to query db");

    let mut songids: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        songids.push(row.get(0).unwrap());
    }


    // let mut songids: Vec<String> = Vec::new();
    // for row in rows {
    //     songids.push(row.unwrap());
    // }
    // println!("albumids: {:?}", songids.len());

    (xid, songids)
}