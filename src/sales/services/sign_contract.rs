use crate::sales::domain_values::ContractFailure;
use crate::sales::Sales;
use crate::shared::domain_values::{ContractNumber, SignDate};

pub trait SignContract {
    fn sign_contract(&self, number: ContractNumber, sign_date: SignDate) -> Result<(), ContractFailure>;
}

impl SignContract for Sales {
    fn sign_contract(&self, number: ContractNumber, sign_date: SignDate) -> Result<(), ContractFailure> {
        let mut contract = self.contracts.get(&number).ok_or(ContractFailure::ContractDoesNotExist)?;
        contract.sign(sign_date);
        self.contracts.save(contract);
        // TODO: confirm contract with risk management inbox
        Ok(())
    }
}