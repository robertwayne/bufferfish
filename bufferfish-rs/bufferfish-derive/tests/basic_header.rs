use bufferfish::{Bufferfish, Encodable, Encode};

#[test]
fn encode_struct() {
    #[derive(Encode, Debug)]
    #[bufferfish(0_u16)]
    struct Foo {
        bar: u8,
    }

    let foo = Foo { bar: 42 };
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 3);
}
