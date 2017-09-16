#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Response {
  Bool { result: bool },
  Update { result: Vec<Update> },
  Message { result: Message },
  Error { description: String },
}

#[serde(untagged)]
#[derive(Debug, Deserialize)]
pub enum Update {
  Message { update_id: i32, message: Message },
  CallbackQuery { update_id: i32, callback_query: CallbackQuery }
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
  #[serde(skip_serializing_if = "Option::is_none")] pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")] pub callback_data: Option<String>,
}

impl InlineKeyboardButton {
  pub fn with_callback_data(text: String, data: String) -> InlineKeyboardButton {
    InlineKeyboardButton {
      text,
      url: None,
      callback_data: Some(data),
    }
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
  #[serde(rename = "type")] pub chat_type: ChatType,
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
pub struct GetUpdate {
  pub offset: i32,
  pub timeout: i32,
}

#[derive(Serialize)]
pub struct QueryAnswer {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub show_alert: Option<bool>,
  pub callback_query_id: String,
}
