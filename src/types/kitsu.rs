use serde_json::Value;
use error::{Error, KitsuError};

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Response {
  Ok { data: Value },
  Error { errors: Vec<ApiError> },
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
  pub title: String,
  pub detail: String,
  code: String,
  status: String,
}
