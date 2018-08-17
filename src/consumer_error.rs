use std::sync::mpsc::RecvError;

pub enum ConsumerError {
    RecvError(RecvError)
}
