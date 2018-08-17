use std::convert::From;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct MessageKey(pub Vec<u8>);

impl From<String> for MessageKey {
    fn from(string: String) -> Self {
        MessageKey(string.into_bytes())
    }
}

impl<'a> From<&'a str> for MessageKey {
    fn from(string: &str) -> Self {
        MessageKey(string.to_owned().into_bytes())
    }
}

impl MessageKey {
    pub fn from_str(key_str: &str) -> Self {
        MessageKey(key_str.to_string().into_bytes())
    }

}
