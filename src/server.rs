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

pub fn send_mail(
	  from: String
	, to:String
	, subject: String
	, body: String
	) -> Result<(()), reqwest::Error> {
	let form = multipart::Form::new()
        .text("from", from)
        .text("to", to)
        .text("subject", subject)
        .text("text", body);

    let res = reqwest::Client::new()
        .post("https://api.mailgun.net/v3/sandboxfc282248ae5c4e9b934bd4715c2fedf7.mailgun.org/messages")
        .basic_auth("api", Some("658d7b3328a09c4ab9eb249c258575ec-19f318b0-8c6caf5d"))
        .multipart(form)
        .send()?
        .text()?;
        Ok(())
}

pub fn register(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::RegisterRequest>
        ) -> Result<String> {
	
	if other::mail_check(_info.email.to_string())==false {
		return Ok(format!("{} is not a valid email.",&_info.email))
	}

	if other::password_check(_info.password.to_string())==false{
		return Ok(String::from(
			"Password must be at least of length 8."))
	}

	let email=&_info.email;
	let username = &_info.username;
	let password = &_info.password;
	let repeat_password = &_info.repeat_password;

	if db_api::query_username(username.to_string(), &_my_pool.pool) {
		return Ok(format!("{} already in database.", username))
	}

	if password == repeat_password{
		match db_api::query_email(email.to_string(), &_my_pool.pool){
			  true => Ok(format!("{} already in database.", email))
			, false => {
				let password = hash_password(password.to_string());
				match db_api::new_user(
					  email.to_string()
					, username.to_string()
					, password
					, &_my_pool.pool){
					  true => {
					  	// send_activation(email.to_string());
					  	Ok(String::from
						("Account created. Check mail for confirmation."))
					  }
					, false => Ok(String::from
						("Error creating account. Check again later."))
				}
			} 
		}
	}
	else {
		Ok(String::from("Passwords must match."))
	}
    
}

pub fn login(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::LoginRequest>
        ) -> Result<String> {
	let username = &_info.username;
	if db_api::query_username(username.to_string(), &_my_pool.pool) == false {
		return Ok(String::from("Login failed. Maybe recheck username."));
	}
	let password = &_info.password;
	let pwdhash=db_api::get_password(username.to_string(), &_my_pool.pool);
	if verify_hash(password.to_string(), pwdhash) == false{
		return Ok(String::from("Login failed. Maybe recheck password."));
	}
	Ok(get_token(username.to_string(), password.to_string()))
    
}

pub fn password_reset(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::PasswordResetRequest>
        ) -> String {
	
	String::from("test")
    
}

pub fn get_token(username: String, password: String) -> String {
	hash_password(format!("zoolx+{}+zoolx+{}+zoolx", username, password))
}