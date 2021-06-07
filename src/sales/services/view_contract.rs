use crate::sales::entities::Contract;
use crate::sales::Sales;
use crate::shared::domain_values::{ContractFailure, ContractNumber};

pub trait ViewContract {
    fn view_contract(&self, number: ContractNumber) -> Result<Contract, ContractFailure>;
}

impl ViewContract for Sales {
    fn view_contract(&self, number: ContractNumber) -> Result<Contract, ContractFailure> {
        self.contracts.get(&number).ok_or(ContractFailure::ContractDoesNotExist)
    }
}

#[cfg(test)]
mod test {
    use crate::sales::Sales;
    use crate::shared::domain_values::ContractNumber;
    use crate::sales::domain_values::{Customer, Car, Brand, Currency, Amount};
    use crate::sales::services::{FillOutContract, ViewContract};

    #[test]
    fn view_contract() {
        let sales = Sales::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let customer = Customer::new("Max".to_owned(), "Mustermann".to_owned());
        let car = Car::new(Brand::Volkswagen);
        let price = Amount::new(123423, Currency::Dollar);

        assert!(sales.contracts.all().is_empty());
        sales.fill_out_contract(number.clone(), customer.clone(), car.clone(), price.clone());
        assert!(!sales.contracts.all().is_empty());
        let result = sales.view_contract(number.clone());
        assert!(result.is_ok());
        let contract = result.unwrap();
        assert_eq!(contract.number(), &number);
        assert_eq!(contract.customer(), &customer);
        assert_eq!(contract.car(), &car);
        assert_eq!(contract.price(), &price);
        assert!(!contract.is_signed());
    }

}