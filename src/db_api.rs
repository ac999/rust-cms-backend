use crate::database;


use time;

pub fn query_email(email: String, pool: &mysql::Pool) -> bool {
	let query = format!("SELECT * FROM users WHERE email LIKE \"{}\"", email);
	println!("Query is {}.", query);


	// if email not in db, server crashes.
	pool.prep_exec( query, () )
		.map(|mut result| {
			let row = result.next()
			.unwrap()
			.unwrap();
			println!("MySQL returned:\n{:?}", row);
		}
		).expect("Error @ prep_exec 3");

	true
}