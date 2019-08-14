#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

extern crate regex;

extern crate time;

use actix_web::{web, App, HttpServer, Result};

pub mod models;
pub mod config;
pub mod other;
pub mod database;
pub mod server;
pub mod db_api;

fn main(){

    HttpServer::new(|| App::new()
        .data(database::MyPool{pool: database::establish_connection()})
        .route(
              "/register"
            , web::post().to(server::register)
            )
        .route(
              "/login"
            , web::post().to(server::login)
            )
        .route(
              "/recover"
            , web::post().to(server::password_reset)
            )
        )
        .bind("127.0.0.1:8081") 
        .expect("Can not bind to 127.0.0.1:8081")
        .run()
        .unwrap();

}
