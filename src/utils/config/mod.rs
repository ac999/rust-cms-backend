use std::fs;
use std::io::{Error};

// use crate::other;

// The structure for reading the mysql configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigStruct{
	  pub db_host: String
	, pub db_port: u16
	, pub db_user: String
	, pub db_pass: String
	, pub db_name: String
	, pub ssl_ca : String
	, pub ssl_crt: String
	, pub ssl_key: String
	,
}

// The struct for reading the whole configuration from JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration{
	pub mysql : ConfigStruct
}

pub fn load_config() -> Result<Configuration, Error> {
	let conf = fs::read_to_string("./config/config-dev.json");

	match conf {
          Ok(configuration) => Ok(serde_json::from_str(&configuration)
            .expect("invalid json format"))
        , Err(no_file) => Err(no_file)
        ,
    }
}

