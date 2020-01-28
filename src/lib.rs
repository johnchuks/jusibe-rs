extern crate reqwest;
extern crate serde_json;
mod message;

use message::{SMSResponse, SMSCreditResponse, SMSRequestPayload, RequestMethods, JusibeError};
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
    #[tokio::main]
    pub async fn available_credits(&self) -> Result<SMSCreditResponse, JusibeError> {
        let endpoint = "get_credits";
        let url = format!("{}{}", BASE_URL, endpoint);

        return self.send_request(RequestMethods::Get, &url, None).await
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



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
