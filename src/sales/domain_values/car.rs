use crate::sales::domain_values::Brand;

#[derive(Getters)]
pub struct Car {
    brand: Brand
}

impl Car {
    pub fn new(brand: Brand) -> Self {
        Car { brand }
    }
}