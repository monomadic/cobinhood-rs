use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {
    /// get server time
    pub fn balances(&self) -> Result<Vec<Balance>, CobinhoodError> {
        let data = self.get("/v1/wallet/balances", "")?;
        let response:APIResponse<Balances> = from_str(data.as_str()).unwrap();

        Ok(response.result.balances)
    }
}
