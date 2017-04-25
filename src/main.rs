extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod telegram;

use telegram::bot::TelegramBot;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    print!("Sagiri Here");
}