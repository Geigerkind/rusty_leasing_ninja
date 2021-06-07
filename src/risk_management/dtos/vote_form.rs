use crate::rocket::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VoteForm {
    pub contract_number: String,
    pub vote_result: String
}