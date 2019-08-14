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

pub fn register(my_pool: web::Data<database::MyPool>
    , info: web::Json<models::RegisterRequest>
        ) -> Result<String> {
	
	let email = &info.email;
	let username = &info.username;
	let password = &info.password;
	let repeat_password = &info.repeat_password;

	if password == repeat_password{
		println!("query_email returned {:?}.",db_api::query_email(email.to_string(), &my_pool.pool));
		Ok(format!("{}+{}+{}",email, username, password))
	}
	else {
		// To implement
		Ok(String::from("Passwords must match."))
	}
    
}

pub fn login(my_pool: web::Data<database::MyPool>
    , info: web::Json<models::LoginRequest>
        ) -> String {
	
	String::from("test")
    
}

pub fn password_reset(my_pool: web::Data<database::MyPool>
    , info: web::Json<models::PasswordResetRequest>
        ) -> String {
	
	String::from("test")
    
}