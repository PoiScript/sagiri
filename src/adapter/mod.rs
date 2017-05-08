pub mod matrix;
pub mod telegram;

use hyper::uri::RequestUri;

#[derive(Debug)]
pub enum Error {
    Api(String),
    TimeOut(String),
    Invalid(String),
    Nom(String),
    NA()
}

pub trait Message {
    fn get_sender_id(self) -> Result<String, Error>;
    fn get_sender_name(self) -> Result<String, Error>;
    fn get_chat_id(self) -> String;
    fn get_chat_name(self) -> Result<String, Error>;
    fn get_command(self) -> Result<String, Error>;
    fn get_argument(self) -> Result<String, Error>;
}

pub trait Adapter: Send + Sync {
    // Return Adapter's Name, Used to Identity
    fn name(&self) -> &str;

    // Return the URL that this adapter listen.
    fn webhook(&self) -> &String;

    // Parse the Response from the WebHook.
    fn parse(&self, content: String) -> Result<Box<Message>, Error>;

    // Send Message by HTTP Client.
    fn send(&self, msg: Box<Message>) -> Result<(), Error>;
}
