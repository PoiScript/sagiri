pub mod kitsu;
pub mod matrix;
pub mod telegram;

use hyper_tls::HttpsConnector;
use hyper::client::{self, HttpConnector};

pub type Client = client::Client<HttpsConnector<HttpConnector>>;

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum DatabaseResponse {
  Ok { data: Vec<User> },
  Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct User {
  pub kitsu_id: i64,
  pub telegram_id: i64,
  pub kitsu_token: String,
}
