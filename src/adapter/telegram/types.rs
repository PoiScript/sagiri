use adapter::Message as sMessage;
use adapter::Error;
use std::str;
use nom::{eol, alphanumeric, non_empty};
use nom::IResult::{Done, Incomplete};
use nom::IResult::Error as NomError;

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

impl sMessage for Message {
    fn get_sender_id(self) -> Result<String, Error> {
        match self.from {
            Some(user) => Ok(user.id.to_string()),
            None => Err(Error::NA())
        }
    }

    fn get_sender_name(self) -> Result<String, Error> {
        match self.from {
            Some(user) => match user.username {
                Some(username) => Ok(username),
                None => Ok(user.first_name)
            },
            None => Err(Error::NA())
        }
    }

    fn get_chat_id(self) -> String {
        self.chat.id.to_string()
    }

    fn get_chat_name(self) -> Result<String, Error> {
        match self.chat.title {
            Some(title) => Ok(title),
            None => Err(Error::NA())
        }
    }

    fn get_command(self) -> Result<String, Error> {
        match self.text {
            Some(text) => {
                match get_command(&text.as_bytes()) {
                    Done(I, O) => Ok(O.to_string()),
                    Incomplete(_) => Err(Error::Invalid("Invalid Message".to_owned())),
                    NomError(err) => Err(Error::Nom("Unable to parse".to_owned())),
                }
            }
            None => Err(Error::NA())
        }
    }

    fn get_argument(self) -> Result<String, Error> {
        match self.text {
            Some(text) => {
                match get_arguments(&text.as_bytes()) {
                    Done(I, O) => Ok(O.to_string()),
                    Incomplete(_) => Err(Error::Invalid("Invalid Message".to_owned())),
                    NomError(err) => Err(Error::Nom("Unable to parse".to_owned())),
                }
            }
            None => Err(Error::NA())
        }
    }
}

named!(get_command<(&str)>, do_parse!(
    tag!("/") >>
    command: map_res!(alphanumeric, str::from_utf8) >>
    (command)
));

named!(get_arguments<(&str)>, do_parse!(
    tag!("/") >>
    alphanumeric >>
    opt!(tag!("@SagiriBot")) >>
    tag!(" ") >> // TODO: Use Early Return
    arguments: map_res!(alt!(eof!() | eol | non_empty), str::from_utf8) >>
    (arguments)
));

// Telegram Sticker, See: https://core.telegram.org/bots/api#sticker
#[derive(Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub emoji: Option<String>,
    pub file_size: Option<Integer>,
}
