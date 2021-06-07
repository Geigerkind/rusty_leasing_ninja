use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize, EnumString)]
#[serde(crate = "rocket::serde")]
pub enum Brand {
    Volvo,
    Mercedes,
    Volkswagen,
    Porsche,
    Tesla,
}