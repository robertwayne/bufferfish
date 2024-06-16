use bufferfish::{Bufferfish, Decodable, Decode};

#[test]
fn decode_struct() {
    #[derive(Decode, Debug, PartialEq)]
    #[bufferfish(0_u16)]
    struct Foo {
        bar: u8,
    }

    let mut bf = Bufferfish::from(vec![0, 0, 42]);
    let foo = Foo::decode(&mut bf).unwrap();

    assert_eq!(foo, Foo { bar: 42 });
}

#[test]
fn decode_unit_struct() {
    #[derive(Decode, Debug, PartialEq)]
    #[bufferfish(0_u16)]
    struct Foo;

    let mut bf = Bufferfish::from(vec![0, 0]);
    let foo = Foo::decode(&mut bf).unwrap();

    assert_eq!(foo, Foo);
}

#[test]
fn decode_tuple_struct() {
    #[derive(Decode, Debug, PartialEq)]
    #[bufferfish(0_u16)]
    struct Foo(u8);

    let mut bf = Bufferfish::from(vec![0, 0, 42]);
    let foo = Foo::decode(&mut bf).unwrap();

    assert_eq!(foo, Foo(42));
}
