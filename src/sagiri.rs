use std::marker::Sized;
use hyper::uri::RequestUri;
use hyper::server::{Server, Request, Response, Handler};

pub struct Message {
    pub from: String,
    pub at: String,
    pub date: String,
    pub command: String,
    pub argument: String,
}

pub trait Adapter: Send + Sync {
    fn onReceived(&self, content: String) -> Message;
    fn sendMessage(&self, msg: Message) -> Message;
    fn getWebHookUri(&self) -> RequestUri;
}

pub struct Sagiri {
    adapters: Vec<Box<Adapter>>,
}

impl Handler for Sagiri {
    fn handle(&self, mut req: Request, mut res: Response) {
        for adapter in &self.adapters {
            assert_eq!(req.uri, adapter.getWebHookUri());
        }
    }
}

impl Sagiri {
    pub fn add_adapter<T>(&mut self, adapter: T) where T: Adapter + 'static {
        &self.adapters.push(Box::new(adapter));
    }

    pub fn start_listen(self) {
        Server::http("localhost:8080").unwrap().handle(self).unwrap();
    }
}