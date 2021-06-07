use crate::sales::domain_values::{Amount, Car, Customer};
use crate::sales::entities::Contract;
use crate::sales::Sales;
use crate::shared::domain_values::ContractNumber;

pub trait FillOutContract {
    fn fill_out_contract(&self, number: ContractNumber, customer: Customer, car: Car, price: Amount);
}

impl FillOutContract for Sales {
    fn fill_out_contract(&self, number: ContractNumber, customer: Customer, car: Car, price: Amount) {
        self.contracts.save(number.clone(), Contract::new(number, customer, car, price));
    }
}

#[cfg(test)]
mod test {
    use crate::sales::Sales;
    use crate::sales::services::FillOutContract;
    use crate::shared::domain_values::ContractNumber;
    use crate::sales::domain_values::{Customer, Car, Brand, Amount, Currency};

    #[test]
    fn fill_out_contract() {
        let sales = Sales::new();
        let number = ContractNumber::new("1234-1234-1234-1234".to_owned()).unwrap();
        let customer = Customer::new("Max".to_owned(), "Mustermann".to_owned());
        let car = Car::new(Brand::Porsche);
        let price = Amount::new(123423, Currency::Euro);

        assert!(sales.contracts.all().is_empty());
        sales.fill_out_contract(number.clone(), customer.clone(), car.clone(), price.clone());
        assert!(!sales.contracts.all().is_empty());
        let contract = sales.contracts.get(&number);
        assert!(contract.is_some());
        let contract = contract.unwrap();
        assert_eq!(contract.number(), &number);
        assert_eq!(contract.customer(), &customer);
        assert_eq!(contract.car(), &car);
        assert_eq!(contract.price(), &price);
        assert!(!contract.is_signed());
    }

}