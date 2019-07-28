pub use mongodb::{Client, ClientOptions, ThreadedClient};
use std::io::{Error};
// mod config;


// pub mod connection{
    // ca_file = path to file containing trusted server certificates.
    // cl_certificate = path to file containing client certificate.
    // key_file = path to file containing client private key.
    // verify_peer = wheter or not to verify that the server certificate
    // is valid. Unless you're testing out something locally, this should
    // ALWAYS be true.
    pub fn set_ssl_options(ca_file: String, cl_certificate: String, key_file:
                           String, verify_peer: bool) -> ClientOptions {
        ClientOptions::with_ssl(Some(&ca_file), &cl_certificate, &key_file,
        verify_peer)
    }

    pub fn db_client(ip: String, port: u16, options: ClientOptions) ->
                                                Result<Client, mongodb::Error> {
        Client::connect_with_options(&ip, port, options)
    }

    pub fn init_connection() -> Result<Client, Error> {
        let _config = super::config::load_config();

        match _config {
            Ok(c) => {
                let options = set_ssl_options(
                                          c.mongo.db_ca_file
                                        , c.mongo.db_cert_file
                                        , c.mongo.db_key_file
                                        , false
                                    );
                Ok(db_client(c.mongo.db_url, c.mongo.db_port, options).unwrap())
            }
            Err(e) => {
                println!("Module \"database\" cannot init connection w/ error {:#?}", e);
                Err(e)
            },
        }
    }
// }
