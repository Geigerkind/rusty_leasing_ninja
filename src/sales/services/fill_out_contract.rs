use crate::sales::domain_values::{Amount, Car, Customer};
use crate::sales::entities::Contract;
use crate::sales::Sales;
use crate::shared::domain_values::ContractNumber;

pub trait FillOutContract {
    fn fill_out_contract(&self, number: ContractNumber, customer: Customer, car: Car, price: Amount);
}

impl FillOutContract for Sales {
    fn fill_out_contract(&self, number: ContractNumber, customer: Customer, car: Car, price: Amount) {
        self.contracts.save(number.clone(), Contract::new(number, customer, car, price));
    }
}