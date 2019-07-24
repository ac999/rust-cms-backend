use serde_json::Number;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub repeat-password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse{
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoveryRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorListResponse {
    pub errors: Vec<String>,
}

//struct UserRequest{
//    pub id: String,
//    pub email: String,
//    pub username: String,
//    pub password: String,
//    pub token: Option(String),
//    pub active: bool,
//    pub register_date: String,
//    pub last_login: String,
//}
