use std::str::FromStr;
use std::time::Duration;

use futures::{future, Future, Stream, Async, Poll};

use hyper::{Uri, Method, Request};
use hyper::header::{ContentType, ContentLength};

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_value, from_slice, to_string};

use types::Client;
use error::{Error, TelegramError};
use types::telegram::{Message, Update, Received, Response, GetUpdate};

#[derive(Clone)]
pub struct Bot {
  client: Client,
  base_url: String,
}

impl Bot {
  pub fn new(token: &str, client: Client) -> Bot {
    Bot {
      client: client,
      base_url: format!("https://api.telegram.org/bot{}/", token),
    }
  }

  fn request<T, S>(&self, method: &str, data: &S) -> Box<Future<Item = T, Error = Error>>
  where
    S: Serialize,
    T: DeserializeOwned + 'static,
  {
    let uri = Uri::from_str(&format!("{}{}", self.base_url, method)).expect("error/build-uri");

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
            Response::Ok { result } => from_value(result).map_err(|e| e.into()),

            Response::Error { description } => {
              return Err(Error::Telegram(TelegramError { description }));
            }
          })
      },
    ))
  }

  pub fn send_message(&self, chat_id: i64, text: String) -> Box<Future<Item = Message, Error = Error>> {
    let message = Message {
      text: Some(text),
      chat_id: Some(chat_id),
      ..Default::default()
    };
    self.request::<_, Message>("sendMessage", &message)
  }
}

pub struct UpdateStream {
  bot: Bot,
  timeout: Duration,
  next_offset: i32,
  pending_response: Option<Box<Future<Item = Vec<Update>, Error = Error>>>,
  pending_updates: Vec<Update>,
}

impl UpdateStream {
  pub fn new(bot: Bot) -> UpdateStream {
    UpdateStream {
      bot,
      timeout: Duration::from_secs(120),
      next_offset: 0,
      pending_response: None,
      pending_updates: Vec::new(),
    }
  }

  fn get_updates(&self, offset: i32) -> Box<Future<Item = Vec<Update>, Error = Error>> {
    let req = GetUpdate {
      offset,
      timeout: self.timeout.as_secs() as i32,
    };

    self.bot.request("getUpdates", &req)
  }
}

impl Stream for UpdateStream {
  type Item = Received;
  type Error = Error;

  fn poll(&mut self) -> Poll<Option<Received>, Error> {
    loop {
      // handle every response given from `getUpdates`
      while let Some(update) = self.pending_updates.pop() {
        // update offset
        let new_offset = update.update_id;
        if new_offset < self.next_offset {
          continue;
        }
        self.next_offset = new_offset + 1;

        return match update.parse() {
          Ok(up) => Ok(Async::Ready(Some(up))),
          Err(err) => Err(err),
        };
      }

      let pending_response = self.pending_response.take();

      if let Some(mut pending) = pending_response {
        match pending.poll() {
          Ok(Async::Ready(updates)) => {
            self.pending_updates = updates;
            continue;
          }
          Ok(Async::NotReady) => {
            self.pending_response = Some(pending);
            return Ok(Async::NotReady);
          }
          Err(e) => {
            return Err(e);
          }
        }
      }

      self.pending_response = Some(self.get_updates(self.next_offset));
    }
  }
}
