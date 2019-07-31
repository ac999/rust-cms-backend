use mongodb::{Bson, bson, doc};
use mongodb::{Client, ClientOptions, ThreadedClient};
use mongodb::db::ThreadedDatabase;
// pub use crate::db_connection::connection;

use actix_web::{web, App, HttpServer, Result};

use crate::models::RegisterRequest;

#[macro_use]
extern crate serde_derive;

mod config;
mod database;
mod api;
mod models;

/*
fn register(info: web::Json<RegisterRequest>
        ) -> Result<mongodb::doc> {
        api::register_request(_client, info)
    }
*/

fn main(){
	// println!("{:#?}", config::load_config());
	// println!("{:#?}", database::init_connection());
    let _client = database::init_connection()
        .expect("Failed to init connection.");

    let request = models::load_register_request().expect("error @ request struct");

    api::register_request(_client, request);

    /*
    HttpServer::new(|| App::new()
        .route(
              "/register"
            , web::post().to(register)
            )
        )
        .bind("127.0.0.1:8080") 
        .unwrap()
        .run()
        .unwrap();
        */
}
