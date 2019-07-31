use mongodb::{Bson, bson, doc};
use mongodb::{Client, ClientOptions, ThreadedClient};
use mongodb::db::ThreadedDatabase;
// pub use crate::db_connection::connection;

use actix_web::{web, App, HttpServer, Result};

use crate::models::{LoginRequest, RecoverRequest, RegisterRequest};

#[macro_use]
extern crate serde_derive;

mod config;
mod database;
mod api;
mod models;

fn get_token() -> String{
    String::from("1t2o3k4e5n6")
}

fn register(info: web::Json<RegisterRequest>
        ) -> Result<String> {
    Ok(format!("Confirmation mail sent to {}.", info.email))
    }

fn login(info: web::Json<LoginRequest>
        ) -> Result<String> {
    Ok(format!("Token {}.", get_token()))
    }

fn recover(info: web::Json<RecoverRequest>
        ) -> Result<String> {
    Ok(format!("Recovery options maybe were sent to {}.", info.email))
    }

fn main(){
	// println!("{:#?}", config::load_config());
	// println!("{:#?}", database::init_connection());
    let _client = database::init_connection()
        .expect("Failed to init connection.");
/*
    let request = models::load_register_request().expect("error @ request struct");

    api::register_request(_client, request);
*/
    
    HttpServer::new(|| App::new()
        .route(
              "/register"
            , web::post().to(register)
            )
        .route(
              "/login"
            , web::post().to(login)
            )
        .route(
              "/recover"
            , web::post().to(recover)
            )
        )
        .bind("127.0.0.1:8080") 
        .unwrap()
        .run()
        .unwrap();

}
