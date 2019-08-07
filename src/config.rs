extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::other;

pub struct ConfigStruct{
	  db_url: String
	, ca_path: String
	, crt_path: String
	, key_path: String
	,
}

pub fn read_env() -> ConfigStruct{
	dotenv().ok();

	ConfigStruct{
		  db_url : env::var("DATABASE_URL")
			.expect("DATABASE_URL not found in .env")

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

pub fn get_db_url(config: ConfigStruct) -> String {
	String::new()
}

pub fn get_ca(config: ConfigStruct) -> String {
	String::new()
}

pub fn get_crt(config: ConfigStruct) -> String{
	String::new()
}

pub fn get_key(config: ConfigStruct) -> String{
	String::new()
}