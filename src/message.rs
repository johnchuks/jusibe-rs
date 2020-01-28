extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMSResponse {
  pub status: String,
  pub message_id: String,
  pub credit_used: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SMSCreditResponse {
  sms_credits: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SMSRequestPayload <'a> {
  pub to: &'a str,
  pub from: &'a str,
  pub message: &'a str
}

pub enum RequestMethods {
  Get,
  Post,
}

#[derive(Debug)]
pub enum JusibeError {
    InvalidCredentialError,
    BadRequestError
}


impl From<reqwest::Error> for JusibeError {
    fn from(err: reqwest::Error) -> JusibeError {
        match err.status() {
            _BadRequest => JusibeError::BadRequestError,
            Unauthorized => JusibeError::InvalidCredentialError
        }
    }
}

