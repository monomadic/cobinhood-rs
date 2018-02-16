use reqwest;
use reqwest::{Response, StatusCode};

use error::*;
use models::*;

#[derive(Clone)]
pub struct Client {
    api_key: String,
}

static API_HOST: &'static str = "https://api.cobinhood.com";

impl Client {
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.into(),
        }
    }

    pub fn get(&self, endpoint: &str, request: &str) -> Result<String, CobinhoodError> {
        let mut url: String = format!("{}{}", API_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str()).expect("expected get request to be valid");

        self.handler(response)
    }

    fn handler(&self, mut response: Response) -> Result<String, CobinhoodError> {
        use std::io::Read;

        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                Ok(body)
            },
            // error_status => Err(error_status),
            // StatusCode::InternalServerError => {
            //     // bail!("Internal Server Error");
            //     // reqwest::Error {
            //     //     kind: reqwest::error::Kind::f,
            //     //     url: "",
            //     // }

            // }
            // StatusCode::ServiceUnavailable => {
            //     // bail!("Service Unavailable");
            // }
            StatusCode::Unauthorized => {
                Err(CobinhoodError {
                    error_type: CobinhoodErrorType::Unauthorized,
                    message: format!("Unauthorised request."),
                })
            }
            // StatusCode::BadRequest => {
            //     // bail!(format!("Bad Request: {:?}", response));
            // }
            s => {
                Err(CobinhoodError {
                    error_type: CobinhoodErrorType::General,
                    message: format!("Received response: {:?}", s),
                })
            }
        }
    }
}
