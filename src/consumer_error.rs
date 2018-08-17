use std::sync::mpsc::RecvError;
use std::convert::From;

pub enum ConsumerError {
    RecvError(RecvError)
}

impl From<RecvError> for ConsumerError {
    fn from(err: RecvError) -> Self {
        ConsumerError::RecvError(err)
    }
}
