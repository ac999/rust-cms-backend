// use crate::database;


use time;

pub fn query_email(email: String, pool: &mysql::Pool) -> bool {
	let query = format!("SELECT * FROM users WHERE email LIKE \"{}\"", email);
	println!("Query is {}.", query);

	pool.prep_exec( query, () )
		.map(|mut result| {
			match result.next(){
				  Some(r) => {
				  	println!("{:#?}", r);
				  	true
				  }
				, None => {
					println!("No entry in db");
					false
				}
			}

		}
		).expect("Error @ prep_exec")

}

pub fn query_user(user: String, pool: &mysql::Pool) -> bool {
	let query = format!("SELECT * FROM users WHERE user LIKE \"{}\"", user);
	println!("Query is {}.", query);

	pool.prep_exec( query, () )
		.map(|mut result| {
			match result.next(){
				  Some(r) => {
				  	println!("{:#?}", r);
				  	true
				  }
				, None => {
					println!("No entry in db");
					false
				}
			}

		}
		).expect("Error @ prep_exec")

}

pub fn new_user(email: String
	, user: String
	, password: String
	) -> bool {
	let query = format!(
		"INSERT INTO users (email, user, password, created_at)
		VALUES ({}, {}, {}, NOW())"
		, email
		, user
		, password
	);

	true

}