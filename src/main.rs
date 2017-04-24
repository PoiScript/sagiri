extern crate hyper;
extern crate hyper_native_tls;

mod telegram;

use telegram::TelegramBot;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    print!("Megumin Here");
}