use lg_webos_client::command::Command;
use lg_webos_client::client::*;
use std::sync::RwLock;
use lazy_static::lazy_static;
use structopt::StructOpt;

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
    let client = WebosClient::new(config).await.unwrap();
    let opt = Opt::from_args();
    let mut res = "".to_string();
    if let Some(subcommand) = opt.commands {
        match subcommand {
            Cli::Get(getargs) => {
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
                }
            }
            Cli::Set(setargs) => {
                match setargs {
                    SetArgs::Vol(vol) => {
                        res = send_command(client, Command::SetVolume(vol.vol)).await;
                    }
                    SetArgs::Mute(state) => {
                        res = send_command(client, Command::SetMute(state.state)).await;
                    }
                    SetArgs::Power(state) => {
                        if !state.state {
                            res = send_command(client, Command::TurnOff).await;
                        } else {
                            send_wol_packet();
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

fn send_wol_packet() {
    let mac_addr: [u8; 6] = [0xe0, 0xd5, 0x5e, 0x26, 0x8a, 0x1b];
    let packet = wake_on_lan::MagicPacket::new(&mac_addr);
    packet.send_to("10.4.1.51", "127.0.1.1");
    println!("Sent WOL packet");
}
