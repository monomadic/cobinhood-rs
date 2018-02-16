use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {
    /// get server time
    pub fn get_server_time(&self) -> Result<u64, CobinhoodError> {
        let data = self.get("/v1/system/time", "")?;
        let response:APIResponse<ServerTime> = from_str(data.as_str()).unwrap();

        Ok(response.result.time)
    }

    pub fn get_server_info(&self) -> Result<ServerInfo, CobinhoodError> {
        let data = self.get("/v1/system/info", "")?;
        let response:APIResponse<ServerInfo> = from_str(data.as_str()).unwrap();

        Ok(response.result)
    }
}
