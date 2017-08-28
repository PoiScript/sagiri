use std::collections::HashMap as Map;

use error::{Error, KitsuError};
use serde_json::Value;
use serde::Deserialize;

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Response {
  Ok {
    data: Vec<Value>,
    included: Option<Vec<Value>>,
    meta: Option<Meta>,
  },
  Error { errors: Vec<ApiError> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  pub title: String,
  pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
  Anime,
  Users,
  Manga,
  LibraryEntries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
  id: String,
  #[serde(rename = "type")]
  kind: Type,
  pub attributes: AnimeAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeAttributes {
  pub canonical_title: String,
  pub episode_count: Option<u32>,
  pub subtype: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  id: i32,
  #[serde(rename = "type")]
  kind: Type,
  attributes: UserAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
  name: String,
  life_spent_on_anime: i32,
  title_language_preference: String,
}

#[derive(Debug, Deserialize)]
pub struct Entries {
  id: String,
  #[serde(rename = "type")]
  kind: Type,
  pub attributes: EntriesAttributes,
}

#[derive(Debug, Deserialize)]
pub struct EntriesAttributes {
  pub progress: i32,
  pub status: EntriesStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntriesStatus {
  OnHold,
  Current,
  Dropped,
  Planned,
  Completed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
  count: i32,
  status_counts: StatusCounts,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCounts {
  current: i32,
  dropped: i32,
  on_hold: i32,
  planned: i32,
  completed: i32,
}

// TODO
//#[derive(Serialize, Deserialize)]
//pub struct Request {
//  pub data: RequestData,
//}
//
//impl Request {
//  pub fn update_anime(user_id: String, anime_id: String, progress: i32) -> Request {
//    Request {
//      data: RequestData {
//        id: user_id,
//        attributes: Attributes { progress: progress },
//        relate: {
//          let mut relate = Map::new();
//          relate.insert(
//            RelateType::Anime,
//            Relate {
//              data: RelateData {
//                id: anime_id,
//                _type: RelateType::Anime,
//              },
//            },
//          );
//          relate
//        },
//        _type: RequestType::LibraryEntries,
//      },
//    }
//  }
//}
//
//
//#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
//pub struct Empty;
//
//#[derive(Serialize, Deserialize)]
//pub struct RequestData {
//  pub id: String,
//  #[serde(rename = "type")]
//  pub _type: RequestType,
//  pub attributes: Attributes,
//  #[serde(rename = "relationships")]
//  pub relate: Map<RelateType, Relate>,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Attributes {
//  pub progress: i32,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Relate {
//  pub data: RelateData,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct RelateData {
//  pub id: String,
//  #[serde(rename = "type")]
//  pub _type: RelateType,
//}
//
//#[derive(Hash, Eq, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "lowercase")]
//pub enum RelateType {
//  Anime,
//  Manga,
//}
