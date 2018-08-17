use std::sync::mpsc::SendError;
use message::Message;

pub enum ProducerError {
    SendError(SendError<Message>)
}
