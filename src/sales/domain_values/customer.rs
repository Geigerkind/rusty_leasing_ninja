use crate::rocket::serde::Serialize;

#[derive(Getters, Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Customer {
    forename: String,
    surname: String,
}

impl Customer {
    pub fn new(forename: String, surname: String) -> Self {
        Customer { forename, surname }
    }

    pub fn name(&self) -> String {
        format!("{1}, {0}", self.forename, self.surname)
    }
}