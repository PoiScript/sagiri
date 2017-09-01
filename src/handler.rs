use std::str::FromStr;

use nom::IResult;
use futures::{done, Future};

use bot::telegram::Bot;
use kitsu::Api;
use error::{Error, TelegramError};
use types::Client;
use types::kitsu::*;
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

    info!("received message: '{}' from {}, in {}", text, user_id, text);

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

    info!("received query: '{}' from {}", data, user_id);

    match query.message {
      Some(msg) => {
        let msg_id = msg.message_id.unwrap();
        let chat_id = msg.chat.unwrap().id;
        let date = msg.date.unwrap();

        match parse_query(&data) {
          IResult::Done(_, command) => {
            match command {
              QueryCommand::Page { kitsu_id, offset } => {
                self.page(kitsu_id, msg_id, chat_id, offset)
              }
            }
          }
          _ => {
            self.bot.send_message(
              chat_id,
              format!("Unknown Command."),
              None,
            )
          }
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
      None,
    )
  }

  fn list(&mut self, user_id: i64, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let api = self.api.clone();
    let bot = self.bot.clone();
    match self.db.get_user(user_id) {
      None => bot.send_message(chat_id, String::from("Unknown command"), None),
      Some(user) => Box::new(
        api
          .fetch_anime(user.kitsu_id, 0)
          .and_then(move |(prev, next, pairs)| {
            Ok(parse_anime(user.kitsu_id, prev, next, pairs))
          })
          .and_then(move |(text, buttons)| {
            bot.send_inline_keyboard(chat_id, text, Some(ParseMode::HTML), buttons)
          }),
      ),
    }
  }

  fn update(&mut self, chat_id: i64) -> Box<Future<Item = Message, Error = Error>> {
    let bot = self.bot.clone();
    Box::new(self.db.fetch().and_then(move |users| {
      bot.send_message(
        chat_id,
        format!("<pre>Successful update: {} user(s)<pre>", users.len()),
        Some(ParseMode::HTML),
      )
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
    Box::new(
      self
        .api
        .fetch_anime(kitsu_id, offset)
        .and_then(move |(prev, next, pairs)| {
          Ok(parse_anime(kitsu_id, prev, next, pairs))
        })
        .and_then(move |(text, buttons)| {
          bot.edit_inline_keyboard(msg_id, chat_id, text, buttons)
        }),
    )
  }
}

fn parse_anime(
  kitsu_id: i64,
  prev: Option<String>,
  next: Option<String>,
  pairs: Option<(Vec<Entries>, Vec<Anime>)>,
) -> (String, Vec<Vec<InlineKeyboardButton>>) {
  let buttons = vec![InlineKeyboardButton::paginator(kitsu_id, prev, next)];

  let mut text = String::new();

  match pairs {
    None => text = format!("No Anime :("),
    Some((entries, animes)) => {
      for (entry, anime) in entries.iter().zip(animes.iter()) {
        text.push_str(&format!(
          "{}\n{:?}({}/{})\n",
          anime.attributes.canonical_title,
          entry.attributes.status,
          entry.attributes.progress,
          anime.attributes.episode_count.unwrap_or(99),
        ))
      }
    }
  };

  (text, buttons)
}
