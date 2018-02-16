#![allow(dead_code)]
#![allow(unused_variables)]

use ::reqwest::StatusCode;
impl From<StatusCode> for CobinhoodError {
    fn from(error: ::reqwest::StatusCode) -> Self {
        match error {
            StatusCode::Unauthorized => {
                CobinhoodError {
                    error_type: CobinhoodErrorType::Unauthorized,
                    message: format!("Unauthorised request."),
                }
            },
            status => {
               CobinhoodError {
                    error_type: CobinhoodErrorType::General,
                    message: format!("Received response: {:?}", status),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CobinhoodError {
    pub error_type: CobinhoodErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum CobinhoodErrorType {
    General,
    Unauthorized,
}
