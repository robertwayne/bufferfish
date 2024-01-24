use bufferfish::{Bufferfish, BufferfishWrite, Serialize};

#[test]
fn bf_derive_serialize() {
    #[derive(Serialize, Debug)]
    struct Foo {
        bar: u8,
    }

    let foo = Foo { bar: 42 };
    let bf: Bufferfish = foo.write().unwrap();

    assert_eq!(bf.len(), 1);
}