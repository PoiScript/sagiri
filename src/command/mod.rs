use adapter::Message;

pub trait Command: Send + Sync {
    fn handle(&self, msg: Message) -> Option<Box<Message>>;
    fn match_command(&self, command: String) -> bool;
}