pub use mongodb::{Client, ClientOptions};

pub mod db_connection{
    pub mod connection{
        pub fn set_ssl_options(ca_file: String, cl_certificate: String,
                               key_file: String, verify_peer: bool)
            -> ClientOptions {}
        pub fn db_client(ip: String, port: u16, options: ClientOptions)
            -> Client {}
    }
}
