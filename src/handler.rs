use std::str::FromStr;

use nom::IResult;
use futures::{done, Future};

use bot::telegram::Bot;
use kitsu::Api;
use error::{Error, TelegramError};
use types::Client;
use types::telegram::*;
use database::Database;

#[derive(Debug)]
enum Command {
  List,
  Update,
}

#[derive(Debug)]
pub enum QueryCommand {
  Page { kitsu_id: i64, offset: i64 },
}

named!(parse_message<&str, Command>,
  alt!(
    map!(tag!("/list"), |_| Command::List) |
    map!(tag!("/update"), |_| Command::Update)
  )
);

named!(parse_query<&str, QueryCommand>,
  alt!(
    do_parse!(
      tag!("/page/") >>
      kitsu_id: map_res!(take_until_and_consume!("/"), i64::from_str)  >>
      offset: map_res!(take_until!("/"), i64::from_str)  >>
      (QueryCommand::Page{ kitsu_id, offset })
    )
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

  pub fn handle_message(&mut self, msg: Message) -> Box<Future<Item = Message, Error = Error>> {
    let chat_id = msg.chat.unwrap().id;
    let user_id = msg.from.unwrap().id;
    let text = msg.text.unwrap_or(String::new());

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

  pub fn handle_query(
    &mut self,
    query: CallbackQuery,
  ) -> Box<Future<Item = Message, Error = Error>> {
    let user_id = query.from.id;
    let data = query.data.unwrap_or(String::new());

    match query.message {
      Some(msg) => {
        let msg_id = msg.message_id.unwrap();
        let chat_id = msg.chat.unwrap().id;
        let date = msg.date.unwrap();

        match parse_query(&data) {
          IResult::Done(_, command) => {
            match command {
              QueryCommand::Page { kitsu_id, offset } => self.page(kitsu_id, msg_id, chat_id, offset),
            }
          }
          _ => self.bot.send_message(chat_id, format!("Unknown Command.")),
        }
      }
      None => {
        Box::new(done::<_, Error>(Err(Error::Telegram(TelegramError {
          description: "Outdated Message.".to_owned(),
        }))))
      }
    }
  }

  fn unknown(&self, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    self.bot.send_message(
      chat_id,
      String::from("Unknown command."),
    )
  }

  fn list(&mut self, user_id: i64, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let api = self.api.clone();
    let bot = self.bot.clone();
    match self.db.get_user(user_id) {
      None => bot.send_message(chat_id, String::from("Unknown command")),
      Some(user) => Box::new(api.fetch_anime(user.kitsu_id, 0).and_then(move |(data,
             included,
             links)| {
        let buttons = vec![
          InlineKeyboardButton::paginator(user.kitsu_id, links.prev, links.next),
        ];

        let text = match included {
          None => format!("No Anime :("),
          Some(animes) => {
            let mut str = String::new();
            for (anime, entry) in animes.iter().zip(data.iter()) {
              str.push_str(&format!(
                "{:?}: {}\n",
                entry.attributes.status,
                anime.attributes.canonical_title
              ))
            }
            str
          }
        };
        bot.send_inline_keyboard(chat_id, text, buttons)
      })),
    }
  }

  fn update(&mut self, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let bot = self.bot.clone();
    Box::new(self.db.fetch().and_then(move |users| {
      bot.send_message(chat_id, format!("Successful update: {} users", users.len()))
    }))
  }

  fn page(
    &self,
    kitsu_id: i64,
    msg_id: i64,
    chat_id: i64,
    offset: i64,
  ) -> Box<Future<Item = Message, Error = Error>> {
    let bot = self.bot.clone();
    Box::new(self.api.fetch_anime(kitsu_id, offset).and_then(move |(data,
           included,
           links)| {
      let buttons = vec![
        InlineKeyboardButton::paginator(kitsu_id, links.prev, links.next),
      ];

      let text = match included {
        None => format!("No Anime :("),
        Some(animes) => {
          let mut text = String::new();
          for (anime, entry) in animes.iter().zip(data.iter()) {
            text.push_str(&format!(
              "{:?}: {}\n",
              entry.attributes.status,
              anime.attributes.canonical_title
            ))
          }
          text
        }
      };
      bot.edit_inline_keyboard(msg_id, chat_id, text, buttons)
    }))
  }
}
