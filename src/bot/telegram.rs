use std::str::FromStr;
use std::time::Duration;

use futures::{future, Async, Future, Poll, Stream};

use hyper::{Method, Request, Uri};
use hyper::header::{ContentLength, ContentType};

use serde::ser::Serialize;
use serde_json::{from_slice, to_string};

use types::Client;
use types::telegram::*;
use error::{Error, TelegramError};

#[derive(Clone)]
pub struct Bot {
  client: Client,
  base_url: String,
}

impl Bot {
  pub fn new(token: &str, client: Client) -> Bot {
    Bot {
      client,
      base_url: format!("https://api.telegram.org/bot{}/", token),
    }
  }

  fn request<S>(&self, method: &str, data: &S) -> Box<Future<Item=Response, Error=Error>>
    where S: Serialize,
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
          .and_then(|res| match res {
            Response::Error { description } => Err(Error::Telegram(TelegramError { description })),
            _ => Ok(res),
          })
      },
    ))
  }

  pub fn send_message(
    &self,
    chat_id: i64,
    text: String,
    parse_mode: Option<ParseMode>,
    buttons: Option<Vec<Vec<InlineKeyboardButton>>>,
  ) -> Box<Future<Item=Message, Error=Error>> {
    let message = Message {
      parse_mode,
      text: Some(text),
      chat_id: Some(chat_id),
      reply_markup: buttons.map(|b| ReplyMarkup::InlineKeyboard(b)),
      ..Default::default()
    };
    Box::new(
      self.request::<Message>("sendMessage", &message)
        .and_then(|res| match res {
          Response::Message { result } => Ok(result),
          _ => Err(Error::Telegram(TelegramError { description: String::from("Invalid JSON") }))
        })
    )
  }

  pub fn edit_inline_keyboard(
    self,
    msg_id: i64,
    chat_id: i64,
    text: String,
    parse_mode: Option<ParseMode>,
    buttons: Option<Vec<Vec<InlineKeyboardButton>>>,
  ) -> Box<Future<Item=Message, Error=Error>> {
    let message = Message {
      parse_mode,
      text: Some(text),
      chat_id: Some(chat_id),
      message_id: Some(msg_id),
      reply_markup: buttons.map(|b| ReplyMarkup::InlineKeyboard(b)),
      ..Default::default()
    };
    Box::new(
      self.request::<Message>("editMessageText", &message)
        .and_then(|res| match res {
          Response::Message { result } => Ok(result),
          _ => Err(Error::Telegram(TelegramError { description: String::from("Invalid JSON") }))
        })
    )
  }

  pub fn answer_query(
    &self,
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
  ) -> Box<Future<Item=bool, Error=Error>> {
    let query_answer = QueryAnswer {
      text,
      show_alert,
      callback_query_id,
    };
    Box::new(
      self.request::<QueryAnswer>("answerCallbackQuery", &query_answer)
        .and_then(|res| match res {
          Response::Bool { result } => Ok(result),
          _ => Err(Error::Telegram(TelegramError { description: String::from("Invalid JSON") }))
        }))
  }
}

pub struct UpdateStream {
  bot: Bot,
  timeout: Duration,
  next_offset: i32,
  pending_updates: Vec<Update>,
  pending_response: Option<Box<Future<Item=Vec<Update>, Error=Error>>>,
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

  fn get_updates(&self, offset: i32) -> Box<Future<Item=Vec<Update>, Error=Error>> {
    let req = GetUpdate {
      offset,
      timeout: self.timeout.as_secs() as i32,
    };

    Box::new(
      self.bot
        .request("getUpdates", &req)
        .and_then(|res| match res {
          Response::Update { result } => Ok(result),
          _ => Err(Error::Telegram(TelegramError { description: String::from("Invalid JSON") }))
        })
    )
  }
}

impl Stream for UpdateStream {
  type Item = Update;
  type Error = Error;

  fn poll(&mut self) -> Poll<Option<Update>, Error> {
    loop {
      // handle every response given from `getUpdates`
      while let Some(update) = self.pending_updates.pop() {
        // update offset
        let new_offset = match update {
          Update::Message { update_id, .. } |
          Update::CallbackQuery { update_id, .. } => update_id
        };
        if new_offset < self.next_offset {
          continue;
        }
        self.next_offset = new_offset + 1;

        return Ok(Async::Ready(Some(update)));
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
