use bufferfish::{Bufferfish, Serialize, ToBufferfish};

enum PacketId {
    Ping,
}

impl Into<u8> for PacketId {
    fn into(self) -> u8 {
        match self {
            PacketId::Ping => 0,
        }
    }
}

#[test]
fn serialize_struct() {
    #[derive(Serialize, Debug)]
    #[bufferfish(PacketId::Ping)]
    struct Foo {
        bar: u8,
    }

    let foo = Foo { bar: 42 };
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 2);
}
