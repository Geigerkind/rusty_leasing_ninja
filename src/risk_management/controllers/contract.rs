use std::str::FromStr;

use rocket::serde::json::Json;
use rocket::State;

use crate::risk_management::domain_values::{CreditRating, VoteResult};
use crate::risk_management::dtos::{RateForm, VoteForm};
use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;
use crate::risk_management::services::{CheckCreditRanking, ListContracts, ReadContract, VoteContract};
use crate::shared::domain_values::{ContractFailure, ContractNumber};

#[get("/contracts")]
pub fn get_all_contracts(me: &State<RiskManagement>) -> Json<Vec<Contract>> {
    Json(me.list_all_contracts())
}

#[get("/contract/<contract_number>")]
pub fn get_contract(me: &State<RiskManagement>, contract_number: String) -> Result<Json<Contract>, ContractFailure> {
    let number = ContractNumber::new(contract_number)?;
    me.read_contract(number).ok_or(ContractFailure::ContractDoesNotExist).map(Json)
}

#[post("/rate", data = "<rate_form>")]
pub fn sign_contract(me: &State<RiskManagement>, rate_form: Json<RateForm>) -> Result<(), ContractFailure> {
    let rate_form = rate_form.into_inner();
    let number = ContractNumber::new(rate_form.contract_number)?;
    let rating = CreditRating::new(rate_form.credit_rating)?;
    me.check_credit_ranking(number, rating)
}

#[post("/vote", data = "<vote_form>")]
pub fn vote_contract(me: &State<RiskManagement>, vote_form: Json<VoteForm>) -> Result<(), ContractFailure> {
    let vote_form = vote_form.into_inner();
    let number = ContractNumber::new(vote_form.contract_number)?;
    let vote_result = VoteResult::from_str(&vote_form.vote_result).map_err(|_| ContractFailure::InvalidInput)?;
    me.vote_contract(number, vote_result)
}

