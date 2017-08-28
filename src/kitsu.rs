use std::str::FromStr;

use futures::{future, Future, Stream};

use hyper::mime::Mime;
use hyper::{Uri, Method, Request};
use hyper::header::ContentType;

use serde_json::{from_value, from_slice};

use types::{Client, Url};
use error::{Error, KitsuError};
use types::kitsu::{Anime, Entries, Response};

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
    chat_id: i64,
    user_id: i64,
  ) -> Box<Future<Item = (Vec<Entries>, Option<Vec<Anime>>, i64), Error = Error>> {
    let url = Url::new("library-entries")
      .params("include", "anime")
      .params("filter[user_id]", &user_id.to_string())
      .params("filter[status]", "current,planned")
      .params("fields[libraryEntries]", "progress,status,updatedAt,anime")
      .params(
        "fields[anime]",
        "canonicalTitle,titles,episodeCount,slug,subtype",
      ).get_url();

    let uri = Uri::from_str(&url).unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType(
      Mime::from_str("application/vnd.api+json").unwrap(),
    ));

    Box::new(
      self
        .request(req)
        .and_then(|response| match response {
          Response::Ok { data, included, .. } => {
            Ok((
              data.into_iter().map(|v| from_value(v).unwrap()).collect(),
              included.map(|v| {
                v.into_iter().map(|v| from_value(v).unwrap()).collect()
              }),
            ))
          }

          Response::Error { errors } => {
            return Err(Error::Kitsu(KitsuError {
              description: format!("{}: {}", errors[0].title, errors[1].detail),
            }));
          }
        })
        .and_then(move |(data, included)| Ok((data, included, chat_id))),
    )
  }
}
