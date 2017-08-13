use std::str::FromStr;

use futures::{future, Future, Stream};

use hyper::{Uri, Method, Request};
use hyper::header::{ContentType, ContentLength};

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_value, from_slice, to_string};

use types::Client;
use error::{Error, KitsuError};
use types::kitsu::Response;

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

  pub fn request<T, S>(
    &self,
    method: &str,
    id: i64,
    data: &S,
  ) -> Box<Future<Item = T, Error = Error>>
  where
    S: Serialize,
    T: DeserializeOwned + 'static,
  {
    let uri = Uri::from_str(&format!("{}{}/{}", self.base_url, method, id))
      .expect("error/build-uri");

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
}
