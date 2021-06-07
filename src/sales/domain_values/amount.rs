use crate::sales::domain_values::Currency;

#[derive(Getters, Debug, Clone)]
pub struct Amount {
    amount: u32,
    currency: Currency,
}

impl Amount {
    pub fn new(amount: u32, currency: Currency) -> Self {
        Amount { amount, currency }
    }
}