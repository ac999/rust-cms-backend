use mongodb::{Bson, bson, doc};
use mongodb::{Client, ClientOptions, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main() {
    // Path to file containing trusted server certificates.
    let ca_file = "ssl/ca.crt";
    // Path to file containing client certificate.
    let certificate = "ssl/mongodb.crt";
    // Path to file containing client private key.
    let key_file = "ssl/mongodb.key";
    // Wheter or not to verify that the server certificate is valid.
    // Unless you;re testing out something locally, this should ALWAYS
    // be true.
    let verify_peer = false;

    let options = ClientOptions::with_ssl(ca_file, certificate, key_file,
                                          verify_peer);

    let client = Client::connect_with_options("127.0.0.1", 27017, options)
        .expect("Failed to initialize standalone client.");

}
