pub mod kitsu;
pub mod matrix;
pub mod telegram;

use hyper_tls::HttpsConnector;
use hyper::client::{self, HttpConnector};

pub type Client = client::Client<HttpsConnector<HttpConnector>>;

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum  DatabaseResponse {
  Ok { data: Vec<User> },
  Error { error: String }
}

#[derive(Debug, Deserialize)]
pub struct User {
  kitsu_id: i32,
  telegram_id: i32,
  kitsu_token: String
}
