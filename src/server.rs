use bcrypt::{hash, verify};

use crate::models;
use crate::database;
use crate::db_api;
use crate::other;


use time;

use std::io::Error;

use actix_web::{web, Result};

use reqwest::multipart;

fn hash_password(password: String) -> String {
	hash(password, 5).expect("Hashing error.")
}

fn verify_hash(password: String, password_hash: String) -> bool {
	verify(password, &password_hash).expect("Hash verify error.")
}

pub fn create_activation(text: &String) -> String {
	let mut activation: String = other::random_string_generator();
	activation.push_str(&text);
	hash(activation, 4).expect("activation hashing error.")
}


pub fn register(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::RegisterRequest>
        ) -> Result<web::Json<models::ServerResponse>> {
	
	if other::mail_check(_info.email.to_string())==false {
		return Ok(
			web::Json( ServerResponse {
				  status: String::from("Error")
				, message: format!("{} is not a valid email.",&_info.email)
				// get message from config json, but how ???
				,
				}
			)
		)
	}

	let email=&_info.email;
	let username = &_info.username;

	if db_api::query_username(username.to_string(), &_my_pool.pool) {
		return Ok(
			web::Json( ServerResponse {
				  status: String::From("Error")
				, message: format!("{} already in database.", username)
				// get message from config json, but how ???
				,
			})
			)
	}

	if query_email(email.to_string(), &_my_pool.pool){
		return Ok(
			web::Json( ServerResponse {
				  status: String::From("Error")
				, message: format!("{} already in database.", email)
				// get message from config json, but how ???
				,
				}
			)
		)
			
				// get message from config json, but how ???
		// return Ok(format!("{} already in database.", email))
	}

	
	Ok(
		web::Json( ServerResponse {
			  status: String::From("Ok")
			, message: format!("Next step was sent to {}", email)
			// get message from config json, but how ???
			,
			}
		)
	)    
}

// password_set

// if other::password_check(_info.password.to_string())==false{
// 		return Ok(
// 			web::Json( ServerResponse {
// 				  status: String::from("Error")
// 				, message: String::from(
// 					"Password must be at least of length 8.")
// 				// get message from config json, but how ???
// 				,
// 				}
// 			)
// 		)
// 		// return Ok(String::from(
// 		// 	"Password must be at least of length 8."))
// 	}

// if password == repeat_password{
// 				let password = hash_password(password.to_string());
// 				match db_api::new_user(
// 					  email.to_string()
// 					, username.to_string()
// 					, password
// 					, &_my_pool.pool){
// 					  true => {
// 					  	// send_activation(email.to_string());
// 					  	// Ok(String::from
// 						// ("Account created. Check mail for confirmation."))
// 					  }
// 					, false => {
// 						// Ok(String::from
// 						// ("Error creating account. Check again later."))
// 					}
						
// 				}
// 	}

pub fn login(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::LoginRequest>
        ) -> Result<web::Json<models::ServerResponse>> {
	let username = &_info.username;
	if db_api::query_username(username.to_string(), &_my_pool.pool) == false {
		// return Ok(String::from("Login failed. Maybe recheck username."));
	}
	let password = &_info.password;
	let pwdhash=db_api::get_password(username.to_string(), &_my_pool.pool);
	if verify_hash(password.to_string(), pwdhash) == false{
		return Ok(String::from("Login failed. Maybe recheck password."));
	}
	// Ok(get_token(username.to_string(), password.to_string()))
    
}

pub fn password_reset(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::PasswordResetRequest>
        ) -> Result<web::Json<models::ServerResponse>> {
	
	String::from("test")
    
}

pub fn get_token(username: String, password: String) -> String {
	hash_password(format!("zoolx+{}+zoolx+{}+zoolx", username, password))
}