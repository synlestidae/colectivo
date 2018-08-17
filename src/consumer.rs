use consumer_error::ConsumerError;
use consumer_try_error::ConsumerTryError;
use message::Message;
use pub_sub::Subscription;

pub struct Consumer {
    _subscription: Subscription<Message>
}

impl Consumer {
    pub(crate) fn from_subscription(subscription: Subscription<Message>) -> Self {
        Self {
            _subscription: subscription
        }
    }
    pub fn try_recv(&self) -> Result<Message, ConsumerTryError> {
        unimplemented!()
    }

    pub fn recv(&self) -> Result<Message, ConsumerError> {
        unimplemented!()
    }
}
