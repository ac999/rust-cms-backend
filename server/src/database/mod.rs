pub use mongodb::{Client, ClientOptions, ThreadedClient};

    // ca_file = path to file containing trusted server certificates.
    // cl_certificate = path to file containing client certificate.
    // key_file = path to file containing client private key.
    // verify_peer = wheter or not to verify that the server certificate
    // is valid. Unless you're testing out something locally, this should
    // ALWAYS be true.
    pub fn set_ssl_options(
          ca_file:          String
        , cl_certificate:   String
        , key_file:         String
        , verify_peer:      bool
        ) -> ClientOptions {
        ClientOptions::with_ssl(Some(&ca_file), &cl_certificate, &key_file,
        verify_peer)
    }

    pub fn db_client(ip: String, port: u16, options: ClientOptions) ->
                                            Result<Client, mongodb::Error> {
        Client::connect_with_options(&ip, port, options)
    }

    pub fn init_connection() -> Result<Client, mongodb::error::Error> {
        let _config = super::config::load_config()
        .expect("database::init_connection(): error loading config");
        
        let options = set_ssl_options(
                                          _config.mongo.db_ca_file
                                        , _config.mongo.db_cert_file
                                        , _config.mongo.db_key_file
                                        , false
                                    );

        let _client = db_client(
              _config.mongo.db_url
            , _config.mongo.db_port
            , options
            );

        match _client{
            Ok(c) => Ok(c),
            Err(e) => Err(e),
    }
}
