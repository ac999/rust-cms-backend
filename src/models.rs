#[derive(Debug, PartialEq, Eq)]
pub struct Payment {
    pub customer_id: i32,
    pub amount: i32,
    pub account_name: Option<String>,
}