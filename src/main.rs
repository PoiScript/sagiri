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
mod handler;
mod database;

use hyper::Client;
use futures::Stream;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use bot::telegram::UpdateStream;
use types::telegram::{Received, Message};

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

  let tg_bot = bot::telegram::Bot::new(TOKEN, client.clone());

  let mut handler = handler::Handler::new(client.clone(), TOKEN.to_string());

  let work = UpdateStream::new(tg_bot.clone())
    .filter_map(|update| match update {
      Received::Message(msg) => Some(msg),
      _ => None,
    })
    .and_then(|res| handler.handle(res))
    .and_then(|message| {
      tg_bot.request::<_, Message>("sendMessage", &message)
    })
    .or_else(|e| {
      error!("Sagiri: {:?}", e);
      Ok::<(), ()>(())
    })
    .for_each(|_| Ok(()));

  core.run(work).unwrap();

  println!("Sagiri Here.");
}
