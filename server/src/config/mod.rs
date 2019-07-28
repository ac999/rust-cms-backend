#[doc = "Serde deserialization of the configuration file.
For more details please check https://serde.rs/data-model.html and the
config.json file"]

use std::fs;
use std::io::{Error};

/// The structure for reading the mongodb configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct Mongo {
      db_url		: String
    , db_name		: String
    , db_ca_file	: String
    , db_key_file	: String
    , db_cert_file	: String
    ,
}

/// The structure for reading the whole configuration json
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration{
	///mongodb
	  mongo		: Mongo
	,
}


pub fn load_config() -> Result<Configuration, Error> {
	let conf = fs::read_to_string("./config/config-dev.json");

    match conf {
        Ok(configuration) => Ok(serde_json::from_str(
            &configuration)
            .unwrap()),
        Err(no_file) => return Err(no_file),
    }
}
