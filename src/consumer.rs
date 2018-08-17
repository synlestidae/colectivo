use consumer_error::ConsumerError;
use consumer_try_error::ConsumerTryError;
use message::Message;
use pub_sub::Subscription;

pub struct Consumer {
    subscription: Subscription<Message>
}

impl Consumer {
    pub(crate) fn from_subscription(subscription: Subscription<Message>) -> Self {
        Self {
            subscription: subscription
        }
    }
    pub fn try_recv(&self) -> Result<Message, ConsumerTryError> {
        let msg = self.subscription.try_recv()?;
        Ok(msg)
    }

    pub fn recv(&self) -> Result<Message, ConsumerError> {
        let msg = self.subscription.recv()?;
        Ok(msg)
    }
}
