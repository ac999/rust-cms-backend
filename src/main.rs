#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

extern crate regex;

extern crate time;

extern crate reqwest;

extern crate rand;

use actix_web::{web, App, HttpServer, Result};

pub mod models;
pub mod config;
pub mod other;
pub mod database;
pub mod server;
pub mod db_api;

fn main(){

    let addr = "127.0.0.1:8081";

    println!("Server address is: {:?}", &addr);

    println!("{}", server::send_activation("mfc@marius-cristian.com".to_string()).expect("Error"));
    println!("{}", server::send_activation("andreicristian6@pm.me".to_string()).expect("Error"));

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
        .bind(&addr) 
        .expect(format!("Can not bind to {}", &addr).as_str())
        .run()
        .unwrap();

}
