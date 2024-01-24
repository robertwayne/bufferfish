use bufferfish::{Bufferfish, Serialize, ToBufferfish};

#[test]
fn serialize_struct() {
    #[derive(Serialize, Debug)]
    struct Foo {
        bar: u8,
    }

    let foo = Foo { bar: 42 };
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 1);
}
