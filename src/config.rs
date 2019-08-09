extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::other;

pub struct ConfigStruct{
	  pub db_host: String
	, pub db_port: u16
	, pub db_user: String
	, pub db_pass: String
	, pub db_name: String
	, pub ssl_ca: String
	, pub ssl_crt: String
	, pub ssl_key: String
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

		, ssl_ca : other::file_exist(
			env::var("CA_PATH").expect("CA_PATH not found in .env")
			).expect("Cannot open file.")

		, ssl_crt : other::file_exist(
			env::var("CRT_PATH").expect("CRT_PATH not found in .env")
			).expect("Cannot open file.")

		, ssl_key : other::file_exist(
			env::var("KEY_PATH").expect("KEY_PATH not found in .env")
			).expect("Cannot open file.")

		,
	}
}
