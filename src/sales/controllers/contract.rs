use rocket::serde::json::Json;
use rocket::State;

use crate::sales::domain_values::{ContractFailure, ContractNumber, Customer, Car, Brand, Amount, Currency};
use crate::sales::entities::Contract;
use crate::sales::Sales;
use crate::sales::services::{ViewContract, FillOutContract};
use crate::sales::dtos::FillOutForm;
use std::str::FromStr;

#[get("/view_contract/<contract_number>")]
pub fn view_contract(me: &State<Sales>, contract_number: String) -> Result<Json<Contract>, ContractFailure> {
    // TODO: Failure should be handled
    let number = ContractNumber::new(contract_number);
    me.view_contract(number).map(Json)
}

#[post("/fill_out_contract", data = "<fill_out_form>")]
pub fn fill_out_contract(me: &State<Sales>, fill_out_form: Json<FillOutForm>) -> Result<(), ContractFailure> {
    // TODO: Failure should be handled
    let fill_out_form = fill_out_form.into_inner();
    let number = ContractNumber::new(fill_out_form.contract_number);
    let customer = Customer::new(fill_out_form.customer_forename, fill_out_form.customer_surname);
    let car_brand = Brand::from_str(&fill_out_form.car_brand).map_err(|_| ContractFailure::InvalidInput)?;
    let car = Car::new(car_brand);
    let price = Amount::new(fill_out_form.price, Currency::Euro);

    me.fill_out_contract(number, customer, car, price);
    Ok(())
}