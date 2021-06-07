use crate::sales::entities::Contract;
use crate::sales::domain_values::ContractNumber;
use std::collections::HashMap;

pub struct Contracts(HashMap<ContractNumber, Contract>);

impl Contracts {
    pub fn save(&mut self, contract: Contract) {
        self.0.insert(contract.number().clone(), contract);
    }

    pub fn get(&self, number: &ContractNumber) -> Option<Contract> {
        self.0.get(number).cloned()
    }
}