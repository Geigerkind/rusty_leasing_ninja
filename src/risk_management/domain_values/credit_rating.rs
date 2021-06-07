use crate::rocket::serde::Serialize;
use crate::shared::domain_values::ContractFailure;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreditRating(u8);

impl CreditRating {
    pub fn new(rating: u8) -> Result<Self, ContractFailure> {
        if rating == 0 || rating > 10 {
            Err(ContractFailure::InvalidInput)
        } else {
            Ok(CreditRating(rating))
        }
    }

    pub fn rating(&self) -> &u8 {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::risk_management::domain_values::CreditRating;

    #[test]
    fn valid_ratings() {
        for i in 1u8..11u8 {
            let credit_rating = CreditRating::new(i);
            assert!(credit_rating.is_ok());
            assert_eq!(credit_rating.unwrap().rating(), &i);
        }
    }

    #[test]
    fn invalid_ratings() {
        assert!(CreditRating::new(0).is_err());
        for i in 11u8..255 {
            assert!(CreditRating::new(i).is_err());
        }
        assert!(CreditRating::new(255).is_err());
    }

}