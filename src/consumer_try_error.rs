use std::sync::mpsc::TryRecvError;
use std::convert::From;

pub enum ConsumerTryError {
    TryRecvError(TryRecvError)
}

impl From<TryRecvError> for ConsumerTryError {
    fn from(err: TryRecvError) -> Self {
        ConsumerTryError::TryRecvError(err)
    }
}
