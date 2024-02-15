# Bufferfish

Bufferfish is utility library for working with binary network messages between
Rust and TypeScript, such as over WebSockets.

## Repository Overview

There are two seperate libraries in this repo: one for Rust and one for
TypeScript / JavaScript. Neither of the libraries have any required
dependencies. The Rust version optionally uses the `unicode-width` crate for
formatting buffer output when `pretty-print` is enabled. Additionally, the Rust
version has a `derive` feature that enables a `#[derive(Serialize)]` macro.

The Rust crate is broken into three seperate crates: 


### /bufferfish

`bufferfish` is a re-export of the other crates, as well as a public
transpilation function for use in `build.rs` files in order to generate
TypeScript definitions from your Rust packet ID type. This is what users will
interact with directly. 

### /bufferfish-derive

`bufferfish_derive` is where the proc macro code for the `#[derive(Serialize)]`
lives. This annotation implements `ToBufferfish` for the annotated type,
allowing it to be serialized to a `Bufferfish` instance automatically. 

### /bufferfish-internal

`bufferfish_internal`is the core library implementation _(trait / type
definitions, byte and cursor logic, and errors)_.

_This is currently a work-in-progress with a wildly unstable API. **Please don't
use this**. I strongly recommend a popular binary protocol instead, like
protobufs, msgpack, flatbuffers, etc._

## Example

```rust
use bufferfish::{Serialize, ToBufferfish};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Debug)]
enum PacketId {
    Hello,
}

// We need to make sure we can convert our enum to a u8, as that is the type
// Bufferfish uses to identify packets.
impl From<PacketId> for u8 {
    fn from(id: PacketId) -> u8 {
        match id {
            PacketId::Hello => 0,
        }
    }
}

// We annotate our packet with the #[Serialize] macro to enable automatic
// serialization to a Bufferfish.
//
// Additionally, we use the #[bufferfish] attribute to specify the packet ID.
#[derive(Serialize)]
#[bufferfish(PacketId::Hello)]
struct HelloPacket {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                eprintln!("Error processing connection: {}", e);
            }
        });
    }

    Ok(())
}

async fn process(steam: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = accept_async(steam).await?;

    let packet = HelloPacket {
        data: "Hello World!".to_string(),
    };
    let bf = packet.to_bufferfish()?;

    ws.send(Message::Binary(bf.into())).await?;

    Ok(())
}

```

```typescript
const ws = new WebSocket("ws://127.0.0.1:3000")

ws.onmessage = (event) => {
  const bf = new Bufferfish(event.data)
  const message = bf.readString()

  console.log(message) // "Hello World!"
}
```

## Why?

There's a million binary protocols and libraries out there, but I wasn't happy
with the output of many of the language-agnostic binary protocols. Schemaless
ones often included too much header information, and schema-based protocols
often had APIs which did not feel native to the language. 

Some of my goals:

- First-class support for Rust and TypeScript.
- Automatic serialization of Rust types.
- Automatic transpilation of Rust packet header IDs to TypeScript.
- Working with in-memory byte buffers should be simple.
- Defining specific values as fixed-length or variable-length is important.
- Minimal overhead (eg. no versioning or complex header data)

## Safety

Bufferfish functions ensure inputs are valid as a "best effort". Internal
buffers are protected with a maximum capacity _(default of 1024 bytes)_, and
will fail to construct if an input would cause the internal buffer to cross that
threshold.

When reading data, you will always get the correct return type - however, you
are still subject to corrupted data if the input was incorrect but technically
valid. For example, if you call `read_u8` on a buffer that contains a `u16` at
the cursor position, you will get a `u8` back, as the buffer has no way to know
that it was originally encoded as a `u16`. It is valid data, but will very
likely be an unexpected value.

This kind of problem should be protected against before operating on the buffer,
based on what you're expecting.

## PacketID Transpilation

Bufferfish provides a `transpile` function that can be used in `build.rs` to
generate TypeScript definitions from your Rust packet ID type. This is useful
because it allows you to define your packet IDs in one place, and have them
inlined by your TypeScript bundler for minimal codegen.

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    bufferfish::transpile("src/packet_id.rs", "../client/src/PacketId.ts")?;

    Ok(())
}
```

If you prefer to manually generate the TypeScript definitions, you can use the
Python script in the `scripts` directory.

`python transpile.py -i src/packet_id.rs -o ../client/src/PacketId.ts`

## Contributing

Bufferfish welcomes any and all contributions; please open an issue before you
work on any new features, though. Just note that the scope of this project is
fairly tight, and I am not looking to cover a wider 'general' use-case; there
are plenty of other full-featured options for that.

## License

Bufferfish source code is dual-licensed under either

- **[MIT License](/LICENSE-MIT)**
- **[Apache License, Version 2.0](/LICENSE-APACHE)**

at your option.
