extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod matrix;
mod telegram;
mod webhook;

use webhook::webhook::WebHook;
use matrix::bot::MatrixBot;
use telegram::bot::TelegramBot;

fn main() {
    print!("Sagiri Here");
//
//    let tg_bot = TelegramBot::new("token");
//    let mx_bot = MatrixBot::new("http://matrix.org", "token");
//
//    let webhook = WebHook::new(mx_bot, tg_bot);
//
//    webhook.start();
}