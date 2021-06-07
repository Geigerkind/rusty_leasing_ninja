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

#[cfg(test)]
mod test {
    use crate::risk_management::RiskManagement;
    use crate::shared::domain_values::ContractNumber;
    use crate::risk_management::services::ApplicationInbox;

    #[test]
    fn confirm_signed_contract() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();

        assert!(risk_management.contracts.all().is_empty());
        risk_management.confirm_signed_contract(number.clone());
        assert!(!risk_management.contracts.all().is_empty());
        let contract = risk_management.contracts.get(&number);
        assert!(contract.is_some());
        let contract = contract.unwrap();
        assert_eq!(contract.number(), &number);
        assert!(!contract.is_rated());
        assert!(!contract.is_voted());
    }
}