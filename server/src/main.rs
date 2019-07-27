// use mongodb::{Bson, bson, doc};
// use mongodb::{Client, ClientOptions, ThreadedClient};
// use mongodb::db::ThreadedDatabase;
// pub use crate::db_connection::connection;

// fn main() {
//     let client = connection::db_client("127.0.0.1", 27017,
//                                        connection::set_ssl_options(
//             "ssl/ca.crt", "ssl/mongodb.crt", "ssl/mongodb.key", false)
//         );

//     let coll = client.db("zoolx").collection("users");

//     let doc = doc! {
//         "_id": "test"
//     };

//     let mut cursor = coll.find(Some(doc.clone()), None)
//         .ok().expect("Failed to execute find.");

//     let item = cursor.next();
// }
fn main(){
	
}