use pallas_addresses::Address;
use redis;
use scrolls_client::ScrollsClient;
use scrolls_client::UTxOsByAddress;

#[tokio::main]
async fn main() {
    let ip = "192.168.0.143".to_string();
    let port = "6379".to_string();
    let client = ScrollsClient::new_redis(ip, port);

    // let address_str = "addr_test1qp7dqz7g6nyg0y08np42aj8magcwdgr8ea6mysa7e9f6qg8hdg3rkwaqkqysqnwqsfl2spx4yreqywa6t5mgftv6x3fsckw6qg";
    let address_str = "addr_test1wq6t9y9k20wp545s2snkt5222vhhwt40p8mqt8pad6xtdnsq95tm0";
    let address = Address::from_bech32(address_str).unwrap();
    let outputs = client.get_utxos_for_address(&address).await.unwrap();

    println!("outputs:");
    println!();
    for utxo in outputs {
        println!(
            "index: {:?}, tx_hash: {:?}, datum: {:?}",
            utxo.output_index(),
            utxo.tx_hash(),
            utxo.datum(),
        );
        for amount in utxo.amount() {
            let quantity = amount.quantity();
            let unit = amount.unit().to_string();
            if unit == "lovelace" {
                println!("quantity: {:?}, unit: {:?}", quantity, unit);
            } else {
                let policy_id = &unit[..56];
                let token_name = &unit[56..];
                let token_name_str: String =
                    String::from_utf8(hex::decode(token_name).unwrap()).unwrap();
                println!(
                    "quantity: {:?}, policy_id: {:?}, token_name: {:?}",
                    quantity, policy_id, token_name_str
                );
            }
        }
        println!();
    }
}
