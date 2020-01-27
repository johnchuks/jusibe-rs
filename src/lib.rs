extern crate reqwest;
extern crate serde_json;
mod message;

use message::{SMSResponse, SMSCreditResponse, SMSRequestPayload, RequestMethods};
use reqwest::{Error, Response};


const BASE_URL: &str = "https://jusibe.com/smsapi/";

pub struct Client {
    pub access_token: String,
    pub public_key: String
}

// enum definition for different kinds exceptions that can be thrown for unsuccessful request
#[derive(Debug)]
pub enum JusibeException {
    ConnectionError,
    BadRequestError,
    InvalidCredentialError
}


impl Client {
    pub fn new(access_token: &str, public_key: &str) -> Client {
        return Client {
            access_token: access_token.to_string(),
            public_key: public_key.to_string()
        }
    }
    
    /// send SMS to a single mobile number
    pub fn send_sms(&self, to: &str, from: &str, message: &str) -> String {
        let endpoint = "send_sms";
        let url = format!("{}{}", BASE_URL, endpoint);
        let payload = SMSRequestPayload{
            to: to,
            from: from,
            message: message
        };
        let response = self.send_request(RequestMethods::Post, &url, Some(&payload));
        let sms_response = &SMSResponse{
            status: "true",
            message_id: "234",
            credit_used: 23
        };
        println!("Jusibe response = {:?}", response);
        let serialized = serde_json::to_string(sms_response).unwrap();
        return serialized;
    }

    pub fn available_credits(&self) -> Result<String, Error> {
        let endpoint = "get_credits";
        let url = format!("{}{}", BASE_URL, endpoint);

        return self.send_request(RequestMethods::Get, &url, None);
        // println!("Jusibe response = {:?}", response);
    }

    #[tokio::main]
    async fn send_request<T>(&self, method: RequestMethods, url: &str, payload: Option<&SMSRequestPayload<'_>>)  -> Result<String, Error> {
    
        let request = reqwest::Client::new();

        println!("{} --- {}", self.public_key, self.access_token);
        println!("{:?}", payload);

        let new_request = match method {
            RequestMethods::Get => request.get(url).basic_auth(&self.public_key, Some(&self.access_token)),
            RequestMethods::Post => request.post(url).basic_auth(&self.public_key, Some(&self.access_token)).json(&payload),
        };
        let response = new_request.send().await?.text().await?;
        println!("we are here {}", response);
        Ok(response)
    }
}



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
