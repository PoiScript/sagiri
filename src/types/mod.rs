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

#[derive(Clone, Debug, Deserialize)]
pub struct User {
  pub kitsu_id: i64,
  pub telegram_id: i64,
  pub kitsu_token: String,
}

#[derive(Debug)]
pub enum MsgCommand {
  List,
  Update,
}

#[derive(Debug)]
pub enum QueryCommand {
  Offset { kitsu_id: i64, offset: i64 },
  Detail { kitsu_id: i64, anime_id: i64 },
}
