use rocket::serde::json::Json;
use rocket::State;

use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;
use crate::risk_management::services::{ListContracts, ReadContract, CheckCreditRanking};
use crate::shared::domain_values::{ContractFailure, ContractNumber};
use crate::risk_management::dtos::RateForm;
use crate::risk_management::domain_values::CreditRating;

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