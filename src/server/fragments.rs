use crate::types;

pub fn frag_artiscount(xx: Vec<types::ArtistCount>) -> String {
    let mut master = Vec::new();
    let foo = "<div><ul class='alphaList'>".to_string();
    master.push(foo);
    for x in xx {
        let mut frag = String::new();
        frag.push_str(&("<div class='alpha'><a href='/artist/".to_owned() + x.alpha.as_str() + "'>"));
        frag.push_str(&("<h1>".to_owned() + x.alpha.as_str() + "</h1>"));
        frag.push_str(&("<h3>".to_owned() + x.count.to_string().as_str() + "</h3>"));
        frag.push_str("</a></div>");
        master.push(frag);

    };

    let end = "</ul></div>".to_string();
    master.push(end);

    let master_string = master.join("");

    master_string
}

pub fn frag_artist_for_alpha(xx: Vec<types::ArtArtidInfo>) -> String {
    let mut master = Vec::new();
    let start = "<div>".to_owned();
    master.push(start);
    for x in xx {
        let mut frag = String::new();
        frag.push_str("<button    >");
        frag.push_str(&x.artist);
        frag.push_str("</button>");
        master.push(frag);
    };
    let end = "</div>".to_string();
    master.push(end);

    let master_string = master.join("");


    master_string
}

pub fn frag_albumcount(xx: Vec<types::AlbumCount>) -> String {
    let mut master = Vec::new();
    let foo = "<div><ul class='alphaList'>".to_string();
    master.push(foo);
    for x in xx {
        let mut frag = String::new();
        frag.push_str(&("<div class='alpha'><a href='/album/".to_owned() + x.alpha.as_str() + "'>"));
        frag.push_str(&("<h1>".to_owned() + x.alpha.as_str() + "</h1>"));
        frag.push_str(&("<h3>".to_owned() + x.count.to_string().as_str() + "</h3>"));
        frag.push_str("</a></div>");
        master.push(frag);

    };

    let end = "</ul></div>".to_string();
    master.push(end);

    let master_string = master.join("");

    master_string
}


