use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Brand {
    Volvo,
    Mercedes,
    Volkswagen,
    Porsche,
    Tesla,
}