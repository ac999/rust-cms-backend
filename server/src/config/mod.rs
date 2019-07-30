#[doc = "Serde deserialization of the configuration file.
For more details please check https://serde.rs/data-model.html and the
config.json file"]

use std::fs;
use std::io::{Error};

/// The structure for reading the mongodb configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct Mongo {
      pub db_url		: String
    , pub db_port       : u16
    , pub db_name		: String
    , pub db_ca_file	: String
    , pub db_cert_file	: String
    , pub db_key_file   : String
    ,
}

/// The structure for reading the whole configuration json
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration{
	///mongodb
	  pub mongo		: Mongo
	,
}


pub fn load_config() -> Result<Configuration, Error> {
	let conf = fs::read_to_string("./config/config-dev.json");

    match conf {
          Ok(configuration) => Ok(serde_json::from_str(&configuration)
            .expect("invalid json format"))
        , Err(no_file) => Err(no_file),
    }
}
