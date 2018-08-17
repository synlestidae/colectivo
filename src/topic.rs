#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Topic(Vec<u8>);

impl From<String> for Topic {
    fn from(string: String) -> Self {
        Topic(string.into_bytes())
    }
}

impl<'a> From<&'a str> for Topic {
    fn from(string: &str) -> Self {
        Topic(string.to_owned().into_bytes())
    }
}
