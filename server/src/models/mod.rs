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

pub fn load_register_request() -> Result <RegisterRequest, Error> {
	let request = fs::read_to_string("./request/register-user.json");

	match request{
		  Ok(r) => Ok(serde_json::from_str(
			&r).expect("invalid json format"))
		, Err(no_file) => Err(no_file)
		,
	}
}

pub fn load_login_request() -> Result <LoginRequest, Error> {
  let request = fs::read_to_string("./request/login-user.json");

  match request{
      Ok(r) => Ok(serde_json::from_str(
      &r).expect("invalid json format"))
    , Err(no_file) => Err(no_file)
    ,
  }
}

pub fn load_recover_request() -> Result <RecoverRequest, Error> {
  let request = fs::read_to_string("./request/recover-user.json");

  match request{
      Ok(r) => Ok(serde_json::from_str(
      &r).expect("invalid json format"))
    , Err(no_file) => Err(no_file)
    ,
  }
}