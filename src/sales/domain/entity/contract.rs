use crate::sales::domain::value::{Amount, Car, ContractNumber, Customer, SignDate};

#[derive(Getters)]
pub struct Contract {
    number: ContractNumber,
    customer: Customer,
    car: Car,
    price: Amount,
    sign_date: Option<SignDate>
}

impl Contract {
    #[cfg(test)]
    pub fn new_test(number: ContractNumber, customer: Customer, car: Car, price: Amount, sign_date: SignDate) -> Self {
        Contract {
            number,
            customer,
            car,
            price,
            sign_date: Some(sign_date)
        }
    }
    
    pub fn new(number: ContractNumber, customer: Customer, car: Car, price: Amount) -> Self {
        Contract {
            number,
            customer,
            car,
            price,
            sign_date: None
        }
    }

    pub fn sign(&mut self, sign_date: SignDate) {
        assert!(!self.is_signed(), "Attempted to sign contract that is already signed!");
        self.sign_date = Some(sign_date);
        assert!(self.is_signed(), "Contract must be signed after signing it!");
    }

    fn is_signed(&self) -> bool {
        self.sign_date.is_some()
    }

}

#[cfg(test)]
mod test {
    mod sign {
        use crate::sales::domain::entity::Contract;
        use crate::sales::domain::value::{ContractNumber, Customer, Car, Brand, Amount, Currency, SignDate};

        #[test]
        #[should_panic]
        fn should_fail_with_signed_contract() {
            let contract_number = ContractNumber::new("1234-1234-1234-1234".to_owned());
            let customer = Customer::new("Max".to_owned(), "Mustermann".to_owned());
            let car = Car::new(Brand::Tesla);
            let amount = Amount::new(12345, Currency::Euro);
            let sign_date = SignDate::from_year_month_day(2020, 1, 1);
            let mut contract = Contract::new_test(contract_number, customer, car, amount, sign_date);

            assert!(contract.is_signed());
            contract.sign(SignDate::from_year_month_day(2021, 1, 1));
        }

        #[test]
        fn should_be_signed_after_signing() {
            let contract_number = ContractNumber::new("1234-1234-1234-1234".to_owned());
            let customer = Customer::new("Max".to_owned(), "Mustermann".to_owned());
            let car = Car::new(Brand::Tesla);
            let amount = Amount::new(12345, Currency::Euro);
            let mut contract = Contract::new(contract_number, customer, car, amount);

            let sign_date = SignDate::from_year_month_day(2020, 1, 1);
            assert!(!contract.is_signed());
            contract.sign(sign_date.clone());
            assert!(contract.is_signed());
            assert_eq!(contract.sign_date(), &Some(sign_date));
        }
    }
}