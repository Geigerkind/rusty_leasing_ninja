#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate strum;

use crate::sales::Sales;
use crate::risk_management::RiskManagement;

mod sales;
mod risk_management;

#[launch]
fn launch() -> _ {
    let sales = Sales::new();
    let risk_management = RiskManagement;

    rocket::build()
        .manage(sales)
        .manage(risk_management)
        .mount("/sales", routes![
            crate::sales::controllers::contract::view_contract,
            crate::sales::controllers::contract::fill_out_contract,
            crate::sales::controllers::contract::sign_contract,
        ])
}