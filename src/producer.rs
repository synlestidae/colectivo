//use topic::Topic;
use message::Message;
use producer_error::ProducerError;
use pub_sub::PubSub;

pub struct Producer {
    pubsub: PubSub<Message>
}

impl Producer {
    pub(crate) fn from_pubsub(pubsub: PubSub<Message>) -> Self {
        Self {
            pubsub: pubsub
        }
    }

    pub fn send(&self, message: Message) -> Result<(), ProducerError> {
        self.pubsub.send(message)?;
        Ok(())
    }
}
