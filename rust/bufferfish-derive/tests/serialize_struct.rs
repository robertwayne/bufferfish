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

#[test]
fn encode_unit_struct() {
    #[derive(Encode, Debug)]
    #[bufferfish(0_u16)]
    struct Foo;

    let foo = Foo;
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 2);
}

#[test]
fn encode_tuple_struct() {
    #[derive(Encode, Debug)]
    #[bufferfish(0_u16)]
    struct Foo(u8);

    let foo = Foo(42);
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 3);
}
