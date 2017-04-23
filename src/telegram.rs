extern crate reqwest;
extern crate env_logger;
extern crate url;

use std::io::Read;
use std::io::Result;
use self::url::Url;
use reqwest::Client;

pub const API_URL: &'static str = "https://api.telegram.org/bot";

pub struct TelegramBot {
    url: Url,
    client: Client
}

impl TelegramBot {
    pub fn new(token: &str) -> Result<TelegramBot> {
        let url = format!("{}{}/", API_URL, token);
        Ok(TelegramBot {
            url: Url::parse(&url).unwrap(),
            client: Client::new().unwrap(),
        })
    }

    pub fn get_me(&self) -> String {
        let url = &self.url.join("getMe").unwrap();
        Self::post_request(&self.client, url.as_str())
    }

    pub fn send_message(&self, chat_id: i64, text: &str) -> String {
        let mut url = self.url.join("sendMessage").unwrap();
        url.query_pairs_mut()
            .append_pair("chat_id", &*chat_id.to_string())
            .append_pair("text", text);
        Self::post_request(&self.client, url.as_str())
    }


    fn post_request(client: &Client, url: &str) -> String {
        let mut res = client.post(url)
            .send()
            .unwrap();

        let mut content = String::new();
        res.read_to_string(&mut content).unwrap();

        content
    }
}