use rocket::serde::json::Json;
use rocket::State;

use crate::sales::domain_values::{ContractFailure, ContractNumber};
use crate::sales::entities::Contract;
use crate::sales::Sales;
use crate::sales::services::ViewContract;

#[get("/view_contract/<contract_number>")]
pub fn view_contract(me: &State<Sales>, contract_number: String) -> Result<Json<Contract>, ContractFailure> {
    me.view_contract(ContractNumber::new(contract_number)).map(Json)
}