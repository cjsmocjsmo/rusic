use rusqlite::Connection;
// use serde::{Deserialize, Serialize};
use std::env;

pub async fn unique_artistids() -> bool {
    let db_path = env::var("ATS_DB_PATH").expect("ATS_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn
        .prepare("SELECT DISTINCT artistid FROM music;")
        .unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut artistids: Vec<String> = Vec::new();
    for row in rows {
        artistids.push(row.unwrap());
    }
    println!("artistids: {:#?}", artistids);

    // let mut rows = stmt.query(&[&qemail]).expect("Unable to query db");
    // let mut exists = false;
    // while let Some(row) = rows.next().expect("Unable to get next row") {
    //     let acct: String = row.get(0).expect("Unable to get acct");

    //     if acct == qemail {
    //         exists = true;
    //     };
    // }

    false
}