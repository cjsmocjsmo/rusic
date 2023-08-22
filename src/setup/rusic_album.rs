use rusqlite::Connection;
// use serde::{Deserialize, Serialize};

pub fn unique_albumids() -> bool {
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
    println!("albumids: {:#?}", albumids.len());

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