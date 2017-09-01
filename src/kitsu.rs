use std::str::FromStr;

use url::Url;

use futures::{future, Future, Stream};

use hyper::mime::Mime;
use hyper::{Uri, Method, Request};
use hyper::header::ContentType;

use serde_json::{from_value, from_slice};

use types::Client;
use error::{Error, KitsuError};
use types::kitsu::{Anime, Entries, Response};

#[derive(Clone)]
pub struct Api {
  base: Url,
  client: Client,
}

impl Api {
  pub fn new(client: Client) -> Api {
    Api {
      base: Url::parse("https://kitsu.io/api/edge/").unwrap(),
      client,
    }
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
    user_id: i64,
    offset: i64,
  ) -> Box<
    Future<
      Item = (Option<String>, Option<String>, Option<(Vec<Entries>, Vec<Anime>)>),
      Error = Error,
    >,
  > {
    let mut endpoint = self.base.join("library-entries").unwrap();

    let url = endpoint
      .query_pairs_mut()
      .append_pair("include", "anime")
      .append_pair("page[limit]", "5")
      .append_pair("page[offset]", &offset.to_string())
      .append_pair("filter[user_id]", &user_id.to_string())
      .append_pair("filter[status]", "current,planned")
      .append_pair("fields[libraryEntries]", "progress,status,updatedAt,anime")
      .append_pair(
        "fields[anime]",
        "canonicalTitle,titles,episodeCount,slug,subtype",
      )
      .finish()
      .as_str();

    let uri = Uri::from_str(url).unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType(
      Mime::from_str("application/vnd.api+json").unwrap(),
    ));

    Box::new(
      self
        .request(req)
        .and_then(|response| match response {
          Response::Ok { data, included, links, .. } => {
            Ok((
              data.into_iter().map(|v| from_value(v).unwrap()).collect(),
              included.map(|v| {
                v.into_iter().map(|v| from_value(v).unwrap()).collect()
              }),
              links,
            ))
          }

          Response::Error { errors } => {
            Err(Error::Kitsu(KitsuError {
              description: format!("{}: {}", errors[0].title, errors[1].detail),
            }))
          }
        })
        .and_then(|(entries, included, links)| match included {
          None => Ok((None, None, None)),
          Some(animes) => Ok((links.prev, links.next, Some((entries, animes)))),
        }),
    )
  }

  pub fn get_anime(
    &self,
    user_id: i64,
    anime_id: i64,
  ) -> Box<Future<Item = Option<(Entries, Anime)>, Error = Error>> {
    let mut endpoint = self.base.join("library-entries").unwrap();

    let url = endpoint
      .query_pairs_mut()
      .append_pair("include", "anime")
      .append_pair("filter[user_id]", &user_id.to_string())
      .append_pair("filter[anime_id]", &anime_id.to_string())
      .finish()
      .as_str();

    let uri = Uri::from_str(url).unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType(
      Mime::from_str("application/vnd.api+json").unwrap(),
    ));

    Box::new(
      self
        .request(req)
        .and_then(|response| match response {
          Response::Ok { mut data, included, .. } => {
            Ok((data.pop(), included.map_or(None, |mut v| v.pop())))
          }
          Response::Error { errors } => {
            Err(Error::Kitsu(KitsuError {
              description: format!("{}: {}", errors[0].title, errors[1].detail),
            }))
          }
        })
        .and_then(|(entry, anime)| match (entry, anime) {
          (Some(entry), Some(anime)) => Ok(Some(
            (from_value(entry).unwrap(), from_value(anime).unwrap()),
          )),
          _ => Ok(None),
        }),
    )
  }
}
