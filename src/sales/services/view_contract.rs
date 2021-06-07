use crate::sales::Sales;
use crate::sales::entities::Contract;
use crate::sales::domain_values::{ContractFailure, ContractNumber};

pub trait ViewContract {
    fn view(&self, number: ContractNumber) -> Result<Contract, ContractFailure>;
}

impl ViewContract for Sales {
    fn view(&self, number: ContractNumber) -> Result<Contract, ContractFailure> {
        self.contracts.get(&number).ok_or(ContractFailure::ContractDoesNotExist)
    }
}