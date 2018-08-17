pub struct MessageKey(Vec<u8>);

impl MessageKey {
    pub fn from_str(key_str: &str) -> Self {
        MessageKey(key_str.to_string().into_bytes())
    }

}
