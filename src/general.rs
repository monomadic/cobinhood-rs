use model::*;
use client::*;
use errors::*;

use serde_json::from_str;

impl Client {
    /// get server time
    pub fn get_server_time(&self) -> Result<(ServerTime)> {
        let data: String = self.client.get("/v1/system/time", "")?;

        let server_time: ServerTime = from_str(data.as_str()).unwrap();

        Ok(server_time)
    }
}
