use bcrypt::{hash, verify};

use crate::models;
use crate::database;
use crate::db_api;
use crate::other;

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
	
	if other::mail_check(_info.email.to_string())==false {
		return Ok(format!("{} is not a valid email.",&_info.email))
	}

	if other::password_check(_info.password.to_string())==false{
		return Ok(String::from(
			"Password must be at least of length 8 and the supported symbols are:
	!@#$%^&*()-_`,./<>?:;'+="))
	}

	// if other::register_criteria(_info.email.to_string()
	// 	,_info.password.to_string()) == false {
	// 	return Ok(String::from(
	// 		"Could not register. E-Mail may be used, or password is weak."))
	// }

	let email=&_info.email;
	let username = &_info.username;
	let password = &_info.password;
	let repeat_password = &_info.repeat_password;

	if password == repeat_password{
		match db_api::query_email(email.to_string(), &_my_pool.pool){
			  true => Ok(format!("{} already in database.", email))
			, false => Ok(format!("Can create account with email: {}", email))
		}
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