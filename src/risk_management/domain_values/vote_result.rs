use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
#[repr(u8)]
pub enum VoteResult {
    Accepted,
    AcceptedWithObligations,
    Rejected
}