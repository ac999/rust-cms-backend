#[doc = "Serde deserialization of the configuration file.
For more details please check https://serde.rs/data-model.html and the config.json file"]
#[macro_use]
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
/// The structure for reading the mongodb configuration
struct Mongo {
	/// connection url to the database
	/// example: mongo - for local, mongodb://url.at.database.hosted.com
      dbUrl			: string
    /// the database name
    /// example: admin; zoolx; database1
    , dbName		: string
    /// Certificate Authority
    , dbCaFile		: string
    /// Priv key for the db cert
    , dbKeyFile		: string
    /// Db public certificate
    , dbCertFile	: string
}

#[derive(Debug, Deserialize)]
/// The structure for reading the whole configuration json
struct Configuration{
	///mongodb
	mongo		: Mongo
}
