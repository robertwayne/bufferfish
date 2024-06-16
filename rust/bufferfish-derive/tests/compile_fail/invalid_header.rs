use bufferfish::Encode;

#[derive(Encode, Debug)]
#[bufferfish("Hello, world!")]
struct Foo {
    bar: u8,
}

fn main() {}
