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