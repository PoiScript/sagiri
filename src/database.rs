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
      uri: Uri::from_str(
        "https://sagiri-izumi.firebaseapp.com/api/kitsu/user",
      ).unwrap(),
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
          .and_then(move |response| match response {
            Response::Ok { data } => {
              users.borrow_mut().clone_from(&data);
              Ok(data)
            }

            Response::Error { error } => Err(Error::Database(DatabaseError { description: error })),
          })
      },
    ))
  }

  pub fn get_user(&mut self, telegram_id: i64) -> Option<User> {
    self
      .users
      .borrow()
      .iter()
      .find(|&x| &x.telegram_id == &telegram_id)
      .cloned()
  }
}
