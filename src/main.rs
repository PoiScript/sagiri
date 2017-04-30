extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;

mod matrix;
mod telegram;
mod sagiri;

use sagiri::*;
use matrix::adapter::MatrixAdapter;
use telegram::adapter::TelegramAdapter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    env_logger::init().unwrap();

    if args.len() != 2 {
        error!("Usage: {} CONFIG_FILE", args[0]);
        std::process::exit(1);
    }

    print!("Sagiri Here");

    let sagiri: Sagiri;
}