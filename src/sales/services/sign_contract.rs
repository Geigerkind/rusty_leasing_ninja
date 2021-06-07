use crate::risk_management::RiskManagement;
use crate::risk_management::services::ApplicationInbox;
use crate::sales::Sales;
use crate::shared::domain_values::{ContractFailure, ContractNumber, SignDate};

pub trait SignContract {
    fn sign_contract(&self, risk_management: &RiskManagement, number: ContractNumber, sign_date: SignDate) -> Result<(), ContractFailure>;
}

impl SignContract for Sales {
    fn sign_contract(&self, risk_management: &RiskManagement, number: ContractNumber, sign_date: SignDate) -> Result<(), ContractFailure> {
        let mut contract = self.contracts.get(&number).ok_or(ContractFailure::ContractDoesNotExist)?;
        contract.sign(sign_date);
        self.contracts.save(number.clone(), contract);
        risk_management.confirm_signed_contract(number);
        Ok(())
    }
}