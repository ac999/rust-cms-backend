pub use mongodb::{Client, ClientOptions, ThreadedClient};

pub mod connection{
    // ca_file = path to file containing trusted server certificates.
    // cl_certificate = path to file containing client certificate.
    // key_file = path to file containing client private key.
    // verify_peer = wheter or not to verify that the server certificate
    // is valid. Unless you're testing out something locally, this should
    // ALWAYS be true.
    pub fn set_ssl_options(ca_file: String, cl_certificate: String, key_file:
                           String, verify_peer: bool) -> ClientOptions {
        ClientOptions::with_ssl(Some(ca_file), client_certificate, key_file,
        verify_peer)
    }
    pub fn db_client(ip: String, port: u16, options: ClientOptions) -> Client{
        Client::connect_with_options(ip, port, options)
            .expect("Failed to initialize standalone client.")
    }
}
