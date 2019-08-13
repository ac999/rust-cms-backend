#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

extern crate regex;

use actix_web::{web, App, HttpServer, Result};

pub mod models;
pub mod config;
pub mod other;
pub mod database;
pub mod server;

fn main(){

    HttpServer::new(|| App::new()
        .data(database::MyPool{pool: database::establish_connection()})
        .route(
              "/register"
            , web::post().to(server::register)
            )
        )
        .bind("127.0.0.1:8081") 
        .expect("Can not bind to 127.0.0.1:8081")
        .run()
        .unwrap();
    // let _pool = database::establish_connection();

}
