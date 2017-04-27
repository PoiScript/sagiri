use std::io::Read;
use hyper::Url;
use hyper::Client;
use hyper::method::Method;
use serde_json::{from_str, to_string};
use serde::de::DeserializeOwned;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use matrix::types::*;

pub struct MatrixBot {
    url: Url,
    token: String,
    client: Client
}

impl MatrixBot {
    pub fn new(homeserver: &str, token: &str) -> MatrixBot {
        let url = format!("{}/_matrix/client/r0/", homeserver);
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        MatrixBot {
            url: Url::parse(&url).unwrap(),
            token: token.to_string(),
            client: Client::with_connector(connector),
        }
    }

    pub fn get_profile(&self, user_id: String) -> Profile {
        let url = format!("{}{}/{}", &self.url, "profile", &user_id);

        MatrixBot::send_request(&self.client, Method::Get, &url, None)
    }

    pub fn send_event(&self, room_id: String, event: Text) -> String {
        let url = format!("{}{}/{}/send/{}/{}?access_token={}", &self.url, "rooms", &room_id, "m.text", 233, self.token);

        MatrixBot::send_request(&self.client, Method::Put, &url, Some(&to_string(&url).unwrap()))
    }

    // Matrix Client-Server API is RESTful
    pub fn send_request<T: DeserializeOwned>(client: &Client, method: Method, url: &str, body: Option<&str>) -> T {
        let req_body: String;

        let mut builder = match method {
            Method::Get => client.get(url),
            Method::Put => client.put(url),
            Method::Post => client.post(url),
            Method::Delete => client.delete(url),
            _ => panic!("Invalid Method")
        };

        if let Some(req_body) = body {
            builder = builder.body(req_body);
        }

        let mut res = builder.send().unwrap();

        let mut content = String::new();
        res.read_to_string(&mut content).unwrap();

        println!("{}", &content);

        from_str(&*content).unwrap()
    }
}