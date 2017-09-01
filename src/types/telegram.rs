use std::borrow::Cow;

use url::Url;
use serde_json::Value;
use error::{Error, TelegramError};

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Response {
  Ok { result: Value },
  Error { description: String },
}

#[derive(Debug, Deserialize)]
pub struct Update {
  pub update_id: i32,
  message: Option<Message>,
  channel_post: Option<Value>,
  inline_query: Option<Value>,
  callback_query: Option<CallbackQuery>,
  edited_message: Option<Value>,
  shipping_query: Option<Value>,
  pre_checkout_query: Option<Value>,
  edited_channel_post: Option<Value>,
  chosen_inline_result: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub enum Received {
  Message(Message),
  ChannelPost(Value),
  InlineQuery(Value),
  CallbackQuery(CallbackQuery),
  EditedMessage(Value),
  ShippingQuery(Value),
  PreCheckoutQuery(Value),
  EditedChannelPost(Value),
  ChosenInlineResult(Value),
}

impl Update {
  pub fn parse(self) -> Result<Received, Error> {
    if let Some(m) = self.message {
      Ok(Received::Message(m))
    } else if let Some(e) = self.edited_message {
      Ok(Received::EditedMessage(e))
    } else if let Some(c) = self.channel_post {
      Ok(Received::ChannelPost(c))
    } else if let Some(e) = self.edited_channel_post {
      Ok(Received::EditedChannelPost(e))
    } else if let Some(i) = self.inline_query {
      Ok(Received::InlineQuery(i))
    } else if let Some(c) = self.chosen_inline_result {
      Ok(Received::ChosenInlineResult(c))
    } else if let Some(c) = self.callback_query {
      Ok(Received::CallbackQuery(c))
    } else if let Some(s) = self.shipping_query {
      Ok(Received::ShippingQuery(s))
    } else if let Some(p) = self.pre_checkout_query {
      Ok(Received::PreCheckoutQuery(p))
    } else {
      Err(Error::Telegram(TelegramError {
        description: "can't parse update".to_owned(),
      }))
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Message {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<User>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub date: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chat: Option<Chat>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chat_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_markup: Option<ReplyMarkup>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inline_message_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parse_mode: Option<ParseMode>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ParseMode {
  HTML,
  Markdown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplyMarkup {
  InlineKeyboard(Vec<Vec<InlineKeyboardButton>>),
  ReplyKeyboardMarkup,
  ReplyKeyboardRemove,
  ForceReply,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
  pub text: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub callback_data: Option<String>,
}

impl InlineKeyboardButton {
  fn with_url(text: String, url: String) -> InlineKeyboardButton {
    InlineKeyboardButton {
      text: text,
      url: Some(url),
      callback_data: None,
    }
  }

  fn with_data(text: String, data: String) -> InlineKeyboardButton {
    InlineKeyboardButton {
      text: text,
      url: None,
      callback_data: Some(data),
    }
  }

  pub fn paginator(kitsu_id: i64, prev: Option<String>, next: Option<String>)
    -> Vec<InlineKeyboardButton> {
    fn get_button(url: Option<String>, kitsu_id: i64, text: &str) -> Option<InlineKeyboardButton> {
      url
        .map_or(None, |x| match Url::parse(&x) {
          Ok(url) => Some(url),
          Err(_) => None,
        })
        .map_or(None, |url| {
          url
            .query_pairs()
            .find(|&(ref key, _)| key == &Cow::Borrowed("page[offset]"))
            .map_or(None, |(_, offset)| Some(offset))
            .map(|offset| {
              InlineKeyboardButton::with_data(
                format!("{}", text),
                format!("/page/{}/{}/", kitsu_id, offset),
              )
            })
        })
    }
    let mut buttons = Vec::new();
    if let Some(button) = get_button(prev, kitsu_id, "Prev") {
      buttons.push(button)
    }
    if let Some(button) = get_button(next, kitsu_id, "Next") {
      buttons.push(button)
    }
    buttons
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: i64,
  pub first_name: String,
  pub last_name: Option<String>,
  pub username: Option<String>,
  pub language_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chat {
  pub id: i64,
  #[serde(rename = "type")]
  pub chat_type: ChatType,
  pub title: Option<String>,
  pub username: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub all_members_are_administrators: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
  Private,
  Group,
  SuperGroup,
  Channel,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallbackQuery {
  pub id: String,
  pub from: User,
  pub data: Option<String>,
  pub message: Option<Message>,
  pub inline_message_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Empty;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct GetUpdate {
  pub offset: i32,
  pub timeout: i32,
}
