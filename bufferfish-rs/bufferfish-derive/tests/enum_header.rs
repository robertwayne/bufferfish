use bufferfish::{Bufferfish, Serialize, ToBufferfish};

enum PacketId {
    Ping,
}

impl From<PacketId> for u8 {
    fn from(packet_id: PacketId) -> u8 {
        match packet_id {
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
