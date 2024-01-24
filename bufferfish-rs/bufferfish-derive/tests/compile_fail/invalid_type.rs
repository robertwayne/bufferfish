use bufferfish_derive::Serialize;

#[derive(Serialize, Debug)]
struct Foo {
    bar: Vec<u8>,
}

fn main() {}
