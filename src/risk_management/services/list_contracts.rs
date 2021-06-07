use crate::risk_management::entities::Contract;
use crate::risk_management::RiskManagement;

pub trait ListContracts {
    fn list_all_contracts(&self) -> Vec<Contract>;
}

impl ListContracts for RiskManagement {
    fn list_all_contracts(&self) -> Vec<Contract> {
        self.contracts.all()
    }
}

#[cfg(test)]
mod test {
    use crate::risk_management::RiskManagement;
    use crate::shared::domain_values::ContractNumber;
    use crate::risk_management::services::{ApplicationInbox, ListContracts};

    #[test]
    fn list_all_contracts() {
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        risk_management.confirm_signed_contract(number.clone());
        assert_eq!(risk_management.list_all_contracts().len(), 1);
        let contracts = risk_management.list_all_contracts();
        let contract = contracts.get(0).unwrap();
        assert_eq!(contract.number(), &number);
    }
}