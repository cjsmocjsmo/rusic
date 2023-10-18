use actix_files as fs;
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use std::env;

pub mod server_functions;
pub mod fragments;

#[actix_web::main]
pub async fn rusic_server_main() -> std::io::Result<()> {
    let img_path = env::var("RUSIC_THUMBS").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(server_functions::hello)
            .service(server_functions::artistcount)
            .service(server_functions::albumcount)
            .service(server_functions::artistalpha)
            .service(server_functions::albumalpha)
            .service(server_functions::albforart)
            .service(server_functions::songsforalbum)
            .service(server_functions::createemptyplaylist)
            .service(server_functions::createrandomplaylist)
            .service(server_functions::allplaylists)
            .service(server_functions::randomcoverart)
            .service(server_functions::deleteplaylist)
            .service(server_functions::addsongtomylikes)
            .service(server_functions::addsongtoplaylist)
            .service(server_functions::getplaylistdata)
            .service(server_functions::delsongfromplaylist)

            // .service(crate::server::server_functions::wheeloftime)
            // .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
            .service(fs::Files::new("/thumbs", img_path.clone()))
            .service(fs::Files::new("/Music", "/usr/share/rusic/rusic/Music/Music".to_string()).show_files_listing())
            // .service(fs::Files::new("/Music", "/home/pipi/Music".to_string()))
        }
    )
    .bind(("192.168.0.26", 8080))?
    // .bind(("192.168.0.91", 8080))?
    .run()
    .await
}
