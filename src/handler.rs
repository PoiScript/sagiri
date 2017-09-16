use nom::IResult;
use futures::{done, Future};

use bot::telegram::Bot;
use kitsu::Api;
use error::{Error, TelegramError};
use types::{Client, MsgCommand, QueryCommand};
use utils::*;
use types::telegram::{CallbackQuery, Message, ParseMode};
use database::Database;

pub struct Handler {
  api: Api,
  bot: Bot,
  db: Database,
}

impl Handler {
  pub fn new(bot: Bot, client: Client, token: String) -> Handler {
    Handler {
      bot,
      api: Api::new(client.clone()),
      db: Database::new(token, client),
    }
  }

  pub fn handle_message(&mut self, msg: Message) -> Box<Future<Item=Message, Error=Error>> {
    let chat_id = msg.chat.unwrap().id;
    let user_id = msg.from.unwrap().id;
    let text = msg.text.unwrap_or(String::new());

    info!("received message: '{}' from {}, in {}", text, user_id, text);

    match parse_message(&text) {
      IResult::Done(_, command) => match command {
        MsgCommand::List => self.list(user_id, chat_id),
        MsgCommand::Update => self.update(chat_id),
      },
      _ => self.unknown(chat_id),
    }
  }

  pub fn handle_query(
    &mut self,
    query: CallbackQuery,
  ) -> Box<Future<Item=Message, Error=Error>> {
    let query_id = query.id;
    let user_id = query.from.id;
    let data = query.data.unwrap_or(String::new());

    info!("received query: '{}' from {}", data, user_id);

    match query.message {
      Some(msg) => {
        let msg_id = msg.message_id.unwrap();
        let chat_id = msg.chat.unwrap().id;

        match parse_query(&data) {
          IResult::Done(_, command) => match command {
            QueryCommand::Offset { kitsu_id, offset } => {
              self.offset(msg_id, chat_id, kitsu_id, offset, query_id)
            }
            QueryCommand::Detail { kitsu_id, anime_id } => {
              self.detail(msg_id, chat_id, kitsu_id, anime_id, query_id)
            }
            QueryCommand::Progress { kitsu_id, anime_id, entry_id, progress } => {
              self.detail(msg_id, chat_id, kitsu_id, anime_id, query_id)
            }
          },
          _ => self.unknown(chat_id),
        }
      }
      None => Box::new(done::<_, Error>(Err(Error::Telegram(TelegramError {
        description: "Outdated Message.".to_owned(),
      })))),
    }
  }

  fn unknown(&self, chat_id: i64) -> Box<Future<Item=Message, Error=Error>> {
    self
      .bot
      .send_message(chat_id, String::from("Unknown command."), None, None)
  }

  fn list(&mut self, user_id: i64, chat_id: i64) -> Box<Future<Item=Message, Error=Error>> {
    let bot = self.bot.clone();
    match self.db.get_kitsu_id(user_id) {
      None => bot.send_message(
        chat_id,
        format!("Non-registered user: {}", user_id),
        None,
        None,
      ),
      Some(kitsu_id) => Box::new(
        self
          .api
          .fetch_anime(kitsu_id, 0)
          .and_then(move |(prev, next, entries, animes)| {
            Ok(parse_anime_list(kitsu_id, prev, next, entries, animes))
          })
          .and_then(move |(text, buttons)| {
            bot.send_message(chat_id, text, Some(ParseMode::HTML), Some(buttons))
          }),
      ),
    }
  }

  fn update(&mut self, chat_id: i64) -> Box<Future<Item=Message, Error=Error>> {
    let bot = self.bot.clone();
    Box::new(self.db.fetch().and_then(move |users| {
      bot.send_message(
        chat_id,
        format!("<pre>Successful update: {} user(s)</pre>", users.len()),
        Some(ParseMode::HTML),
        None,
      )
    }))
  }

  fn offset(
    &self,
    msg_id: i64,
    chat_id: i64,
    kitsu_id: i64,
    offset: i64,
    query_id: String,
  ) -> Box<Future<Item=Message, Error=Error>> {
    let bot1 = self.bot.clone();
    let bot2 = self.bot.clone();
    Box::new(
      self
        .api
        .fetch_anime(kitsu_id, offset)
        .and_then(move |(prev, next, entries, animes)| {
          Ok(parse_anime_list(kitsu_id, prev, next, entries, animes))
        })
        .and_then(move |(text, buttons)| {
          bot1.edit_inline_keyboard(msg_id, chat_id, text, Some(ParseMode::HTML), Some(buttons))
        })
        .and_then(move |_| {
          bot2.answer_query(query_id, None, None)
        }),
    )
  }

  fn detail(
    &self,
    msg_id: i64,
    chat_id: i64,
    kitsu_id: i64,
    anime_id: i64,
    query_id: String,
  ) -> Box<Future<Item=Message, Error=Error>> {
    let bot = self.bot.clone();
    Box::new(
      self
        .api
        .get_anime(kitsu_id, anime_id)
        .and_then(move |pair| Ok(parse_anime_detail(kitsu_id, pair)))
        .and_then(move |(text, buttons)| {
          bot.edit_inline_keyboard(msg_id, chat_id, text, Some(ParseMode::HTML), Some(buttons))
        }),
    )
  }
}
