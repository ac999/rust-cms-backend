use bcrypt::{hash, verify};

use models;
use database;

fn register(my_pool: web::Data<database::MyPool>
    , info: web::Json<models::RegisterRequest>
        ) -> Result<String> {
	
	Ok(String::from("test"))
    
    }