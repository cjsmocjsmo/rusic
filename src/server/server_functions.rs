use actix_web::{get, post, HttpResponse, Responder};
// use actix_web::web::Json;
// use rusqlite::{Connection, Result};
// use serde::Serialize;
// use anyhow::Error;

#[get("/test")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Rusic Web Server is running!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

