use bcrypt::{hash, verify};

use crate::models::{self, ServerResponse};
use crate::database;
use crate::db_api;
use crate::other;
use crate::mail;



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

fn create_activation(text: &String) -> String {
	let mut activation: String = other::random_string_generator();
	activation.push_str(&text);
	hash(activation, 4).expect("activation hashing error.")
}

fn send_activation(mail: &String) {
	let activation = create_activation(mail) ;
	let from = String::from("no-reply@test.zoolx.ro");
	let to = String::from(mail);
	let subject = String::from("Activation") ;
	let body = format!("127.0.0.1:8081/set_password/{}",activation);
	mail::send_mail(
		  from
		, to
		, subject
		, body
		);
}

fn create_response(status_text: String,
	message_text: String) -> Result<web::Json<models::ServerResponse>> {
	Ok(
		web::Json( ServerResponse {
			  status: status_text
			, message: message_text
			,
			}
		)
	)
}

pub fn register(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::RegisterRequest>
        ) -> Result<web::Json<models::ServerResponse>> {
	
	if other::mail_check(_info.email.to_string())==false {
		return create_response(String::from("Error")
				, format!("{} is not a valid email.",&_info.email)
				// get message from config json, but how ???
		)
	}

	let email=&_info.email;
	let username = &_info.username;

	if db_api::query_username(username.to_string(), &_my_pool.pool) {
		return create_response(String::from("Error")
				, format!("{} already in database.", username)
				// get message from config json, but how ???
			)
	}

	if db_api::query_email(email.to_string(), &_my_pool.pool){
		return create_response(String::from("Error")
				, format!("{} already in database.", email)
				// get message from config json, but how ???	
		)
	}

	send_activation(email.to_string());

	
	create_response(String::from("Error")
		, format!("Next step was sent to {}", email)
		// get message from config json, but how ???
	)
}

pub fn set_password(_my_pool: web::Data<database::MyPool>
	, _info: web::Json<models::SetPasswordRequest>
	) -> Result<web::Json<models::ServerResponse>> {

	if other::password_check(_info.password.to_string())==false{
		return Ok(
			web::Json( ServerResponse {
				  status: String::from("Error")
				, message: String::from(
					"Password must be at least of length 8.")
				// get message from config json, but how ???
				,
				}
			)
		)
	}

	// recheck if user & email is valid
	// to avoid certain types of attacks.
	// to do...

	// check if password is in password history
	// to do...

	let email = &_info.email;
	let username = &_info.username;
	let password = &_info.password;
	let rpassword = &_info.rpassword;

	if password == rpassword {
		let password = hash_password(password.to_string());
		match db_api::new_user(
			  email.to_string()
			, username.to_string()
			, password
			, &_my_pool.pool){
					true => {
						Ok(
							web::Json( ServerResponse {
								  status: String::from("Ok")
								, message: String::from(
									"Password set. You can now login.")
							// get message from config json, but how ???
									,
								}
							)
						)
				  }
				, false => {
					Ok(
						web::Json( ServerResponse {
							  status: String::from("Error")
							, message: String::from(
								"Error creating account. Check again later.")
						// get message from config json, but how ???
								,
							}
						)
					)
				}
					
			}


	}
	else {
		Ok(
				web::Json( ServerResponse {
					  status: String::from("Ok")
					, message: String::from("Passwords are not the same.")
					// get message from config json, but how ???
					,
					}
				)
			)
	}
}




pub fn login(_my_pool: web::Data<database::MyPool>
    , _info: web::Json<models::LoginRequest>
        ) -> Result<web::Json<models::ServerResponse>> {
	let username = &_info.username;
	if db_api::query_username(username.to_string(), &_my_pool.pool) == false {
		Ok(
		web::Json( ServerResponse {
			  status: String::from("Ok")
			, message: String::from("Login failed. Maybe recheck username.")
			// get message from config json, but how ???
			,
			}
		)
	);

		// return Ok(String::from("Login failed. Maybe recheck username."));
	}
	let password = &_info.password;
	let pwdhash=db_api::get_password(username.to_string(), &_my_pool.pool);

	if verify_hash(password.to_string(), pwdhash) == false {

		return Ok(
		web::Json( ServerResponse {
			  status: String::from("Ok")
			, message: String::from("Login failed. Maybe recheck password.")
			// get message from config json, but how ???
			,
			}
		)
	);
	}

	Ok(
		web::Json( ServerResponse {
			  status: String::from("Ok")
			, message: String::from("token")
			// get message from config json, but how ???
			,
			}
		)
	)

	
	}

	// Ok(get_token(username.to_string(), password.to_string()))
    


// pub fn password_reset(_my_pool: web::Data<database::MyPool>
//     , _info: web::Json<models::PasswordResetRequest>
//         ) -> Result<web::Json<models::ServerResponse>> {
	
	
    
// }

pub fn get_token(username: String, password: String) -> String {
	hash_password(format!("zoolx+{}+zoolx+{}+zoolx", username, password))
}