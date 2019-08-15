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
					println!("email not in db");
					false
				}
			}

		}
		).expect("Error @ prep_exec")

}

pub fn query_username(username: String, pool: &mysql::Pool) -> bool {
	let query = format!("SELECT * FROM users WHERE username LIKE \"{}\"", username);
	println!("Query is {}.", query);

	pool.prep_exec( query, () )
		.map(|mut result| {
			match result.next(){
				  Some(r) => {
				  	println!("{:#?}", r);
				  	true
				  }
				, None => {
					println!("username not in db");
					false
				}
			}

		}
		).expect("Error @ prep_exec")

}

pub fn new_user(email: String
	, username: String
	, password: String
	, pool: &mysql::Pool
	) -> bool {
	let query = format!(
		"INSERT INTO users (email, username, password, created_at)
		VALUES (\"{}\", \"{}\", \"{}\", NOW())"
		, email
		, username
		, password
	);

	match pool.prep_exec( query, () ) {
		  Ok(r) => {
		  	println!("{:?}", r);
		  	true
		  }
		, Err(e) => {
			println!("{:?}", e);
			false
		}
	}

}

pub fn get_password(username: String
	, pool: &mysql::Pool
	) -> String {
	let query = format!(
		"SELECT password FROM users WHERE username LIKE \"{}\"", username
		);
	let mut password = String::new();
	for row in pool.prep_exec( query , () ).unwrap() {
		password = mysql::from_row(row.unwrap());
	}
	password
}