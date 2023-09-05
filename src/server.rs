use actix_files as fs;
use actix_web::{App, HttpServer};
use std::env;

pub mod server_functions;

#[actix_web::main]
pub async fn fire_server_main() -> std::io::Result<()> {
    let img_path = env::var("RUSIC_THUMBS").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(server_functions::hello)
            .service(server_functions::echo)
            .service(server_functions::artistalpha)
            .service(server_functions::albumalpha)



            // .service(crate::server::server_functions::wheeloftime)
            .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
        }
    )
    .bind(("192.168.0.26", 8080))?
    .run()
    .await
}
