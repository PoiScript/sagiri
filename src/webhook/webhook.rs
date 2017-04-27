use std::io::Read;
use std::str::FromStr;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use hyper::server::{Server, Request, Response, Handler};
use serde_json::from_str;

use matrix::bot::MatrixBot;
use telegram::bot::TelegramBot;

pub struct WebHook {
    matrix: MatrixBot,
    matrix_url: RequestUri,
    telegram: TelegramBot,
    telegram_url: RequestUri,
}

impl Handler for WebHook {
    fn handle(&self, mut req: Request, mut res: Response) {
        if req.method == Method::Post {
            *res.status_mut() = StatusCode::MethodNotAllowed
        }
        if req.uri == self.matrix_url {
            let mut content = String::new();
            req.read_to_string(&mut content).unwrap();
            println!("{}", content);
        } else if req.uri == self.telegram_url {
            let mut content = String::new();
            req.read_to_string(&mut content).unwrap();
            match from_str(&*content) {
                Ok(update) => self.telegram.handle(update),
                Err(_) => println!("Invalid Response")
            }
        } else {
            *res.status_mut() = StatusCode::Unauthorized
        }
    }
}

impl WebHook {
    pub fn new(matrix: MatrixBot, mx_token: String,
               telegram: TelegramBot, tg_token: String) -> WebHook {
        let matrix_url = RequestUri::from_str("/api/mx").unwrap();
        let telegram_url = RequestUri::from_str("/api/tg").unwrap();
        WebHook {
            matrix: matrix,
            matrix_url: matrix_url,
            telegram: telegram,
            telegram_url: telegram_url,
        }
    }

    pub fn start(self) {
        Server::http("localhost:8080").unwrap().handle(self).unwrap();
    }
}