// use actix_web::{get, post, HttpResponse, Responder};
// // use actix_web::web::Json;
// // use rusqlite::{Connection, Result};
// // use serde::Serialize;

// pub mod envvars;
// use setup;


// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("RUSIC IS ALIVE!")
// }

// #[get("/setup")]
// async fn setup() -> impl Responder {
//     let _env_vars = crate::envvars::set_env_vars();

//         let setup_result = setup::run_setup();
//         // println!("\nSetup is Complete! \nStarting Server\n{:?} {}", duration, setup_result);
//         if setup_result {
//             let _start_server = crate::server::fire_server_main();
//         };
//     HttpResponse::Ok().body("SETUP COMPLETE!")
// }