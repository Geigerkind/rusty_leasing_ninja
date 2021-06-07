use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ContractNumber(String);

impl ContractNumber {
    pub fn new(number: String) -> Self {
        assert!(is_valid_rainbow_format(&number), "Invalid ContractNumber Format");
        ContractNumber(number)
    }
}

// Rainbow Format:
// XXXX-XXXX-XXXX-XXXX
// Where X is alphanumeric
fn is_valid_rainbow_format(number: &String) -> bool {
    let number_parts: Vec<&str> = number.split("-").collect();
    number_parts.len() == 4 && number_parts.iter().all(|part| part.len() == 4 && part.chars().all(char::is_alphanumeric))
}

#[cfg(test)]
mod test {
    mod rainbow_format {
        use crate::sales::domain_values::contract_number::is_valid_rainbow_format;

        #[test]
        fn invalid_gibberish() {
            assert!(!is_valid_rainbow_format(&"asfsdfsdfsdfsdfsd".to_owned()));
        }

        #[test]
        fn invalid_too_long() {
            assert!(!is_valid_rainbow_format(&"ABCD-1234-A1B2-C3d4-afvs".to_owned()));
        }

        #[test]
        fn invalid_too_short() {
            assert!(!is_valid_rainbow_format(&"ABCD-1234-A1B2".to_owned()));
        }

        #[test]
        fn invalid_not_alphanumeric() {
            assert!(!is_valid_rainbow_format(&"ABCD-1234-A1B2-][Ab".to_owned()));
        }

        #[test]
        fn invalid_too_few_characters() {
            assert!(!is_valid_rainbow_format(&"ABCD-1234-A1B-][Ab".to_owned()));
        }

        #[test]
        fn valid() {
            assert!(is_valid_rainbow_format(&"ABCD-1234-A1B2-Qas3".to_owned()));
        }
    }

    mod constructor {
        use crate::sales::domain_values::ContractNumber;

        #[test]
        #[should_panic]
        fn invalid_format() {
            let _ = ContractNumber::new("ABCD-1234-A[[2-Qas3".to_owned());
        }

        #[test]
        fn valid() {
            let _ = ContractNumber::new("ABCD-1234-A1B2-Qas3".to_owned());
        }
    }
}