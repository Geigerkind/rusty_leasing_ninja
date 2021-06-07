use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;
use crate::shared::domain_values::ContractNumber;

pub trait ApplicationInbox {
    fn confirm_signed_contract(&self, number: ContractNumber);
}

impl ApplicationInbox for RiskManagement {
    fn confirm_signed_contract(&self, number: ContractNumber) {
        self.contracts.save(number.clone(), Contract::new(number))
    }
}