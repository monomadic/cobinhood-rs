use errors::*;
use reqwest;

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

    pub fn get(&self, endpoint: &str, request: &str) -> Result<(String)> {
        let mut url: String = format!("{}{}", API_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str())?;

        self.handler(response)
    }
}
