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

#[cfg(test)]
mod test {
    use crate::risk_management::RiskManagement;
    use crate::shared::domain_values::{ContractNumber, ContractFailure};
    use crate::risk_management::services::{ApplicationInbox, CheckCreditRanking, VoteContract};
    use crate::risk_management::domain_values::{CreditRating, VoteResult};

    #[test]
    fn vote_contract() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let vote_result = VoteResult::Accepted;
        risk_management.confirm_signed_contract(number.clone());
        let _ = risk_management.check_credit_ranking(number.clone(), CreditRating::new(8).unwrap());
        let contract = risk_management.contracts.get(&number).unwrap();
        assert!(!contract.is_voted());
        let result = risk_management.vote_contract(number.clone(), vote_result.clone());
        assert!(result.is_ok());
        let contract = risk_management.contracts.get(&number).unwrap();
        assert!(contract.is_voted());
        assert_eq!(contract.vote_result(), &Some(vote_result));
    }

    #[test]
    #[should_panic]
    fn vote_contract_panic_if_not_rated() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        risk_management.confirm_signed_contract(number.clone());
        let _ = risk_management.vote_contract(number.clone(), VoteResult::Accepted);
    }

    #[test]
    fn contract_does_not_exist() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let vote_result = VoteResult::Accepted;
        let result = risk_management.vote_contract(number.clone(), vote_result);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ContractFailure::ContractDoesNotExist);
    }
}