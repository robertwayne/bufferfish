#[cfg(feature = "derive")]
mod tests {
    use bufferfish_core::{Bufferfish, Encodable};
    use bufferfish_derive::Encode;

    #[allow(dead_code)]
    enum MessageId {
        Ping,
    }

    impl From<MessageId> for u16 {
        fn from(message_id: MessageId) -> u16 {
            match message_id {
                MessageId::Ping => 0,
            }
        }
    }

    #[derive(Encode, Debug)]
    #[bufferfish(MessageId::Ping)]
    struct Foo {
        bar: u8,
    }

    #[test]
    fn encode_struct() {
        let foo = Foo { bar: 42 };
        let bf: Bufferfish = foo.to_bufferfish().unwrap();

        assert_eq!(bf.len(), 3);
    }
}
