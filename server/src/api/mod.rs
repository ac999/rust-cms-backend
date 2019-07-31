use mongodb::db::{Database, ThreadedDatabase};
use mongodb::{Client, ThreadedClient};
use mongodb::{bson, doc};

use bcrypt::{DEFAULT_COST, hash, verify};

pub fn register_request(_client: Client
		, request_struct: super::models::RegisterRequest
		) {

	// search for username/email in database first

	if request_struct.password == request_struct.repeat_password {
		let hashed = hash(&request_struct.password, 5).expect("hashing error");

			let _doc = doc!{
				  "email": request_struct.email
				, "username": request_struct.username
				, "password": hashed
			};

	}
	else {
		println!("Passwords do not match.");
	}

}