use crate::shared::domain_values::{ContractNumber, ContractFailure};
use crate::risk_management::domain_values::VoteResult;
use crate::risk_management::RiskManagement;

pub trait VoteContract {
    fn vote_contract(&self, number: ContractNumber, vote_result: VoteResult) -> Result<(), ContractFailure>;
}

impl VoteContract for RiskManagement {
    fn vote_contract(&self, number: ContractNumber, vote_result: VoteResult) -> Result<(), ContractFailure> {
        let mut contract = self.contracts.get(&number).ok_or(ContractFailure::ContractDoesNotExist)?;
        contract.vote(vote_result);
        self.contracts.save(number, contract);
        Ok(())
    }
}