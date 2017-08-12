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
  message: Option<Value>,
  channel_post: Option<Value>,
  inline_query: Option<Value>,
  callback_query: Option<Value>,
  edited_message: Option<Value>,
  shipping_query: Option<Value>,
  pre_checkout_query: Option<Value>,
  edited_channel_post: Option<Value>,
  chosen_inline_result: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub enum Received {
  Message(Value),
  ChannelPost(Value),
  InlineQuery(Value),
  CallbackQuery(Value),
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
      Err(Error::Telegram(
        TelegramError { description: "can't parse update".to_owned() },
      ))
    }
  }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
  pub message_id: i64,
  pub from: User,
  pub date: i32,
  pub chat: Chat,
  pub text: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
  pub id: i64,
  pub first_name: String,
  pub last_name: Option<String>,
  pub username: Option<String>,
  pub language_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
  Private,
  Group,
  SuperGroup,
  Channel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Empty;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct GetUpdate {
  pub offset: i32,
  pub timeout: i32,
}

#[derive(Serialize)]
pub struct SendMessage {
  pub chat_id: i64,
  pub text: String,
}
