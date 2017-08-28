pub mod kitsu;
pub mod matrix;
pub mod telegram;

use std::fmt::Display;

use hyper_tls::HttpsConnector;
use hyper::client::{self, HttpConnector};

pub type Client = client::Client<HttpsConnector<HttpConnector>>;

const BASE_URL: &'static str = "https://kitsu.io/api/edge/";

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum DatabaseResponse {
  Ok { data: Vec<User> },
  Error { error: String },
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
  pub kitsu_id: i64,
  pub telegram_id: i64,
  pub kitsu_token: String,
}

pub struct Url {
  url: String,
}

impl Url {
  pub fn new(path: &str) -> Url {
    Url { url: format!("{}{}?", BASE_URL, path) }
  }

  pub fn params<T: Display>(mut self, key: &T, value: &T) -> Self {
    self.url.push_str(&format!("&{}={}", key, value));

    self
  }

  pub fn get_url(self) -> String {
    self.url
  }
}
