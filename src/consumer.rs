use consumer_error::ConsumerError;
use consumer_try_error::ConsumerTryError;
use message::Message;

pub struct Consumer {
}

impl Consumer {
    pub fn try_recv(&self) -> Result<Message, ConsumerTryError> {
        unimplemented!()
    }

    pub fn recv(&self) -> Result<Message, ConsumerError> {
        unimplemented!()
    }
}
