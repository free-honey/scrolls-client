use crate::Backend::Redis;
use async_trait::async_trait;
use error::Result;
use pallas_addresses::Address;
use redis;
use redis::Commands;
use serde_aux::prelude::deserialize_number_from_string;

use serde::Deserialize;

pub mod error;

pub enum Backend {
    Redis { ip: String, port: String },
}

pub struct ScrollsClient {
    backend: Backend,
}

impl ScrollsClient {
    pub fn new_redis(ip: String, port: String) -> Self {
        let backend = Redis { ip, port };
        ScrollsClient { backend }
    }
}

#[derive(Debug, Deserialize)]
pub struct UTxO {
    amount: Vec<Amount>,
    output_index: u64,
    tx_hash: String,
    datum: Option<String>,
    datum_hash: Option<String>,
}

impl UTxO {
    pub fn amount(&self) -> &Vec<Amount> {
        &self.amount
    }

    pub fn output_index(&self) -> u64 {
        self.output_index
    }

    pub fn tx_hash(&self) -> &str {
        &self.tx_hash
    }

    pub fn datum(&self) -> Option<&str> {
        self.datum.as_ref().map(|s| s.as_ref())
    }

    pub fn datum_hash(&self) -> Option<&str> {
        self.datum_hash.as_ref().map(|s| s.as_ref())
    }
}

#[derive(Debug, Deserialize)]
pub struct Amount {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    quantity: u64,
    unit: String,
}

impl Amount {
    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn unit(&self) -> &str {
        &self.unit
    }
}

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
