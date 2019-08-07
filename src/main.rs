#[macro_use]
extern crate mysql;
extern crate dotenv;

// use actix_web::{web, App, HttpServer, Result};

pub mod models;
pub mod config;
pub mod other;
pub mod database;

fn main(){

    // HttpServer::new(|| App::new()
    //     .data(MyPool{ pool: establish_connection() })
    //     .route(
    //           "/register"
    //         , web::post().to(register)
    //         )
    //     )
    //     .bind("127.0.0.1:8081") 
    //     .expect("Can not bind to 127.0.0.1:8081")
    //     .run()
    //     .unwrap();
    let _config = config::read_env();
    let pool = database::establish_connection(_config);
}
