use rusqlite::{Connection, Result};

pub fn create_tables() -> Result<()> {
    // let db_path = env::var("MTV_DB_PATH").expect("MTV_DB_PATH not set");
    let conn = Connection::open("./db/rusic.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS music_images (
            id INTEGER PRIMARY KEY,
            rusicid TEXT NOT NULL,
            width TEXT NOT NULL,
            height TEXT NOT NULL,
            basedir TEXT NOT NULL,
            filename TEXT NOT NULL,
            extension TEXT NOT NULL,
            artist TEXT NOT NULL,
            artistid TEXT NOT NULL,
            album TEXT NOT NULL,
            albumid TEXT NOT NULL,
            filesize TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS music (
            id INTEGER PRIMARY KEY,
            rusicid TEXT NOT NULL,
            imgurl TEXT NOT NULL,
            artist TEXT NOT NULL,
            artistid TEXT NOT NULL,
            album TEXT NOT NULL,
            albumid TEXT NOT NULL,
            song TEXT NOT NULL,
            filenameresults TEXT NOT NULL,
            musicartistresults TEXT NOT NULL,
            musicalbumresults TEXT NOT NULL,
            durationresults TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            extension TEXT NOT NULL,
            idx TEXT NOT NULL,
            page TEXT NOT NULL,
            fsizeresults TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS songsforalbum (
            id INTEGER PRIMARY KEY,

            albumid TEXT NOT NULL,
            songs TEXT NOT NULL,

        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS albumsforartist (
            id INTEGER PRIMARY KEY,
            artistid TEXT NOT NULL,
            albums TEXT NOT NULL,
            index TEXT NOT NULL,
            page TEXT NOT NULL

        )",
        (),
    )?;

    Ok(())
}

// conn.execute(
//     "CREATE TABLE IF NOT EXISTS duration (
//         id INTEGER PRIMARY KEY,
//         rusicid TEXT NOT NULL,
//         duration TEXT NOT NULL,
//         path TEXT NOT NULL
//     )",
//     (),
// )?;

// conn.execute(
//     "CREATE TABLE IF NOT EXISTS fileinfo (
//         id INTEGER PRIMARY KEY,
//         rusicid TEXT NOT NULL,
//         filename TEXT NOT NULL,
//         extension TEXT NOT NULL,
//         filesize TEXT NOT NULL,
//         duration TEXT NOT NULL,
//         idx TEXT NOT NULL,
//         fullpath TEXT NOT NULL,
//         basedir TEXT NOT NULL
//     )",
//     (),
// )?;

// conn.execute(
//     "CREATE TABLE IF NOT EXISTS artistids (
//         id INTEGER PRIMARY KEY,
//         artist TEXT NOT NULL,
//         artistid TEXT NOT NULL,
//         path TEXT NOT NULL
//     )",
//     (),
// )?;

// conn.execute(
//     "CREATE TABLE IF NOT EXISTS albumids (
//         id INTEGER PRIMARY KEY,
//         album TEXT NOT NULL,
//         albumid TEXT NOT NULL,
//         path TEXT NOT NULL
//     )",
//     (),
// )?;
