use lg_webos_client::command::Command;
use lg_webos_client::client::*;
use std::sync::RwLock;
use warp::{Filter, http};
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::time::Duration;
use std::str::FromStr;
use structopt::StructOpt;

lazy_static! {
    static ref CONFIG: RwLock<WebOsClientConfig> = RwLock::new(WebOsClientConfig::default());
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cmd")]
struct Opt {
    #[structopt(name = "arg")]
    argument: Argument,
}

#[derive(Debug,StructOpt)]
enum Argument {
    SetVol {
        volume: i8
    },
    GetVol,
}

impl FromStr for Argument {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "setvol" => Ok(Argument::SetVol),
            "getvol" => Ok(Argument::GetVol),
            _ => { panic!("could not match") }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config: WebOsClientConfig = confy::load_path("lgtv.conf").unwrap();
    let client = WebosClient::new(config).await.unwrap();
    let opt = Opt::from_args();
    println!("{:?}", opt)
}