use std::str::FromStr;

use futures::{future, Future, Stream};

use hyper::mime::Mime;
use hyper::{Uri, Method, Request};
use hyper::header::{ContentType, ContentLength};

use serde_json::Value;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_value, from_slice, to_string};

use types::Client;
use error::{Error, KitsuError};
use types::kitsu::{Response, Request as KitsuRequest};

#[derive(Clone)]
pub struct Api {
  client: Client,
  base_url: &'static str,
}

impl Api {
  pub fn new(client: Client) -> Api {
    Api {
      client: client,
      base_url: "https://kitsu.io/api/edge/",
    }
  }

  fn request<S, T>(
    &self,
    uri: Uri,
    data: &S,
    method: Method,
  ) -> Box<Future<Item = T, Error = Error>>
  where
    S: Serialize,
    T: DeserializeOwned + 'static,
  {
    let json = to_string(data).expect("error/json-to-string");

    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    Box::new(self.client.request(req).from_err::<Error>().and_then(
      |res| {
        res
          .body()
          .from_err::<Error>()
          .concat2()
          .and_then(|chunks| {
            future::result::<Response, Error>(from_slice(&chunks).map_err(|e| e.into()))
          })
          .and_then(|response| match response {
            Response::Ok { data } => from_value(data).map_err(|e| e.into()),

            Response::Error { errors } => {
              return Err(Error::Kitsu(KitsuError { errors }));
            }
          })
      },
    ))
  }

  pub fn update_anime(
    &self,
    progress: i32,
    user_id: String,
    anime_id: String,
  ) -> Box<Future<Item = Value, Error = Error>> {
    let uri = Uri::from_str(&format!(
      "{}{}/{}",
      self.base_url,
      "library-entries",
      &user_id
    )).expect("error/build-uri");
    let data = KitsuRequest::update_anime(user_id, anime_id, progress);

    let json = to_string(&data).expect("error/json-to-string");

    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    Box::new(self.client.request(req).from_err::<Error>().and_then(
      |res| {
        res
          .body()
          .from_err::<Error>()
          .concat2()
          .and_then(|chunks| {
            future::result::<Response, Error>(from_slice(&chunks).map_err(|e| e.into()))
          })
          .and_then(|response| match response {
            Response::Ok { data } => from_value(data).map_err(|e| e.into()),

            Response::Error { errors } => {
              return Err(Error::Kitsu(KitsuError { errors }));
            }
          })
      },
    ))
  }

  pub fn fetch_anime(
    &self,
    page: String,
    user_id: String,
  ) -> Box<Future<Item = Value, Error = Error>> {
    let uri = Uri::from_str("https://kitsu.io/api/edge/library-entries?fields[libraryEntries]=progress,status,updatedAt,anime&fields[anime]=canonicalTitle,titles,episodeCount,slug,subtype&fields&filter[user_id]=140033&filter[status]=current,planned&include=anime&page[limit]=12&sort=status,-progressed_at,-updated_at").unwrap();

    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType(
      Mime::from_str("application/vnd.api+json").unwrap(),
    ));

    Box::new(self.client.request(req).from_err::<Error>().and_then(
      |res| {
        res
          .body()
          .from_err::<Error>()
          .concat2()
          .and_then(|chunks| {
            future::result::<Response, Error>(from_slice(&chunks).map_err(|e| e.into()))
          })
          .and_then(|response| match response {
            Response::Ok { data } => from_value(data).map_err(|e| e.into()),

            Response::Error { errors } => {
              return Err(Error::Kitsu(KitsuError { errors }));
            }
          })
      },
    ))
  }
}
