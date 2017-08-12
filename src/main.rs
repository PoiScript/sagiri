#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate serde;
extern crate hyper;
extern crate futures;
extern crate hyper_tls;
extern crate env_logger;
extern crate tokio_core;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod bot;
mod error;
mod kitsu;
mod types;
mod database;

fn main() {
  println!("Sagiri Here.")
}
