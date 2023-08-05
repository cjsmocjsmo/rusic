use actix_web::{get, HttpResponse, Responder};
// use actix_web::web::Json;
// use rusqlite::{Connection, Result};
// use serde::Serialize;


use crate::setup;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("RUSIC IS ALIVE!")
}

#[get("/run-stp")]
async fn rusic_setup() -> impl Responder {
    let _env_vars = crate::envvars::set_env_vars();

        let _stp_result = setup::run_setup();
        println!("\nSetup is Complete! \nStarting Server\n");
        // if setup_result {
        //     let _start_server = crate::server::fire_server_main();
        // };
    HttpResponse::Ok().body("SETUP COMPLETE!")
}