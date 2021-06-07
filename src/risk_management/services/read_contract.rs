use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;
use crate::shared::domain_values::ContractNumber;

pub trait ReadContract {
    fn read_contract(&self, number: ContractNumber) -> Option<Contract>;
}

impl ReadContract for RiskManagement {
    fn read_contract(&self, number: ContractNumber) -> Option<Contract> {
        self.contracts.get(&number)
    }
}

#[cfg(test)]
mod test {
    use crate::risk_management::RiskManagement;
    use crate::risk_management::services::{ApplicationInbox, ReadContract};
    use crate::shared::domain_values::ContractNumber;

    #[test]
    fn read_contract() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        risk_management.confirm_signed_contract(number.clone());
        let contract = risk_management.read_contract(number.clone());
        assert!(contract.is_some());
        let contract = contract.unwrap();
        assert_eq!(contract.number(), &number);
    }
}