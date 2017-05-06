use std::io::Read;
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::server::{Server, Request, Response, Handler};
use adapter::{Adapter, Message};
use command::Command;


pub struct Sagiri {
    adapters: Vec<Box<Adapter>>,
    commands: Vec<Box<Command>>,
}

impl Handler for Sagiri {
    fn handle(&self, mut req: Request, mut res: Response) {
        for adapter in &self.adapters {
            // TODO: Identify the request uri then call `adapter.parse()`
        }
        *res.status_mut() = StatusCode::Ok;
    }
}

impl Sagiri {
    pub fn add_adapter<T>(&mut self, adapter: T) where T: Adapter + 'static {
        &self.adapters.push(Box::new(adapter));
    }

    pub fn receive<T>(&self, msg: T) where T: Message {
        for command in &self.commands {
            // TODO: Call `command.run`
        }
    }

    pub fn start_listen(self) {
        Server::http("localhost:8080").unwrap().handle(self).unwrap();
    }
}