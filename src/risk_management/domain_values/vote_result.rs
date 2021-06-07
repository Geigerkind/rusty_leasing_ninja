use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize, EnumString)]
#[serde(crate = "rocket::serde")]
#[repr(u8)]
pub enum VoteResult {
    Accepted,
    AcceptedWithObligations,
    Rejected
}