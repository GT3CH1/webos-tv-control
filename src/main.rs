use lg_webos_client::WebosClient;
use lg_webos_client::Command;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut client = WebosClient::new("ws://10.4.1.51:3000/").await.unwrap();
    let resp = client.send_command(Command::GetChannelList).await.unwrap();
    println!("Got response {:?}", resp.await.payload);
}