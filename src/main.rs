use lg_webos_client::command::Command;
use lg_webos_client::client::*;
use std::sync::RwLock;
use lazy_static::lazy_static;
use structopt::StructOpt;
use std::process::exit;

lazy_static! {
    static ref CONFIG: RwLock<WebOsClientConfig> = RwLock::new(WebOsClientConfig::default());
}

#[derive(StructOpt, Debug)]
#[structopt(name = "upstairs-tv", about = "SQLSprinkler")]
struct Opt {
    #[structopt(subcommand)]
    pub commands: Option<Cli>,
}

#[derive(StructOpt, Debug)]
enum Cli {
    Get(GetArgs),
    Set(SetArgs),
}

#[derive(StructOpt, Debug)]
enum GetArgs {
    Vol,
    InputList,
    Mute,
    Mac,
}

#[derive(StructOpt, Debug)]
enum SetArgs {
    Vol(IntOpts),
    Power(BoolOpts),
    Mute(BoolOpts),
}

#[derive(StructOpt, Debug)]
struct IntOpts {
    #[structopt(parse(try_from_str))]
    vol: i8,
}

#[derive(StructOpt, Debug)]
struct BoolOpts {
    #[structopt(parse(try_from_str))]
    state: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config: WebOsClientConfig = confy::load_path("/opt/lgtv/lgtv.conf").unwrap();
    let opt = Opt::from_args();
    let mut res = "".to_string();
    if let Some(subcommand) = opt.commands {
        match subcommand {
            Cli::Get(getargs) => {
                if !check_is_online() {
                    println!("TV is offline.");
                    exit(1);
                }
                let client = match WebosClient::new(config).await {
                    Ok(c) => c,
                    Err(e) => {
                        let formatted = format!("An error occurred: {}", e);
                        println!("{}", formatted);
                        exit(-1);
                    }
                };
                match getargs {
                    GetArgs::Vol => {
                        res = send_command(client, Command::GetVolume).await;
                    }
                    GetArgs::InputList => {
                        res = send_command(client, Command::GetExternalInputList).await;
                    }
                    GetArgs::Mute => {
                        res = send_command(client, Command::IsMuted).await;
                    }
                    GetArgs::Mac => {
                        res = send_command(client, Command::GetNetState).await;
                    }
                }
            }
            Cli::Set(setargs) => {
                match setargs {
                    SetArgs::Vol(vol) => {
                        let client = WebosClient::new(config).await.unwrap();

                        res = send_command(client, Command::SetVolume(vol.vol)).await;
                    }
                    SetArgs::Mute(state) => {
                        let client = WebosClient::new(config).await.unwrap();

                        res = send_command(client, Command::SetMute(state.state)).await;
                    }
                    SetArgs::Power(state) => {
                        if state.state {
                            send_wol_packet();
                        } else {
                            let client = WebosClient::new(config).await.unwrap();
                            res = send_command(client, Command::TurnOff).await;
                        }
                    }
                }
            }
        }
    }
    println!("{}", res)
}

async fn send_command(mut client: WebosClient, command: Command) -> String {
    let data = client.send_command(command).await.unwrap();
    let pl = data.payload.unwrap().to_string();
    pl
}

fn check_is_online() -> bool {
    let mut cmd = std::process::Command::new("ping");
    cmd
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .args(&["10.4.1.51", "-W", "1", "-c", "1"])
        .status()
        .unwrap()
        .success()
}

fn send_wol_packet() {
    let mac_addr: [u8; 6] = [0xec, 0xf4, 0x51, 0x5a, 0xe4, 0xf6];
    let packet = wake_on_lan::MagicPacket::new(&mac_addr);
    packet.send_to("10.4.1.51:9", "0.0.0.0:0").unwrap();
    println!("Sent WOL packet");
}
