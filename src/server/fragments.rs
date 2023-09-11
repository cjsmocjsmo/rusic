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
