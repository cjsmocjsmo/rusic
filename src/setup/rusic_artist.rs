use rusqlite::Connection;
// use serde::{Deserialize, Serialize};
// use std::env;

pub fn unique_artistids() -> Vec<String> {
    // let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open("./db/rusic.db").expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT DISTINCT artistid FROM music;")
        .unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut artistids: Vec<String> = Vec::new();
    for row in rows {
        artistids.push(row.unwrap());
    }
    // println!("artistids: {:?}", artistids.len());

    artistids
}

pub fn albumids_for_artistid() -> bool {


    false
}