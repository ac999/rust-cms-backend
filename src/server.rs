use bcrypt::{hash, verify};

use crate::models;
use crate::database;
use crate::db_api;

use std::io::Error;

use actix_web::{web, Result};

fn hash_password(password: String) -> String {
	hash(password, 5).expect("Hashing error.")
}

fn verify_hash(password: String, password_hash: String) -> bool {
	verify(password, &password_hash).expect("Hash verify error.")
}

pub fn register(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::RegisterRequest>
        ) -> Result<String> {
	
	let email = &_info.email;
	let username = &_info.username;
	let password = &_info.password;
	let repeat_password = &_info.repeat_password;

	if password == repeat_password{
		println!("query_email returned {:?}.",db_api::query_email(email.to_string(), &_my_pool.pool));
		Ok(format!("{}+{}+{}",email, username, password))
	}
	else {
		// To implement
		Ok(String::from("Passwords must match."))
	}
    
}

pub fn login(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::LoginRequest>
        ) -> String {
	
	String::from("test")
    
}

pub fn password_reset(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::PasswordResetRequest>
        ) -> String {
	
	String::from("test")
    
}