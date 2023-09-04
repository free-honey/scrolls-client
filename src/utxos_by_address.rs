use crate::error::Result;
use crate::{Backend, ScrollsClient, UTxO};
use async_trait::async_trait;
use pallas_addresses::Address;
use redis::Commands;

#[async_trait]
pub trait UTxOsByAddress {
    async fn get_utxos_for_address(&self, address: &Address) -> Result<Vec<UTxO>>;
}

#[async_trait]
impl UTxOsByAddress for ScrollsClient {
    async fn get_utxos_for_address(&self, address: &Address) -> Result<Vec<UTxO>> {
        match &self.backend {
            Backend::Redis { ip, port } => redis_get_utxos_for_address(address, ip, port),
        }
    }
}

fn redis_get_utxos_for_address(address: &Address, ip: &str, port: &str) -> Result<Vec<UTxO>> {
    let location = format!("redis://{ip}:{port}");
    let client = redis::Client::open(location.as_ref()).unwrap();
    let mut con = client.get_connection().unwrap();
    let key = address.to_string();
    let outputs_raw: Vec<String> = con.smembers(&key).unwrap();
    let outputs = outputs_raw
        .iter()
        .map(|o| serde_json::from_str(o).unwrap())
        .collect();
    Ok(outputs)
}
