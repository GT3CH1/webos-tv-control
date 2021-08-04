use lg_webos_client::command::Command;
use lg_webos_client::client::*;
use std::sync::RwLock;
use lazy_static::lazy_static;
use structopt::StructOpt;
use std::str::FromStr;
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
    Vol(VolOpts)
}

#[derive(StructOpt, Debug)]
struct VolOpts {
    vol: i8,
}

impl FromStr for VolOpts {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i8::from_str(s) {
            Ok(res) => Ok(VolOpts { vol: res }),
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