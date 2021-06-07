use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;
use crate::shared::domain_values::ContractNumber;

pub trait ReadContract {
    fn read_contract(&self, number: ContractNumber) -> Option<Contract>;
}

impl ReadContract for RiskManagement {
    fn read_contract(&self, number: ContractNumber) -> Option<Contract> {
        self.contracts.get(&number)
    }
}