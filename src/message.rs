extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMSResponse <'a> {
  pub status: &'a str,
  pub message_id: &'a str,
  pub credit_used: u32
}

#[derive(Serialize, Deserialize)]
pub struct SMSCreditResponse <'a>{
  credits: &'a str
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

