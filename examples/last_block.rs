use scrolls_client::LastBlockInfo;
use scrolls_client::ScrollsClient;

#[tokio::main]
async fn main() {
    let ip = "192.168.0.143".to_string();
    let port = "6379".to_string();
    let client = ScrollsClient::new_redis(ip, port);

    let block_info = client.get_last_block_info().await.unwrap();

    println!("{:?}", block_info);
}
