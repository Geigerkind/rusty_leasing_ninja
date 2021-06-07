use rocket::State;
use crate::risk_management::RiskManagement;
use rocket::serde::json::Json;
use crate::risk_management::entities::Contract;
use crate::risk_management::services::ListContracts;

#[get("/contracts")]
pub fn get_all_contracts(me: &State<RiskManagement>) -> Json<Vec<Contract>> {
    Json(me.list_all_contracts())
}