#![deny(warnings)]

extern crate pub_sub;

pub mod message;
pub mod message_key;
pub mod topic;
pub mod producer;
pub mod consumer;
pub mod consumer_error;
pub mod consumer_try_error;
pub mod producer_error;
pub mod payload;
pub mod colectivo;

pub use message::*;
pub use message_key::*;
pub use topic::*;
pub use producer::*;
pub use consumer::*;
pub use consumer_error::*;
pub use consumer_try_error::*;
pub use producer_error::*;
pub use payload::*;
pub use colectivo::*;

#[cfg(test)]
mod tests {
    use message::Message;
    use colectivo::Colectivo;

    #[test]
    fn it_creates_a_message() {
        let msg = Message::new("test_key", "Hello world!");
        assert_eq!(msg.key.0, "test_key".to_string().into_bytes());
        assert_eq!(msg.payload.0, "Hello world!".to_string().into_bytes());
    }

    #[test]
    fn it_creates_a_colectivo() {
        Colectivo::new();
    }

    #[test] 
    fn it_creates_producer_and_consumer() {
        let mut c = Colectivo::new();
        c.producer("test_topic");
        c.consumer("test_topic");
    }

    #[test] 
    fn it_produces_and_consumers_one_message() {
        let mut c = Colectivo::new();
        let prod = c.producer("test_topic");
        let con = c.consumer("test_topic");
        let msg = Message::new("test_message", "Just hanging around");
        prod.send(msg.clone()).expect("Message didn't send");
        assert_eq!(msg.payload, con.try_recv().expect("Message was not available").payload);
    }

    #[test] 
    fn it_produces_and_consumes_messages() {
        let mut c = Colectivo::new();
        let prod = c.producer("test_topic");
        let con = c.consumer("test_topic");

        let msg1 = Message::new("test_message", "The blockbuster");
        let msg2 = Message::new("test_message", "The bust");
        let msg3 = Message::new("test_message", "The cash grab");

        prod.send(msg1.clone()).unwrap();
        prod.send(msg2.clone()).unwrap();
        prod.send(msg3.clone()).unwrap();

        assert_eq!(msg1.payload, con.try_recv().unwrap().payload);
        assert_eq!(msg2.payload, con.try_recv().unwrap().payload);
        assert_eq!(msg3.payload, con.try_recv().unwrap().payload);
    }

    #[test] 
    fn it_produces_and_consumers_messages_on_two_topics() {
        let mut c = Colectivo::new();
        let hp_prod = c.producer("harry_potter");
        let lotr_prod = c.producer("lord_of_the_rings");
        let hp_con = c.consumer("harry_potter");
        let lotr_con = c.consumer("lord_of_the_rings");
        let gof_con = c.consumer("game_of_thrones");

        let msg1 = Message::new("book", "Philosopher's Stone");
        let msg2 = Message::new("book", "Chamber of secrets");
        let msg3 = Message::new("book", "Prisoner of Azkaban");
        let msg4 = Message::new("book", "Fellowship of the Ring");
        let msg5 = Message::new("book", "The Two Towers");


        hp_prod.send(msg1.clone()).unwrap();
        lotr_prod.send(msg4.clone()).unwrap();
        hp_prod.send(msg2.clone()).unwrap();
        lotr_prod.send(msg5.clone()).unwrap();
        hp_prod.send(msg3.clone()).unwrap();

        assert_eq!(msg1.payload, hp_con.try_recv().unwrap().payload);
        assert_eq!(msg2.payload, hp_con.try_recv().unwrap().payload);
        assert_eq!(msg4.payload, lotr_con.try_recv().unwrap().payload);
        assert_eq!(msg5.payload, lotr_con.try_recv().unwrap().payload);
        assert_eq!(msg3.payload, hp_con.try_recv().unwrap().payload);

        assert!(gof_con.try_recv().is_err());
    }
}
