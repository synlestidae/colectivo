#![deny(warnings)]

pub mod message;
pub mod message_key;


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
