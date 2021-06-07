use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;

pub trait ListContracts {
    fn list_all_contracts(&self) -> Vec<Contract>;
}

impl ListContracts for RiskManagement {
    fn list_all_contracts(&self) -> Vec<Contract> {
        self.contracts.all()
    }
}