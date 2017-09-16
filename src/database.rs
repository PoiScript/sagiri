use std::rc::Rc;
use std::str::FromStr;
use std::cell::RefCell;

use futures::{future, Future, Stream};

use hyper::{Method, Request, Uri};
use hyper::header::Authorization;

use serde_json::from_slice;

use error::{DatabaseError, Error};
use types::{Client, DatabaseResponse as Response, User};

pub struct Database {
  uri: Uri,
  token: String,
  client: Client,
  users: Rc<RefCell<Vec<User>>>,
}

impl Database {
  pub fn new(token: String, client: Client) -> Database {
    Database {
      token,
      client,
      users: Rc::new(RefCell::new(Vec::new())),
      uri: Uri::from_str("https://sagiri-izumi.firebaseapp.com/api/kitsu/user").unwrap(),
    }
  }

  pub fn fetch(&mut self) -> Box<Future<Item = Vec<User>, Error = Error>> {
    let users = self.users.clone();
    let mut req = Request::new(Method::Get, self.uri.clone());
    req.headers_mut().set(Authorization(self.token.clone()));

    Box::new(self.client.request(req).from_err::<Error>().and_then(
      move |res| {
        res
          .body()
          .from_err::<Error>()
          .concat2()
          .and_then(|chunks| {
            future::result::<Response, Error>(from_slice(&chunks).map_err(|e| e.into()))
          })
          .and_then(move |res| match res {
            Response::Ok { data } => {
              users.borrow_mut().clone_from(&data);
              Ok(data)
            }

            Response::Error { error } => Err(Error::Database(DatabaseError { description: error })),
          })
      },
    ))
  }

  pub fn get_kitsu_id(&mut self, telegram_id: i64) -> Option<i64> {
    self
      .users
      .borrow()
      .iter()
      .find(|&x| &x.telegram_id == &telegram_id)
      .map(|ref x| &x.kitsu_id)
      .cloned()
  }

  pub fn get_token(&mut self, telegram_id: i64, kitsu_id: i64) -> Option<String> {
    self
      .users
      .borrow()
      .iter()
      .find(|&x| &x.telegram_id == &telegram_id && &x.kitsu_id == &kitsu_id)
      .map(|ref x| &x.kitsu_token)
      .cloned()
  }
}
