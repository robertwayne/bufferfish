# Bufferfish

Bufferfish is utility library for working with binary network messages between Rust and TypeScript, such as over WebSockets. It provides a simple API for serializing and deserializing data into binary arrays, as well as generating TypeScript definitions and deserialize functions from your Rust packet definitions.

_This library has an unstable API and is missing a variety of functionality. I can't recommend using it in production, although I am using it for my own production project._

## Repository Overview

There are two seperate libraries in this repo: one for Rust and one for TypeScript. Neither of the libraries have any required dependencies. The Rust version optionally uses the `unicode-width` crate for formatting buffer output when `pretty-print` is enabled. Additionally, the Rust version has a `derive` feature that enables a `#[derive(Serialize)]` macro.

The Rust crate is broken into three seperate crates:

### /bufferfish

`bufferfish` is a re-export of the other crates, as well as a `generate` function for use in `build.rs` files in order to generate TypeScript definitions from your Rust packet ID type. This is what users will interact with directly.

### /bufferfish-derive

`bufferfish_derive` is where the proc macro code for the `#[derive(Serialize)]` lives. This annotation implements `ToBufferfish` for the annotated type, allowing it to be serialized to a `Bufferfish` instance automatically.

### /bufferfish-internal

`bufferfish_internal`is the core library implementation _(trait / type definitions, byte and cursor logic, and errors)_.

### /bufferfish-ts

`bufferfish-ts` is the TypeScript library that provides the `Bufferfish` class with a mirrored API to the Rust version.

## Example

```rust
use bufferfish::{Serialize, ToBufferfish};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Debug)]
enum PacketId {
    Join,
}

// We need to make sure we can convert our enum to a u8, as that is the type
// Bufferfish uses to identify packets. You can use the `num_enum` crate and
// derive `IntoPrimitive` and `FromPrimitive` to remove this step completely.
impl From<PacketId> for u8 {
    fn from(id: PacketId) -> u8 {
        match id {
            PacketId::Join => 0,
        }
    }
}

// We annotate our packet with the #[Serialize] macro to enable automatic
// serialization to a Bufferfish.
//
// Additionally, we use the #[bufferfish] attribute to specify the packet ID.
#[derive(Serialize)]
#[bufferfish(PacketId::Join)]
struct JoinPacket {
    id: u32
    username: String,
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

    let packet = JoinPacket {
        id: 1,
        username: "Rob".to_string(),
    };
    let bf = packet.to_bufferfish()?;

    ws.send(Message::Binary(bf.into())).await?;

    Ok(())
}
```

### Using Generated Packet Readers

```typescript
const ws = new WebSocket("ws://127.0.0.1:3000")

ws.onmessage = (event) => {
  const bf = new Bufferfish(event.data)
  const packetId = bf.readUint8()

    if (packetId === PacketId.Join) {
        const packet = parseJoinPacket(bf)
    
        console.log(packet) // { packetId: 0, id: 1, username: "Rob" }
    }
}
```

### Manually Reading Packets

```typescript
const ws = new WebSocket("ws://127.0.0.1:3000")

ws.onmessage = (event) => {
    const bf = new Bufferfish(event.data)
    const packetId = bf.readUint8()

    if (packetId === PacketId.Join) {
        const id = bf.readUint32()
        const username = bf.readString()

        console.log({
            packetId,
            id,
            username,
        }) // { packetId: 0, id: 1, username: "Rob" }
    }
}
```



## Packet Generation

Bufferfish provides a `generate` function that can be used in `build.rs` _(or used in a CLI script, called by server at launch, etc)_ to generate TypeScript definitions and functions from your Rust packets, meaning your Rust server becomes the source of truth for all network messages.

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    bufferfish::generate("src/packet.rs", "../client/src/Packet.ts")?;

    Ok(())
}
```

### Example Generation

```rust
// This would be your server Rust code.
pub enum PacketId {
    Join = 0,
    Leave,
    Unknown = 255,
}

#[bufferfish(PacketId::Join)]
pub struct JoinPacket {
    pub id: u8,
    pub username: String,
}

#[bufferfish(PacketId::Leave)]
pub struct LeavePacket;
```

```typescript
/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */
/* Make sure your bundler is configured to inline TypeScript enums in order to avoid bloated codegen from the default TypeScript enum behaviour. */
import { Bufferfish } from 'bufferfish';

export enum PacketId {
    Join = 0,
    Leave = 1,
    Unknown = 255,
}

export interface JoinPacket {
    id: number;
    username: string;
}

export const parseJoinPacket = (bf: Bufferfish): JoinPacket => {
    const id = bf.readUint8();
    const username = bf.readString();

    return {
        id,
        username,
    };
};
```

## Tips

- I strongly recommend the usage of the [num_enum](https://github.com/illicitonion/num_enum) crate for deriving `IntoPrimitive` and `FromPrimitve` on your packet ID enum. This removes a lot of boilerplate 

## Security

Bufferfish functions ensure inputs are valid as a "best effort". Internal buffers are protected with a maximum capacity _(default of 1024 bytes)_, and will fail to construct if an input would cause the internal buffer to cross that threshold.

When reading data, you will always get the correct return type - however, you are still subject to corrupted data if the input was incorrect but technically valid. For example, if you call `read_u8` on a buffer that contains a `u16` at the cursor position, you will get a `u8` back, as the buffer has no way to know that it was originally encoded as a `u16`. It is valid data, but will very likely be an unexpected value.

This kind of problem should be protected against before operating on the buffer, based on what you're expecting.

## Contributing

Bufferfish welcomes any and all contributions; please open an issue before you work on any new features, though. Just note that the scope of this project is fairly tight, and I am not looking to cover a wider 'general' use-case; there are plenty of other full-featured options for that.

## License

Bufferfish source code is dual-licensed under either

- **[MIT License](/LICENSE-MIT)**
- **[Apache License, Version 2.0](/LICENSE-APACHE)**

at your option.
