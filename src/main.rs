#![feature(custom_attribute)]

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

use hyper::Client;
use futures::{Future, Stream};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use bot::telegram::UpdateStream;
use types::telegram::Received;

fn main() {
  const TOKEN: &'static str = env!("TOKEN");

  env_logger::init().expect("error/env-logger");

  let mut core = Core::new().expect("error/init-core");
  let handle = core.handle();

  let client = Client::configure()
    .connector(HttpsConnector::new(4, &handle).expect(
      "error/create-connector",
    ))
    .build(&handle);

  let api = kitsu::Api::new(client.clone());
  let tg_bot = bot::telegram::Bot::new(TOKEN, client.clone());
  let mut db = database::Database::new(TOKEN.to_string(), client);

  let work = UpdateStream::new(tg_bot.clone())
    .filter_map(|update| match update {
      Received::Message(msg) => Some(msg),
      _ => None,
    })
    .and_then(|msg| {
      println!("{:?}", msg);
      Ok(())
    })
    .for_each(|_| Ok(()));


  core.run(work).unwrap();

  println!("Sagiri Here.");
}
