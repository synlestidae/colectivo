use topic::Topic;
use message::Message;
use producer_error::ProducerError;

pub struct Producer {
}

impl Producer {
    pub fn new(_topic: Topic) -> Self {
        unimplemented!()
    }

    pub fn send(_message: Message) -> Result<(), ProducerError> {
        unimplemented!()
    }
}
