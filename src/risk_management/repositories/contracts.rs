use std::collections::HashMap;
use std::sync::RwLock;

use crate::risk_management::entities::Contract;
use crate::shared::domain_values::ContractNumber;

#[derive(Debug)]
pub struct Contracts(RwLock<HashMap<ContractNumber, Contract>>);

impl Contracts {
    pub fn new() -> Self {
        Contracts(RwLock::new(HashMap::default()))
    }

    pub fn save(&self, contract: Contract) {
        let mut repo = self.0.write().unwrap();
        repo.insert(contract.number().clone(), contract);
    }

    pub fn get(&self, number: &ContractNumber) -> Option<Contract> {
        let repo = self.0.read().unwrap();
        repo.get(number).cloned()
    }
}