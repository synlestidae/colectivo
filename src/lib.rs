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

pub struct Colectivo {

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
