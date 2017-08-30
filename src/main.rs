#![feature(custom_attribute)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate url;
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

use bot::telegram::{Bot, UpdateStream};
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

  let tg_bot = Bot::new(TOKEN, client.clone());

  let mut handler = handler::Handler::new(tg_bot.clone(), client.clone(), TOKEN.to_string());

  let work = UpdateStream::new(tg_bot)
    .filter_map(|update| match update {
      Received::Message(msg) => Some(handler.handle_message(msg)),
      Received::CallbackQuery(query) => Some(handler.handle_query(query)),
      _ => None,
    })
    .and_then(|f| f)
    .map(|_| ())
    .or_else(|e| {
      error!("{:?}", e);
      Ok::<(), ()>(())
    })
    .for_each(|_| Ok(()));

  core.run(work).unwrap();

  println!("Sagiri Here.");
}
