use std::collections::HashMap as Map;

use serde_json::Value;

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

#[derive(Serialize, Deserialize)]
pub struct Request {
  pub data: RequestData,
}

impl Request {
  pub fn update_anime(user_id: String, anime_id: String, progress: i32) -> Request {
    Request {
      data: RequestData {
        id: user_id,
        attributes: Attributes { progress: progress },
        relate: {
          let mut relate = Map::new();
          relate.insert(
            RelateType::Anime,
            Relate {
              data: RelateData {
                id: anime_id,
                _type: RelateType::Anime,
              }
            },
          );
          relate
        },
        _type: RequestType::LibraryEntries,
      },
    }
  }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Empty;

#[derive(Serialize, Deserialize)]
pub struct RequestData {
  pub id: String,
  #[serde(rename = "type")]
  pub _type: RequestType,
  pub attributes: Attributes,
  #[serde(rename = "relationships")]
  pub relate: Map<RelateType, Relate>,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
  pub progress: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Relate {
  pub data: RelateData
}

#[derive(Serialize, Deserialize)]
pub struct RelateData {
  pub id: String,
  #[serde(rename = "type")]
  pub _type: RelateType,
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelateType {
  Anime,
  Manga,
}

#[derive(Serialize, Deserialize)]
pub enum RequestType {
  #[serde(rename = "library-entries")]
  LibraryEntries,
}
