use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumSongs {
    pub page: i32,
    pub albumid: String,
    pub rusicids: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistAlbums {
    pub page: i32,
    pub artistid: String,
    pub albums: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicImageInfo {
    pub rusicid: String,
    pub width: String,
    pub height: String,
    pub artist: String,
    pub artistid: String,
    pub album: String,
    pub albumid: String,
    pub filesize: String,
    pub fullpath: String,
    pub thumbpath: String,
    pub idx: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicInfo {
    pub rusicid: String,
    pub imgurl: String,
    pub artist: String,
    pub artistid: String,
    pub album: String,
    pub albumid: String,
    pub song: String,
    pub fullpath: String,
    pub extension: String,
    pub idx: String,
    pub page: String,
    pub fsizeresults: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstLetterInfo {
    pub rusicid: String,
    pub artist: String,
    pub album: String,
    pub artistid: String,
    pub albumid: String,
    pub artist_first_letter: String,
    pub album_first_letter: String,
}