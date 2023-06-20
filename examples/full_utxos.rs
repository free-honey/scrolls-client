use pallas_addresses::Address;
use redis;
use redis::Commands;
use scrolls_client::ScrollsClient;
use scrolls_client::UTxOsByAddress;

#[tokio::main]
async fn main() {
    let ip = "192.168.0.143".to_string();
    let port = "6379".to_string();
    let client = ScrollsClient::new_redis(ip, port);

    let address_str = "addr_test1qp7dqz7g6nyg0y08np42aj8magcwdgr8ea6mysa7e9f6qg8hdg3rkwaqkqysqnwqsfl2spx4yreqywa6t5mgftv6x3fsckw6qg";
    let address = Address::from_bech32(address_str).unwrap();
    let outputs = client.get_utxos_for_address(&address).await.unwrap();

    println!("outputs:");
    println!();
    for utxo in outputs {
        println!(
            "index: {:?}, tx_hash: {:?}",
            utxo.output_index(),
            utxo.tx_hash()
        );
        for amount in utxo.amount() {
            println!("{:?}", amount);
        }
        println!();
    }
}
