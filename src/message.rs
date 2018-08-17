use message_key::MessageKey;

pub struct Message {
    pub index: u64,
    pub key: MessageKey,
    pub payload: Vec<u8>
}
