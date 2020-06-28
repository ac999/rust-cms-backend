#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
      pub email: String
    , pub username: String
    , pub mailing_list: Option<bool>
    , 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationRequest {
      pub email: String
    , pub token: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetPasswordRequest {
      pub email: String
    , pub username: String
    , pub password: String
    , pub rpassword: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
      pub username: String
    , pub password: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordResetRequest {
      pub email: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerResponse {
      pub status: String
    , pub message: String
    ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
      pub title: String
    , pub description: String
    , pub author: String
    // , pub create_date: String
    // , pub edit_date: Option<String>
    ,
}
