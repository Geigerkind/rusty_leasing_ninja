use crate::risk_management::domain_values::{CreditRating, VoteResult};
use crate::rocket::serde::Serialize;
use crate::shared::domain_values::ContractNumber;

#[derive(Debug, Clone, Serialize, Getters)]
#[serde(crate = "rocket::serde")]
pub struct Contract {
    number: ContractNumber,
    credit_rating: Option<CreditRating>,
    vote_result: Option<VoteResult>,
}

impl Contract {
    #[cfg(test)]
    pub fn new_test(number: ContractNumber, credit_rating: Option<CreditRating>, vote_result: Option<VoteResult>) -> Self {
        Contract {
            number,
            credit_rating,
            vote_result,
        }
    }

    pub fn new(number: ContractNumber) -> Self {
        Contract {
            number,
            credit_rating: None,
            vote_result: None,
        }
    }

    pub fn check_credit_rating(&mut self, credit_rating: CreditRating) {
        assert!(!self.is_voted(), "Contract should not have been voted yet!");
        self.credit_rating = Some(credit_rating);
        assert!(self.is_rated(), "Contract should be rated!");
    }

    pub fn vote(&mut self, vote_result: VoteResult) {
        assert!(self.is_rated(), "Contract must be rated, before its voted!");
        self.vote_result = Some(vote_result);
        assert!(self.is_voted(), "Contract must have been voted!");
    }

    pub fn is_rated(&self) -> bool {
        self.credit_rating.is_some()
    }

    pub fn is_voted(&self) -> bool {
        self.vote_result.is_some()
    }
}

#[cfg(test)]
mod test {
    use crate::risk_management::entities::Contract;
    use crate::shared::domain_values::ContractNumber;
    use crate::risk_management::domain_values::{VoteResult, CreditRating};

    #[test]
    fn correctly_initialized() {
        let number = ContractNumber::new("1234-1243-3242-1234".to_owned()).unwrap();
        let contract = Contract::new(number.clone());
        assert!(!contract.is_rated());
        assert!(!contract.is_voted());
        assert_eq!(contract.number, number);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_voted_before_rated() {
        let number = ContractNumber::new("1234-1243-3242-1234".to_owned()).unwrap();
        let mut contract = Contract::new(number.clone());
        contract.vote(VoteResult::Accepted);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_rated_when_its_already_voted() {
        let number = ContractNumber::new("1234-1243-3242-1234".to_owned()).unwrap();
        let mut contract = Contract::new_test(number.clone(), None, Some(VoteResult::Accepted));
        contract.check_credit_rating(CreditRating::new(4).unwrap());
    }

    #[test]
    fn check_rating() {
        let number = ContractNumber::new("1234-1243-3242-1234".to_owned()).unwrap();
        let mut contract = Contract::new(number.clone());

        assert!(!contract.is_rated());
        assert!(!contract.is_voted());
        contract.check_credit_rating(CreditRating::new(8).unwrap());
        assert!(contract.is_rated());
        assert!(!contract.is_voted());
    }

    #[test]
    fn vote() {
        let number = ContractNumber::new("1234-1243-3242-1234".to_owned()).unwrap();
        let mut contract = Contract::new_test(number.clone(), Some(CreditRating::new(5).unwrap()), None);

        assert!(contract.is_rated());
        assert!(!contract.is_voted());
        contract.vote(VoteResult::Accepted);
        assert!(contract.is_rated());
        assert!(contract.is_voted());
    }
}