use lg_webos_client::command::Command;
use lg_webos_client::client::*;
use std::sync::RwLock;
use lazy_static::lazy_static;
use structopt::StructOpt;
use std::str::{FromStr, ParseBoolError};
use std::num::ParseIntError;

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
}

#[derive(StructOpt, Debug)]
enum SetArgs {
    Vol(IntOpts),
    Power(BoolOpts),
}

#[derive(StructOpt, Debug)]
struct IntOpts {
    vol: i8,
}

#[derive(StructOpt, Debug)]
struct BoolOpts {
    state: bool,
}

impl FromStr for BoolOpts {
    type Err = ParseBoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match bool::from_str(s) {
            Ok(res) => Ok(BoolOpts { state: res }),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for IntOpts {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i8::from_str(s) {
            Ok(res) => Ok(IntOpts { vol: res }),
            Err(e) => Err(e)
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config: WebOsClientConfig = confy::load_path("lgtv.conf").unwrap();
    let mut client = WebosClient::new(config).await.unwrap();
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
                }
            }
            Cli::Set(setargs) => {
                match setargs {
                    SetArgs::Vol(vol) => {
                        res = send_command(client, Command::SetVolume(vol.vol)).await;
                    }
                    SetArgs::Power(opts) => {
                        if !opts.state {
                            res = send_command(client, Command::TurnOff).await;
                        } else {
                            todo!("Not implemented")
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