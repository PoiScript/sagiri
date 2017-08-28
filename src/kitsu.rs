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
use types::kitsu::Response;
use types::kitsu::Anime;

use types::telegram::Message;

#[derive(Clone)]
pub struct Api {
  client: Client,
}

impl Api {
  pub fn new(client: Client) -> Api {
    Api { client: client }
  }

  fn request(&self, req: Request) -> Box<Future<Item = Response, Error = Error>> {
    Box::new(self.client.request(req).from_err::<Error>().and_then(
      |res| {
        res.body().from_err::<Error>().concat2().and_then(|chunks| {
          future::result::<Response, Error>(from_slice(&chunks).map_err(|e| e.into()))
        })
      },
    ))
  }

  // TODO
  //  pub fn update_anime(
  //    &self,
  //    progress: i32,
  //    user_id: String,
  //    anime_id: String,
  //  ) -> Box<Future<Item = Value, Error = Error>> {
  //    let uri = Uri::from_str(&format!(
  //      "{}{}/{}",
  //      self.base_url,
  //      "library-entries",
  //      &user_id
  //    )).expect("error/build-uri");
  //    let data = KitsuRequest::update_anime(user_id, anime_id, progress);
  //
  //    let json = to_string(&data).expect("error/json-to-string");
  //
  //    let mut req = Request::new(Method::Post, uri);
  //    req.headers_mut().set(ContentType::json());
  //    req.headers_mut().set(ContentLength(json.len() as u64));
  //    req.set_body(json);
  //
  //    Box::new(self.client.request(req).from_err::<Error>().and_then(
  //      |res| {
  //        res
  //          .body()
  //          .from_err::<Error>()
  //          .concat2()
  //          .and_then(|chunks| {
  //            future::result::<Response, Error>(from_slice(&chunks).map_err(|e| e.into()))
  //          })
  //          .and_then(|response| match response {
  //            Response::Ok { data } => from_value(data).map_err(|e| e.into()),
  //
  //            Response::Error { errors } => {
  //              return Err(Error::Kitsu(KitsuError { errors }));
  //            }
  //          })
  //      },
  //    ))
  //  }

  pub fn fetch_anime(
    &self,
    url: String,
    msg: Message,
  ) -> Box<Future<Item = (Option<Vec<Value>>, Message), Error = Error>> {
    let uri = Uri::from_str(&url).unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType(
      Mime::from_str("application/vnd.api+json").unwrap(),
    ));

    Box::new(self.request(req).and_then(|response| match response {
      Response::Ok { included, .. } => Ok((included, msg)),

      Response::Error { errors } => {
        return Err(Error::Kitsu(KitsuError {
          description: format!("{}: {}", errors[0].title, errors[1].detail),
        }));
      }
    }))
  }
}
