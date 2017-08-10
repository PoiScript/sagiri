#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate serde;
extern crate hyper;
extern crate hyper_tls;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod bot;
mod error;
mod kitsu;
mod database;

fn main() {
  println!("Sagiri Here.")
}
