use crate::error::Result;
use crate::{Backend, ScrollsClient};
use async_trait::async_trait;
use redis::Commands;

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub epoch: u64,
    pub height: u64,
    pub slot: u64,
    pub block_hash: String,
    pub block_era: String,
    pub first_transaction_hash: String,
    pub last_transaction_hash: String,
    pub transactions_count: u64,
}
#[async_trait]
pub trait LastBlockInfo {
    async fn get_last_block_info(&self) -> Result<BlockInfo>;
}

#[async_trait]
impl LastBlockInfo for ScrollsClient {
    async fn get_last_block_info(&self) -> Result<BlockInfo> {
        match &self.backend {
            Backend::Redis { ip, port } => redis_get_last_block_info(ip, port),
        }
    }
}

fn redis_get_last_block_info(ip: &str, port: &str) -> Result<BlockInfo> {
    let location = format!("redis://{ip}:{port}");
    let client = redis::Client::open(location.as_ref()).unwrap();
    let mut con = client.get_connection().unwrap();
    let current_epoch: u64 = con.get("last_block.epoch_no").unwrap();
    let current_height: u64 = con.get("last_block.height").unwrap();
    let current_slot: u64 = con.get("last_block.slot_no").unwrap();
    let current_block_hash: String = con.get("last_block.block_hash").unwrap();
    let current_block_era: String = con.get("last_block.block_era").unwrap();
    let current_block_first_tx_hash: String = con.get("last_block.first_transaction_hash").unwrap();
    let current_block_last_tx_hash: String = con.get("last_block.last_transaction_hash").unwrap();
    let current_block_last_tx_count: u64 = con.get("last_block.transactions_count").unwrap();
    let block_info = BlockInfo {
        epoch: current_epoch,
        height: current_height,
        slot: current_slot,
        block_hash: current_block_hash,
        block_era: current_block_era,
        first_transaction_hash: current_block_first_tx_hash,
        last_transaction_hash: current_block_last_tx_hash,
        transactions_count: current_block_last_tx_count,
    };
    Ok(block_info)
}
