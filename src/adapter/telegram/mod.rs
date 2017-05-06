use std::io::Read;
use hyper::Url;
use hyper::Client;
use serde_json::from_str;
use serde::de::DeserializeOwned;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::uri::RequestUri;
use std::str::FromStr;

use adapter::{Adapter, Error};
use adapter::Message as sMessage;

mod types;

use self::types::*;

pub const API_URL: &'static str = "https://api.adapter.telegram.org/bot";

pub struct TelegramAdapter {
    url: Url,
    client: Client,
    webhook_url: RequestUri,
}

impl Adapter for TelegramAdapter {
    fn name(&self) -> &str {
        "Telegram"
    }

    // Return the URL that this adapter listen.
    fn webhook(&self) -> String {
        "test".to_string()
    }

    // Parse the Response from the WebHook.
    fn parse(&self, content: String) -> Result<Box<sMessage>, Error> {
        Err(Error::Api("test".to_string()))
    }

    // Send Message by HTTP Client.
    fn send(&self, msg: Box<sMessage>) -> Result<(), Error> {
        Err(Error::Api("test".to_string()))
    }
}

impl TelegramAdapter {
    pub fn new(token: &str) -> TelegramAdapter {
        let url = format!("{}{}/", API_URL, token);
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let webhook_url = format!("/api/tg/{}/", token);

        TelegramAdapter {
            url: Url::parse(&url).unwrap(),
            client: Client::with_connector(connector),
            webhook_url: RequestUri::from_str(&*webhook_url).unwrap(),
        }
    }

    pub fn set_webhook(&self, token: &str, domain: &str, max_connections: Option<Integer>,
                       allowed_updates: Option<&str>) -> bool {
        let webhook_url = format!("{}/api/tg/{}/", domain, token);

        let mut url = self.url.join("sendMessage").unwrap();

        url.query_pairs_mut().append_pair("url", &*webhook_url);
        if let Some(conn) = max_connections {
            url.query_pairs_mut().append_pair("max_connections", &*conn.to_string());
        }
        if let Some(updates) = allowed_updates {
            url.query_pairs_mut().append_pair("allowed_updates", updates);
        }

        // TODO: Handle the Error
        Self::post_request(&self.client, url.as_str()).unwrap()
    }

    pub fn get_me(&self) -> User {
        let url = &self.url.join("getMe").unwrap();

        // TODO: Handle the Error
        Self::post_request(&self.client, url.as_str()).unwrap()
    }

    pub fn send_message(&self, chat_id: Integer, text: &str) -> Message {
        let mut url = self.url.join("sendMessage").unwrap();

        url.query_pairs_mut()
            .append_pair("chat_id", &*chat_id.to_string())
            .append_pair("text", text);

        // TODO: Handle the Error
        Self::post_request(&self.client, url.as_str()).unwrap()
    }

    // Telegram Bot API supports both GET and POST, so one post function is enough.
    fn post_request<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T, Error> {
        let mut res = client.post(url).send().unwrap();

        let mut content = String::new();
        res.read_to_string(&mut content).unwrap();

        // TODO: Handle the Error
        match from_str(&*content).unwrap() {
            Response { ok: true, result: Some(result), .. } => {
                Ok(result)
            }
            Response { ok: false, description: Some(description), .. } => {
                Err(Error::Api(description))
            }
            _ => {
                Err(Error::Invalid("Invalid Response".into()))
            }
        }
    }
}