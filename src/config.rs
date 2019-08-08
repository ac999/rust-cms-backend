extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::other;

pub struct ConfigStruct{
	  db_host: String
	, db_port: u16
	, db_user: String
	, db_pass: String
	, db_name: String
	, ca_path: String
	, crt_path: String
	, key_path: String
	,
}

pub fn read_env() -> ConfigStruct{
	dotenv().ok();

	ConfigStruct{
		  db_host : env::var("HOSTNAME")
			.expect("HOSTNAME not found in .env")

		, db_port : other::string_to_u16(
				env::var("PORT")
				.expect("PORT not found in .env")
				)
			
		, db_user : env::var("USER")
			.expect("USER not found in .env")

		, db_pass : env::var("PASS")
			.expect("PASS not found in .env")

		, db_name : env::var("DB_NAME")
			.expect("DB_NAME not found in .env")

		, ca_path : other::file_exist(
			env::var("CA_PATH").expect("CA_PATH not found in .env")
			).expect("Cannot open file.")

		, crt_path : other::file_exist(
			env::var("CRT_PATH").expect("CRT_PATH not found in .env")
			).expect("Cannot open file.")

		, key_path : other::file_exist(
			env::var("KEY_PATH").expect("KEY_PATH not found in .env")
			).expect("Cannot open file.")

		,
	}
}
impl ConfigStruct{
	pub fn get_db_host(&self) -> &String {
		&self.db_host
	}

	pub fn get_db_port(&self) -> &u16 {
		&self.db_port
	}

	pub fn get_db_user(&self) -> &String {
		&self.db_user
	}

	pub fn get_db_pass(&self) -> &String {
		&self.db_pass
	}

	pub fn get_db_name(&self) -> &String {
		&self.db_name
	}

	pub fn get_ssl_ca(&self) -> &String {
		&self.ca_path
	}

	pub fn get_ssl_crt(&self) -> &String{
		&self.crt_path
	}

	pub fn get_ssl_key(&self) -> &String{
		&self.key_path
	}
}