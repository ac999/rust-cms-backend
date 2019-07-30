use mongodb::db::{Database, ThreadedDatabase};
use mongodb::{bson, doc};

use std::fs;
use std::io::{Error};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn registerRequest(db: Database
		, requestStruct: super::data::RegisterRequest
		) {

	println!("{:?}", requestStruct);

	// search for username/email in database first

	let mut hasher = DefaultHasher::new();
	requestStruct.password.hash(&mut hasher);

	hasher.finish().hash(&mut hasher);

	let password = hasher.finish();

	let mut hasher2 = DefaultHasher::new();
	requestStruct.repeatPassword.hash(&mut hasher2);

	hasher2.finish().hash(&mut hasher2);

	let repeat_password = hasher2.finish();

	if password == repeat_password {
		println!("Passwords match");
	}
	else {
		println!("Passwords do not match");
	}


	println!("password hash: {:?}", password);
	println!("repeated password hash: {:?}", repeat_password);

	let _doc = doc!{
		  "email": requestStruct.email
		, "username": requestStruct.username
		, "password": password
	};

	// println!("{:?}", _doc);


	// let coll = db.collection("users");



}