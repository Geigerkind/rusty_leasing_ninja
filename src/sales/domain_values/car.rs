use crate::rocket::serde::Serialize;
use crate::sales::domain_values::Brand;

#[derive(Getters, Debug, Clone, Serialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Car {
    brand: Brand
}

impl Car {
    pub fn new(brand: Brand) -> Self {
        Car { brand }
    }
}