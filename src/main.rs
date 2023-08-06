// use actix_files as fs;
use actix_web::{App, HttpServer};
// use std::env;

pub mod server;
pub mod setup;
pub mod envvars;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _env_vars = envvars::set_env_vars();

    HttpServer::new(move || {
        App::new()
            .service(server::hello)
            .service(server::rusic_setup)




            // .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
        }
    )
    // .bind(("192.168.0.26", 8080))?
    .bind(("192.168.0.90", 8080))?
    .run()
    .await
}
