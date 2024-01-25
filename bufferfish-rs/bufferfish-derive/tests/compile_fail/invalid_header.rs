use bufferfish::Serialize;

#[derive(Serialize, Debug)]
#[bufferfish("Hello, world!")]
struct Foo {
    bar: u8,
}

fn main() {}
