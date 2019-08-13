#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
      pub email: String
    , pub username: String
    , pub password: String
    , pub repeat_password: String
    ,
}