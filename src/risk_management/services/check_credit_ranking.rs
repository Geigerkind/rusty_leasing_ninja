use crate::risk_management::domain_values::CreditRating;
use crate::risk_management::RiskManagement;
use crate::shared::domain_values::{ContractNumber, ContractFailure};

pub trait CheckCreditRanking {
    fn check_credit_ranking(&self, number: ContractNumber, ranking: CreditRating) -> Result<(), ContractFailure>;
}

impl CheckCreditRanking for RiskManagement {
    fn check_credit_ranking(&self, number: ContractNumber, ranking: CreditRating) -> Result<(), ContractFailure> {
        let mut contract = self.contracts.get(&number).ok_or(ContractFailure::ContractDoesNotExist)?;
        contract.check_credit_rating(ranking);
        self.contracts.save(number, contract);
        Ok(())
    }
}