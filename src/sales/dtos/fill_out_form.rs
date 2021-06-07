use crate::rocket::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FillOutForm {
    pub contract_number: String,
    pub customer_forename: String,
    pub customer_surname: String,
    pub car_brand: String,
    pub price: u32
}