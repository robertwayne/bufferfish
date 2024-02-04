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

#[test]
fn serialize_unit_struct() {
    #[derive(Serialize, Debug)]
    struct Foo;

    let foo = Foo;
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 0);
}

#[test]
fn serialize_tuple_struct() {
    #[derive(Serialize, Debug)]
    struct Foo(u8);

    let foo = Foo(42);
    let bf: Bufferfish = foo.to_bufferfish().unwrap();

    assert_eq!(bf.len(), 1);
}
