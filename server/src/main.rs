use mongodb::{Bson, bson, doc};
use mongodb::{Client, ClientOptions, ThreadedClient};
use mongodb::db::ThreadedDatabase;
// pub use crate::db_connection::connection;

use actix_web::{web, App, HttpServer, Result};

//test
use crate::models::{LoginRequest, RecoverRequest, RegisterRequest};
use bcrypt::{hash};

#[macro_use]
extern crate serde_derive;

mod config;
mod database;
mod api;
mod models;

fn get_token() -> String{
    String::from("1t2o3k4e5n6")
}

struct MyClient{
    client: mongodb::Client
    ,
}

fn check_register_info(my_client: web::Data<MyClient>
    ,   info: web::Json<RegisterRequest>) -> Result<String> {

    let coll = my_client.client.db("zoolx_test").collection("users");

    let _doc = doc!{
        "$or" : [
                  { "username" : &info.username }
                , { "email" : &info.email }
                ,
        ]
    };

    let mut cursor = coll.find(Some(_doc), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    match item{
          Some(Ok(doc)) => Ok(String::from("Entry already exists."))
        , Some(Err(_)) => Ok(String::from("Failed to get \"find\" from server."))
        , None =>{
            let new_user = doc!{
                  "username":   &info.username
                , "email"   :   &info.email
                , "password":   hash(&info.password, 5).expect("hashing error")
                ,
            };
            coll.insert_one(new_user, None).ok().expect("Failed to create user.");
            Ok(String::from("Created new user. Confirm mail."))
        }
        ,
    }
}

fn register(my_client: web::Data<MyClient>
    , info: web::Json<RegisterRequest>
        ) -> Result<String> {
    let coll = my_client.client.db("zoolx_test").collection("users");
    Ok(format!("Confirmation mail sent to {}.", info.email))
    }

fn login(my_client: web::Data<MyClient>
    , info: web::Json<LoginRequest>
        ) -> Result<String> {
    Ok(format!("Token {}.", get_token()))
    }

fn recover(my_client: web::Data<MyClient>
    , info: web::Json<RecoverRequest>
        ) -> Result<String> {
    Ok(format!("Recovery options maybe were sent to {}.", info.email))
    }

fn main(){
	// println!("{:#?}", config::load_config());
	// println!("{:#?}", database::init_connection());
    /*let _client = database::init_connection()
        .expect("Failed to init connection.");*/
    
    HttpServer::new(|| App::new()
        .data(MyClient{ client: database::init_connection()
            .expect("Failed to init connection.") })
        .route(
              "/register"
            , web::post().to(check_register_info)
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
        .expect("Can not bind to 127.0.0.1:8080")
        .run()
        .unwrap();

}
