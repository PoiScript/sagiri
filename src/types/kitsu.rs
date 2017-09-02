use std::fmt;
use std::default::Default;

use serde_json::Value;

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Response {
  Ok {
    data: Vec<Value>,
    included: Option<Vec<Value>>,
    meta: Option<Meta>,
    links: Links,
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
  pub id: String,
  pub attributes: AnimeAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeAttributes {
  pub canonical_title: String,
  pub episode_count: Option<u32>,
  pub status: Option<AnimeStatus>,
  pub subtype: Option<AnimeSubtype>,
  pub titles: AnimeTitles,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnimeSubtype {
  ONA,
  OVA,
  TV,
  #[serde(rename = "movie")]
  Movie,
  #[serde(rename = "music")]
  Music,
  #[serde(rename = "special")]
  Special,
  Unknown,
}

impl Default for AnimeSubtype {
  fn default() -> AnimeSubtype {
    AnimeSubtype::Unknown
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimeStatus {
  Current,
  Finished,
  Tba,
  Unreleased,
  Upcoming,
  Unknown,
}

impl Default for AnimeStatus {
  fn default() -> AnimeStatus {
    AnimeStatus::Unknown
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeTitles {
  pub ja_jp: Option<String>,
}

impl fmt::Display for AnimeTitles {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.ja_jp {
      Some(ref title) => write!(f, "(<i>{}</i>)", title),
      None => write!(f, ""),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  id: i32,
  pub attributes: UserAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
  pub name: String,
  pub life_spent_on_anime: i32,
  pub title_language_preference: String,
}

#[derive(Debug, Deserialize)]
pub struct Entries {
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
  pub count: i32,
  pub status_counts: StatusCounts,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCounts {
  current: Option<i32>,
  dropped: Option<i32>,
  on_hold: Option<i32>,
  planned: Option<i32>,
  completed: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
  pub prev: Option<String>,
  pub next: Option<String>,
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
