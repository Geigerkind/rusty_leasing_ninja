use crate::sales::repositories::Contracts;

#[derive(Debug)]
pub struct Sales {
    pub contracts: Contracts
}

impl Sales {
    pub fn new() -> Self {
        Sales { contracts: Contracts::new() }
    }
}