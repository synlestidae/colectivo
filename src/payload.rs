use std::convert::From;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Payload(pub Vec<u8>);

impl From<String> for Payload {
    fn from(string: String) -> Self {
        Payload(string.into_bytes())
    }
}

impl<'a> From<&'a str> for Payload {
    fn from(string: &str) -> Self {
        Payload(string.to_owned().into_bytes())
    }
}
