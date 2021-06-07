use crate::sales::domain_values::{Amount, Car, ContractNumber, Customer};
use crate::sales::entities::Contract;
use crate::sales::Sales;

pub trait FillOutContract {
    fn fill_out_contract(&mut self, number: ContractNumber, customer: Customer, car: Car, price: Amount);
}

impl FillOutContract for Sales {
    fn fill_out_contract(&mut self, number: ContractNumber, customer: Customer, car: Car, price: Amount) {
        self.contracts.save(Contract::new(number, customer, car, price));
    }
}