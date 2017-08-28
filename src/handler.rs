use error::Error;
use futures::{Future, future};
use nom::IResult;

use kitsu::Api;
use database::Database;
use types::{Client, User, Url};
use types::telegram::{Message, send_message};

#[derive(Debug)]
enum Command {
  List,
  Update,
}

named!(parse_message<&str, Command>,
  alt!(
    map!(tag!("/list"), |_| Command::List) |
    map!(tag!("/update"), |_| Command::Update)
  )
);

pub struct Handler {
  api: Api,
  db: Database,
}

impl Handler {
  pub fn new(client: Client, token: String) -> Handler {
    Handler {
      api: Api::new(client.clone()),
      db: Database::new(token, client),
    }
  }

  pub fn handle(&mut self, msg: Message) -> Box<Future<Item = Message, Error = Error>> {
    let chat_id = msg.chat.unwrap().id;
    let user_id = msg.from.unwrap().id;
    let text = msg.text.unwrap_or_else(|| String::new());

    return match parse_message(&text) {
      IResult::Done(_, command) => {
        match command {
          Command::List => self.list(user_id, chat_id),
          Command::Update => self.update(chat_id),
        }
      }
      _ => self.unknown(chat_id),
    };
  }

  fn unknown(&self, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    send_message(chat_id, String::from("Unknown command"))
  }

  fn list(&mut self, user_id: i64, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let api = self.api.clone();
    match self.db.get_user(user_id) {
      None => send_message(chat_id, String::from("Unknown command")),
      Some(user) => {
        send_message(
          chat_id,
          format!("your kitsu token is: {}", user.kitsu_token),
        )
      }
    }
  }

  fn update(&mut self, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    Box::new(self.db.fetch().and_then(move |users| {
      send_message(chat_id, format!("Successful update: {} users", users.len()))
    }))
  }
}
