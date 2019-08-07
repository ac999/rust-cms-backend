#[macro_use]
extern crate mysql;
extern crate dotenv;

use mysql as my;

use dotenv::dotenv;
use std::env;

use actix_web::{web, App, HttpServer, Result};

pub mod models;
pub mod config;
pub mod other;

// use self::models::Payment;

// struct MyPool{
//       pool: my::Pool
//     ,
// }

// fn establish_connection() -> my::Pool {
// 	dotenv().ok();

// 	let database_url = env::var("DATABASE_URL")
// 		.expect("DATABASE_URL error");
// 	my::Pool::new(&database_url)
// 		.expect(&format!("Could not connect to {}", database_url))
// }

// fn query_db(my_pool: web::Data<MyPool>
//     ){

//     let pool = &my_pool.pool;
//     let payments = vec![
//         Payment { customer_id: 1, amount: 2, account_name: None },
//         Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
//         Payment { customer_id: 5, amount: 6, account_name: None },
//         Payment { customer_id: 7, amount: 8, account_name: None },
//         Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
//     ];

//     // Let's insert payments to the database
//     // We will use into_iter() because we do not need to map Stmt to anything else.
//     // Also we assume that no error happened in `prepare`.
//     for mut stmt in pool.prepare(r"INSERT INTO payment
//                                        (customer_id, amount, account_name)
//                                    VALUES
//                                        (:customer_id, :amount, :account_name)").into_iter() {
//         for p in payments.iter() {
//             // `execute` takes ownership of `params` so we pass account name by reference.
//             // Unwrap each result just to make sure no errors happened.
//             stmt.execute(params!{
//                 "customer_id" => p.customer_id,
//                 "amount" => p.amount,
//                 "account_name" => &p.account_name,
//             }).unwrap();
//         }
//     }

//     // Let's select payments from database
//     let selected_payments: Vec<Payment> =
//     pool.prep_exec("SELECT customer_id, amount, account_name from payment", ())
//     .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
//         // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
//         // will map each `MyResult` to contained `row` (no proper error handling)
//         // and second call to `map` will map each `row` to `Payment`
//         result.map(|x| x.unwrap()).map(|row| {
//             // ⚠️ Note that from_row will panic if you don't follow your schema
//             let (customer_id, amount, account_name) = my::from_row(row);
//             Payment {
//                 customer_id: customer_id,
//                 amount: amount,
//                 account_name: account_name,
//             }
//         }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
//     }).unwrap(); // Unwrap `Vec<Payment>`

//     // Now make sure that `payments` equals to `selected_payments`.
//     // Mysql gives no guaranties on order of returned rows without `ORDER BY`
//     // so assume we are lukky.
//     println!("Yay!");
// }

fn main(){

    // HttpServer::new(|| App::new()
    //     .data(MyPool{ pool: establish_connection() })
    //     .route(
    //           "/register"
    //         , web::post().to(query_db)
    //         )
    //     )
    //     .bind("127.0.0.1:8081") 
    //     .expect("Can not bind to 127.0.0.1:8081")
    //     .run()
    //     .unwrap();

    let config = config::read_env();

    
}
