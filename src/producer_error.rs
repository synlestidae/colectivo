use std::sync::mpsc::SendError;
use message::Message;
use std::convert::From;

pub enum ProducerError {
    SendError(SendError<Message>)
}

impl From<SendError<Message>> for ProducerError {
    fn from(err: SendError<Message>) -> Self {
        ProducerError::SendError(err)
    }
}
