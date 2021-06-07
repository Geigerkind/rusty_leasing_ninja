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

#[cfg(test)]
mod test {
    use chrono::{DateTime, NaiveDateTime, Utc};

    use crate::risk_management::RiskManagement;
    use crate::sales::domain_values::{Amount, Brand, Car, Currency, Customer};
    use crate::sales::Sales;
    use crate::sales::services::{FillOutContract, SignContract};
    use crate::shared::domain_values::{ContractNumber, SignDate};

    #[test]
    fn sign_contract() {
        let sales = Sales::new();
        let risk_management = RiskManagement::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let customer = Customer::new("Max".to_owned(), "Mustermann".to_owned());
        let car = Car::new(Brand::Porsche);
        let price = Amount::new(123423, Currency::Yen);
        let sign_date = SignDate::from_date_time(DateTime::from_utc(NaiveDateTime::from_timestamp(32432423, 0), Utc));

        sales.fill_out_contract(number.clone(), customer.clone(), car.clone(), price.clone());
        assert!(!sales.contracts.all().is_empty());
        let contract = sales.contracts.get(&number).unwrap();
        assert!(!contract.is_signed());

        assert!(risk_management.contracts.all().is_empty());
        let signature = sales.sign_contract(&risk_management, number.clone(), sign_date);
        assert!(signature.is_ok());
        assert!(!risk_management.contracts.all().is_empty());
        let contract = sales.contracts.get(&number).unwrap();
        assert!(contract.is_signed());
    }
}