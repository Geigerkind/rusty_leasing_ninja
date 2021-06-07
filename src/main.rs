#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate strum;

use crate::sales::Sales;

mod sales;
mod risk_management;

#[launch]
fn launch() -> _ {
    let sales = Sales::new();

    rocket::build()
        .manage(sales)
        .mount("/sales", routes![
            crate::sales::controllers::contract::view_contract,
            crate::sales::controllers::contract::fill_out_contract,
        ])
}