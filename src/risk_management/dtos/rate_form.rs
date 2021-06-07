use crate::rocket::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RateForm {
    pub contract_number: String,
    pub credit_rating: u8
}