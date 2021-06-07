use crate::sales::domain::value::Currency;

#[derive(Getters)]
pub struct Amount {
    amount: u32,
    currency: Currency,
}

impl Amount {
    pub fn new(amount: u32, currency: Currency) -> Self {
        Amount { amount, currency }
    }
}