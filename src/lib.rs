use crate::Backend::Redis;
use serde_aux::prelude::deserialize_number_from_string;

use serde::Deserialize;

pub mod error;
pub mod last_block_info;
pub mod utxos_by_address;

pub use error::*;
pub use last_block_info::LastBlockInfo;
pub use utxos_by_address::UTxOsByAddress;

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
