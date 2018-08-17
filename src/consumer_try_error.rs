use std::sync::mpsc::TryRecvError;
use std::convert::From;

#[derive(Debug)]
pub enum ConsumerTryError {
    TryRecvError(TryRecvError)
}

impl From<TryRecvError> for ConsumerTryError {
    fn from(err: TryRecvError) -> Self {
        ConsumerTryError::TryRecvError(err)
    }
}
