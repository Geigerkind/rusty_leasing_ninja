use crate::sales::entities::Contract;
use crate::shared::domain_values::ContractNumber;
use crate::shared::repositories::MemStorage;

#[derive(Debug)]
pub struct Sales {
    pub contracts: MemStorage<ContractNumber, Contract>
}

impl Sales {
    pub fn new() -> Self {
        Sales { contracts: MemStorage::new() }
    }
}