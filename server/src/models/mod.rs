use std::fs;
use std::io::{Error};


#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
      pub email: String
    , pub username: String
    , pub password: String
    , pub repeat_password: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
      pub username: String
    , pub password: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse{
      pub token: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverRequest {
      pub email: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorListResponse {
	  pub errors: Vec<String>
	,

}
