//  primitive adapter.telegram
pub type Integer = i64;
pub type Float = f32;

// Response from Telegram Bot API, See: https://core.telegram.org/bots/api#making-requests
#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub error_code: Option<Integer>,
    pub description: Option<String>,
    pub result: Option<T>,
}

// Incoming Update from Telegram, See: https://core.telegram.org/bots/api#update
#[derive(Serialize, Deserialize)]
pub struct Update {
    pub update_id: Integer,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
}

// Telegram User or Bot, See: https://core.telegram.org/bots/api#user
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

// Telegram Chat, See: https://core.telegram.org/bots/api#chat
#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: Integer,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
}

// Telegram Message, See: https://core.telegram.org/bots/api#message
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message_id: Integer,
    pub from: Option<User>,
    pub date: Integer,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<Integer>,
    pub forward_date: Option<Integer>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<Integer>,
    pub text: Option<String>,
    pub sticker: Option<Sticker>,
    pub new_chat_member: Option<User>,
    pub left_chat_member: Option<User>
}

// Telegram Sticker, See: https://core.telegram.org/bots/api#sticker
#[derive(Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub emoji: Option<String>,
    pub file_size: Option<Integer>,
}