use std::str::FromStr;

use url::Url;

use futures::{future, Future, Stream};

use hyper::mime::Mime;
use hyper::{Method, Request, Uri};
use hyper::header::{Authorization, Bearer, ContentLength, ContentType};

use serde_json::{from_slice, to_string};

use types::Client;
use error::{Error, KitsuError};
use types::kitsu::{Anime, Entry, Json, Type, Relationships, EntryAttributes};

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

  fn request(&self, req: Request) -> Box<Future<Item=Json, Error=Error>> {
    Box::new(self.client.request(req).from_err::<Error>().and_then(
      |res| {
        res.body().from_err::<Error>().concat2().and_then(|chunks| {
          future::result::<Json, Error>(from_slice(&chunks).map_err(|e| e.into()))
        }).and_then(|res| match res {
          Json::Error { errors } => {
            let mut description = String::new();
            for e in errors {
              description.push_str(&format!("{}: {}", e.title, e.detail))
            }
            Err(Error::Kitsu(KitsuError { description }))
          }
          _ => Ok(res)
        })
      },
    ))
  }

  pub fn fetch_anime(
    &self,
    user_id: i64,
    offset: i64,
  ) -> Box<Future<Item=(Option<String>, Option<String>, Vec<Entry>, Vec<Anime>), Error=Error>> {
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
        .and_then(|res| match res {
          Json::AnimeEntry { data, included, links, .. } => Ok((data, included, links)),
          _ => Err(Error::Kitsu(KitsuError { description: String::from("Invalid JSON") }))
        })
        .and_then(|(entries, animes, links)|
          Ok((links.prev, links.next, entries, animes))
        ),
    )
  }

  pub fn get_anime(
    &self,
    user_id: i64,
    anime_id: i64,
  ) -> Box<Future<Item=Option<(Entry, Anime)>, Error=Error>> {
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
        .and_then(|res| match res {
          Json::AnimeEntry { mut data, mut included, .. } => Ok((data.pop(), included.pop())),
          _ => Err(Error::Kitsu(KitsuError { description: String::from("Invalid JSON") }))
        })
        .and_then(|(entry, anime)| match (entry, anime) {
          (Some(entry), Some(anime)) => Ok(Some((entry, anime))),
          _ => Ok(None),
        }),
    )
  }

  pub fn update_anime_entry(
    &self,
    token: String,
    entry_id: String,
    progress: i64,
    anime_id: String
  ) -> Box<Future<Item=Entry, Error=Error>> {
    let url = self.base.join("library-entries/").unwrap().join(&entry_id).unwrap();
    let uri = Uri::from_str(url.as_str()).unwrap();

    let json = Json::Entry {
      data: Entry {
        id: entry_id,
        kind: Type::LibraryEntries,
        attributes: Some(EntryAttributes { status: None, progress: Some(progress) }),
        relationships: Some(
          Relationships { anime: Some(Anime { id: anime_id, attributes: None }) }
        ),
      },
    };
    let body = to_string(&json).expect("error/json-to-string");

    let mut req = Request::new(Method::Patch, uri);
    req.headers_mut().set(ContentType(
      Mime::from_str("application/vnd.api+json").unwrap(),
    ));
    req.headers_mut().set(Authorization(Bearer { token }));
    req.headers_mut().set(ContentLength(body.len() as u64));
    req.set_body(body);

    Box::new(
      self
        .request(req)
        .and_then(|res| match res {
          Json::Entry { data } => Ok(data),
          _ => Err(Error::Kitsu(KitsuError { description: String::from("Invalid JSON") }))
        }))
  }
}
