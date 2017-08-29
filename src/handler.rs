use nom::IResult;
use futures::Future;

use bot::telegram::Bot;
use kitsu::Api;
use error::Error;
use types::Client;
use types::telegram::Message;
use database::Database;

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
  bot: Bot,
  db: Database,
}

impl Handler {
  pub fn new(bot: Bot, client: Client, token: String) -> Handler {
    Handler {
      bot: bot,
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
    self.bot.send_message(
      chat_id,
      String::from("Unknown command"),
    )
  }

  fn list(&mut self, user_id: i64, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let api = self.api.clone();
    let bot = self.bot.clone();
    match self.db.get_user(user_id) {
      None => bot.send_message(chat_id, String::from("Unknown command")),
      Some(user) => Box::new(api.fetch_anime(chat_id, user.kitsu_id).and_then(
        move |(data,
               included,
               chat_id)| {
          let text = match included {
            None => format!("No Anime :("),
            Some(animes) => {
              let mut str = String::new();
              for (anime, entry) in animes.iter().zip(data.iter()) {
                str.push_str(&format!(
                  "{:?}: {}",
                  entry.attributes.status,
                  anime.attributes.canonical_title
                ))
              }
              str
            }
          };
          bot.send_message(chat_id, text)
        },
      )),
    }
  }

  fn update(&mut self, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let bot = self.bot.clone();
    Box::new(self.db.fetch().and_then(move |users| {
      bot.send_message(chat_id, format!("Successful update: {} users", users.len()))
    }))
  }
}
