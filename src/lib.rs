extern crate reqwest;
mod message;

use message::{SMSResponse, SMSCreditResponse, SMSRequestPayload, RequestMethods};


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
    

    pub async fn send_sms(&self, to: &str, from: &str, message: &str) -> &SMSResponse<'_> {
        let endpoint = "send_sms";
        let url = format!("{}/{}", BASE_URL, endpoint);
        let payload = SMSRequestPayload{
            to: to,
            from: from,
            message: message
        };

        // let request = reqwest::Client::new();
        // let response = request.post(url)
        //     .json(&payload)
        //     .send()
        //     .await?;
        
        let sms_response = &SMSResponse{
            status: "true",
            message_id: "234",
            credit_used: 23
        };
        return sms_response
    }

    #[tokio::main]
    async fn send_request(&self, method: RequestMethods, url: &str, payload: &SMSRequestPayload<'_>)  -> Result<(), reqwest::Error> {
    
        let request = reqwest::Client::new();

        let new_request = match method {
            RequestMethods::Get => request.get(url),
            RequestMethods::Post => request.post(url).json(&payload),
        };

        let response = new_request.send().await?;
        println!("Jusibe response = {:?}", response);
        Ok(())
    }
}



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
