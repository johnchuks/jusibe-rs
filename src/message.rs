extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMSResponse {
  pub status: String,
  pub message_id: String,
  pub credit_used: u32
}

#[derive(Serialize, Deserialize)]
pub struct SMSCreditResponse {
  credits: String
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


