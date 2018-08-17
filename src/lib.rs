#![deny(warnings)]

extern crate pub_sub;

pub mod message;
pub mod message_key;
pub mod topic;
pub mod producer;
pub mod consumer;
pub mod consumer_error;
pub mod consumer_try_error;
pub mod producer_error;
pub mod payload;

use topic::Topic;
use consumer::Consumer;
use producer::Producer;
use message::Message;
use std::convert::Into;
use std::collections::BTreeMap;
use pub_sub::PubSub;

pub struct Colectivo {
    topics: BTreeMap<Topic, PubSub<Message>>
} 

impl Colectivo {
    pub fn new() -> Self {
        Self {
            topics: BTreeMap::new()
        }
    }

    pub fn producer<T: Into<Topic>>(&mut self, t: T) -> Producer {
        Producer::from_pubsub(self.get_pubsub(&t.into()))
    }

    pub fn consumer<T: Into<Topic>>(&mut self, t: T) -> Consumer {
        let pubsub = self.get_pubsub(&t.into());
        let subscription = pubsub.subscribe();
        Consumer::from_subscription(subscription)
    }

    fn get_pubsub(&mut self, topic: &Topic) -> PubSub<Message> {
        match self.topics.get(topic).map(|p| p.clone()) {
            Some(p) => p,
            None => self.new_topic(topic)
        }
    }

    fn new_topic(&mut self, topic: &Topic) -> PubSub<Message>{
        let pubsub = PubSub::new();
        self.topics.insert(topic.clone(), pubsub.clone());
        pubsub
    }
}


#[cfg(test)]
mod tests {
    use message::Message;
    use message_key::MessageKey;

    #[test]
    fn it_works() {
        let message = Message {
            index: 0,
            key: MessageKey::from_str("test_key"),
            payload: "This is a test message!".to_owned().into_bytes()
        };
        drop(message);
    }
}
