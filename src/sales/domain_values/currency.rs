use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum Currency {
    Euro,
    Dollar,
    Yen,
}