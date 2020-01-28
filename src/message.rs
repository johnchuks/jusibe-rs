extern crate serde;
extern crate reqwest;

use serde::{Serialize, Deserialize};
use reqwest::StatusCode;

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
  BadRequestError,
  NoError,
  RequestError
}


impl From<reqwest::Error> for JusibeError {
    fn from(err: reqwest::Error) -> JusibeError {
        match err.status() {
            Some(StatusCode::BAD_REQUEST) => JusibeError::BadRequestError,
            Some(StatusCode::UNAUTHORIZED) => JusibeError::InvalidCredentialError,
            None => JusibeError::NoError,
            _  => JusibeError::RequestError
        } 
    }
}

