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

#[cfg(test)]
mod test {
    use crate::risk_management::RiskManagement;
    use crate::shared::domain_values::{ContractNumber, ContractFailure};
    use crate::risk_management::services::{ApplicationInbox, CheckCreditRanking};
    use crate::risk_management::domain_values::CreditRating;

    #[test]
    fn check_credit_ranking() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let ranking = CreditRating::new(8).unwrap();
        risk_management.confirm_signed_contract(number.clone());
        let contract = risk_management.contracts.get(&number).unwrap();
        assert!(!contract.is_rated());
        let result = risk_management.check_credit_ranking(number.clone(), ranking.clone());
        assert!(result.is_ok());
        let contract = risk_management.contracts.get(&number).unwrap();
        assert!(contract.is_rated());
    }

    #[test]
    fn credit_ranking_does_not_exist() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let ranking = CreditRating::new(8).unwrap();
        let result = risk_management.check_credit_ranking(number.clone(), ranking.clone());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ContractFailure::ContractDoesNotExist);
    }
}