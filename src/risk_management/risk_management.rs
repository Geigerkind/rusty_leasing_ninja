use crate::shared::repositories::MemStorage;
use crate::shared::domain_values::ContractNumber;
use crate::risk_management::entities::Contract;

#[derive(Debug)]
pub struct RiskManagement {
    contracts: MemStorage<ContractNumber, Contract>
}

impl RiskManagement {
    pub fn new() -> Self {
        RiskManagement { contracts: MemStorage::new() }
    }
}
