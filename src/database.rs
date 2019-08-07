#[macro_use]
extern crate mysql;

use mysql as my;

pub fn establish_connection(database_url: String) -> my::Pool {
	my::Pool::new(&database_url)
		.expect(&format!("Could not connect to {}", database_url))
}