#[macro_use]
extern crate rocket;
#[macro_use]
extern crate derive_getters;

mod sales;
mod risk_management;

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![])
}