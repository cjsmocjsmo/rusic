use crate::types;

pub fn frag_artiscount(xx: Vec<types::ArtistCount>) -> String {
    let mut master = Vec::new();
    for x in xx {
        let mut frag = String::new();
        frag.push_str("<div>");
        frag.push_str("<ul class='alphaList'>");
        frag.push_str("<div class='alpha'>");
        frag.push_str(&("<a href='/artist/'".to_owned() + x.alpha.as_str() + ">" + x.alpha.as_str() + "</a>"));
        frag.push_str(&("<h1>".to_owned() + x.alpha.as_str() + "</h1>"));
        frag.push_str(&("<h3>".to_owned() + x.count.to_string().as_str() + "</h3>"));
        frag.push_str("</a></div></ul></div>");
        master.push(frag);

    }

    let master_string = master.join("");

    master_string
}
