use std::sync::mpsc::TryRecvError;

pub enum ConsumerTryError {
    TryRecvError(TryRecvError)
}
