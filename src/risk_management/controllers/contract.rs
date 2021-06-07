use rocket::serde::json::Json;
use rocket::State;

use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;
use crate::risk_management::services::{ListContracts, ReadContract};
use crate::shared::domain_values::{ContractFailure, ContractNumber};

#[get("/contracts")]
pub fn get_all_contracts(me: &State<RiskManagement>) -> Json<Vec<Contract>> {
    Json(me.list_all_contracts())
}

#[get("/contracts/<contract_number>")]
pub fn get_contract(me: &State<RiskManagement>, contract_number: String) -> Result<Json<Contract>, ContractFailure> {
    let number = ContractNumber::new(contract_number)?;
    me.read_contract(number).ok_or(ContractFailure::ContractDoesNotExist).map(Json)
}

