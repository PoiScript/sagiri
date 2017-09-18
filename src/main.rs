#![feature(custom_attribute)]

extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

mod bot;
mod error;
mod kitsu;
mod utils;
mod types;
mod handler;
mod database;

use futures::Stream;
use types::telegram::Update;

fn main() {
  const TOKEN: &'static str = env!("TOKEN");

  env_logger::init().expect("error/init-logger");

  let mut core = tokio_core::reactor::Core::new().expect("error/init-core");
  let handle = core.handle();

  let client = hyper::Client::configure()
    .connector(hyper_tls::HttpsConnector::new(4, &handle).expect("error/create-connector"))
    .build(&handle);

  let tg_bot = bot::telegram::Bot::new(TOKEN, client.clone());

  let mut handler = handler::Handler::new(tg_bot.clone(), client.clone(), TOKEN.to_string());

  let work = bot::telegram::UpdateStream::new(tg_bot)
    .filter_map(|up| match up {
      Update::Message { message, .. } => Some(handler.handle_message(message)),
      Update::CallbackQuery { callback_query, .. } => Some(handler.handle_query(callback_query)),
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
