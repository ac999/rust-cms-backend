#[doc = "Serde deserialization of the configuration file.
For more details please check https://serde.rs/data-model.html and the config.json file"]
#[macro_use]
use std::fs;
#[macro_use]
use serde::{Serialize, Deserialize};
/// The structure for reading the mongodb configuration
#[derive(Serialize, Deserialize, Debug)]
pub struct Mongo {
      dbUrl			: String
    , dbName		: String
    , dbCaFile		: String
    , dbKeyFile		: String
    , dbCertFile	: String
    ,
}

/// The structure for reading the whole configuration json
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration{
	///mongodb
	  mongo		: Mongo
	,
}


pub fn load_config(){
	let conf = fs::read_to_string("./config-dev.json").expect(
		"File Does not Exist");

	let conf_deserialized :Mongo = serde_json::from_str(&conf).unwrap();

	println!("deserialized = {:?}", conf_deserialized);

}
