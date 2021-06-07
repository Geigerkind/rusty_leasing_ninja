use std::io::Cursor;

use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;

use crate::rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ContractFailure {
    ContractDoesNotExist,
    InvalidInput
}

impl<'r> Responder<'r, 'r> for ContractFailure {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        let body;
        let status = match self {
            Self::InvalidInput => {
                body = "Invalid input!".to_owned();
                Status::new(400)
            }
            Self::ContractDoesNotExist => {
                body = "Contract does not exist!".to_owned();
                Status::new(404)
            }
        };
        Response::build().status(status).sized_body(body.len(), Cursor::new(body)).ok()
    }
}