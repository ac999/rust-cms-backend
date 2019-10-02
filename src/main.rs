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
pub mod mail;
pub mod test;

fn main(){

    let addr = "127.0.0.1:8081";

    println!("Server address is: {:?}", &addr);

    // println!("{}",test::create_and_send_activation());
    // println!("{}",test::create_and_send_recovery());

    HttpServer::new(|| App::new()
        .data(
            database::MyPool {
                pool: database::establish_connection() 
            }
        )
        .route(
              "/register"
            , web::post().to(server::register)
        )
        .route(
              "/set-password"
            , web::post().to(server::set_password)
        )
        .route(
              "/login"
            , web::post().to(server::login)
        )
        // .route(
        //       "/recover"
        //     , web::post().to(server::password_reset)
        // )
    )
    .bind(&addr) 
    .expect(format!("Can not bind to {}", &addr).as_str())
    .run()
    .unwrap();

}
