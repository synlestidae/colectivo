use message_key::MessageKey;
use std::convert::Into;
use payload::Payload;

#[derive(Clone)]
pub struct Message {
    pub index: u64,
    pub key: MessageKey,
    pub payload: Payload
}

unsafe impl Sync for Message {
}

unsafe impl Send for Message {
}

impl Message {
    pub fn new<K: Into<MessageKey>, M: Into<Payload>>(key: K, payload: M) -> Self {
        Message {
            index: 0,
            key: key.into(),
            payload: payload.into()
        }
    }
}
