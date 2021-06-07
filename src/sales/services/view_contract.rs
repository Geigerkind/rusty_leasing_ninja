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