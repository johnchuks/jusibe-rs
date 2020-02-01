extern crate reqwest;
extern crate serde_json;
mod utils;

use utils::{SMSResponse, SMSCreditResponse, SMSRequestPayload,
    RequestMethods, JusibeError, DeliveryStatusResponse, BulkSMSResponse, BulkStatusResponse};

use reqwest::{StatusCode};


const BASE_URL: &str = "https://jusibe.com/smsapi/";

pub struct Client {
    pub access_token: String,
    pub public_key: String
}


impl Client {
    pub fn new(access_token: &str, public_key: &str) -> Client {
        return Client {
            access_token: access_token.to_string(),
            public_key: public_key.to_string()
        }
    }


    /// send SMS to a single mobile number
    /// # Arguments
    /// * `to` - the phone number to send the SMS
    /// * `from` - a 11 character string reference that represents who sent the message
    /// * `message` - the message body to be sent
    #[tokio::main]
    pub async fn send_sms(&self, to: &str, from: &str, message: &str) -> Result<SMSResponse, JusibeError> {
        let endpoint = "send_sms";
        let url = format!("{}{}", BASE_URL, endpoint);
        let payload = SMSRequestPayload{
            to: to,
            from: from,
            message: message
        };
        self.send_request(RequestMethods::Post, &url, Some(&payload)).await
    }



    /// Retrieve the availabe credits for a user account
    /// # Arguments
    /// Returns the total available sms credits
    #[tokio::main]
    pub async fn available_credits(&self) -> Result<SMSCreditResponse, JusibeError> {
        let endpoint = "get_credits";
        let url = format!("{}{}", BASE_URL, endpoint);

        return self.send_request(RequestMethods::Get, &url, None).await
    }



    ///  Retrieve and check the delivery status sent to a single phone number
    /// # Arguments
    /// * `message_id` - the message ID that was returned when the SMS was sent initially
    #[tokio::main]
    pub async fn delivery_status(&self, message_id: &str) -> Result<DeliveryStatusResponse, JusibeError> {
        let endpoint = "delivery_status";
        let url = format!("{}{}?message_id={}", BASE_URL, endpoint, message_id);

        self.send_request(RequestMethods::Get, &url, None).await
    }



    #[tokio::main]
    pub async fn send_bulk_sms(&self, to: &str, from: &str, message: &str) -> Result<BulkSMSResponse, JusibeError> {
        let endpoint = "bulk/send_sms";
        let url = format!("{}{}", BASE_URL, endpoint);
        let payload = SMSRequestPayload{
            to: to,
            from: from,
            message: message
        };
        self.send_request(RequestMethods::Post, &url, Some(&payload)).await
    }

    
    #[tokio::main]
    pub async fn bulk_delivery_status(&self, bulk_message_id: &str) -> Result<BulkStatusResponse, JusibeError> {
        let endpoint = "bulk/status";
        let url = format!("{}{}?bulk_message_id={}", BASE_URL, endpoint, bulk_message_id);
        
        self.send_request(RequestMethods::Get, &url, None).await
    }
    


    async fn send_request<T>(&self, method: RequestMethods, url: &str, payload: Option<&SMSRequestPayload<'_>>)  -> Result<T, JusibeError> 
    where 
        T: serde::de::DeserializeOwned,
    {
    
            let request = reqwest::Client::new();

            let new_request = match method {
                RequestMethods::Get => request.get(url).basic_auth(&self.public_key, Some(&self.access_token)),
                RequestMethods::Post => request.post(url).basic_auth(&self.public_key, Some(&self.access_token)).json(&payload),
            };

            let response = new_request.send().await?;

            match response.status() {
                StatusCode::OK => (),
                StatusCode::BAD_REQUEST => return Err(JusibeError::BadRequestError),
                StatusCode::UNAUTHORIZED => return Err(JusibeError::InvalidCredentialError),
                _ => ()
            };

            let response_text = response.text().await?;

            let decoded: T = serde_json::from_str(&response_text).unwrap();

            Ok(decoded)
    }
}
