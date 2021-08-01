use lg_webos_client::command::Command;
use lg_webos_client::client::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = WebOsClientConfig::new("ws://10.4.1.51:3000/", true, "df483773b34d4919c29ba348728f5987");
    let mut client = WebosClient::new(config).await.unwrap();
    let resp = client.send_command(Command::GetChannelList).await.unwrap();
    println!("Got response {:?}", resp.payload);
}