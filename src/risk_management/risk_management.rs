use crate::risk_management::repositories::Contracts;

#[derive(Debug)]
pub struct RiskManagement {
    contracts: Contracts
}

impl RiskManagement {
    pub fn new() -> Self {
        RiskManagement { contracts: Contracts::new() }
    }
}
