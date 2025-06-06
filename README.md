# 🐡 bufferfish

`bufferfish` is utility library for working with binary network messages between Rust and TypeScript, such as over WebSockets. It provides a simple API for encoding and decoding data into binary arrays, as well as generating TypeScript definitions and encode/decode functions from your Rust code.

_This library has an unstable API. I can't recommend using it in production, although I am using it for my own production project._

## Table of Contents

  - [Repository Overview](#repository-overview)
    - [/rust/bufferfish](#rustbufferfish)
    - [/rust/bufferfish-derive](#rustbufferfish-derive)
    - [/rust/bufferfish-core](#rustbufferfish-core)
    - [/typescript/bufferfish](#typescriptbufferfish)
  - [Getting Started](#getting-started)
  - [Examples](#examples)
    - [Using Generated Decoding Functions (JavaScript)](#using-generated-decoding-functions-javascript)
    - [Manually Decoding a Bufferfish (JavaScript)](#manually-decoding-a-bufferfish-javascript)
  - [TypeScript Code Generation](#typescript-code-generation)
    - [Codegen Example](#codegen-example)
  - [Encodable / Decodable Types](#encodable--decodable-types)
  - [Feature Flags](#feature-flags)
  - [Notes](#notes)
  - [Security](#security)
  - [Contributing](#contributing)
  - [License](#license)


## Repository Overview

There are two seperate libraries in this repo: one for Rust and one for TypeScript. Neither of the libraries have any required dependencies. See the [Feature Flags](#feature-flags) section for optional dependencies.

The Rust library is broken into three seperate crates:

### /rust/bufferfish

`bufferfish` is a re-export of the other crates. It also includes the TypeScript codegen compiler and the generate function to interact with it. This is what users will interact with directly. General tests also live here.

### /rust/bufferfish-derive

`bufferfish_derive` is where the proc macros for the `#[derive(Encode)]` and `#[derive(Decode)]` macros live. These annotations implement `Encodable` and `Decodable` - respectively - for the annotated types.

### /rust/bufferfish-core

`bufferfish_core`is the primary library implementation. Trait and type definitions, byte and cursor logic, and error handling live here.

### /typescript/bufferfish

The TypeScript implementation lives here. The API is generally mirrored from the Rust version.

## Getting Started

Use `cargo add bufferfish` to add the Rust library to your project.

Use `bun install bufferfish` to add the TypeScript library to your project.

## Examples

```rust
use bufferfish::{Encode};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Encode)]
#[repr(u16)]
enum MessageId {
    Join,
}

// We need to make sure we can convert our enum to a u16, as that is the type
// bufferfish uses to identify messages. You can use the `num_enum` crate and
// derive `IntoPrimitive` and `FromPrimitive` to remove this step completely.
impl From<MessageId> for u16 {
    fn from(id: MessageId) -> u16 {
        match id {
            MessageId::Join => 0,
        }
    }
}

// We annotate our message with the #[Encode] and #[Decode] macros to enable
//  automatic encoding and decoding to or from a `Bufferfish`.
//
// Additionally, we use the #[bufferfish] attribute to specify the message ID.
#[derive(Encode)]
#[bufferfish(MessageId::Join)]
struct JoinMessage {
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

    let message = JoinMessage {
        id: 1,
        username: "Rob".to_string(),
    };
    let bf = message.to_bufferfish()?;

    ws.send(Message::Binary(bf.into())).await?;

    Ok(())
}
```

### Using Generated Decoding Functions (JavaScript)

These are built when defining a struct or enum in Rust with the `#[derive(Encode)]` macro after calling the `bufferfish::generate()` function.

```typescript
const ws = new WebSocket("ws://127.0.0.1:3000")
ws.binaryType = "arraybuffer"

ws.onmessage = (event) => {
  const bf = new Bufferfish(event.data)
  const messageId = bf.readUint16()

    if (messageId === MessageId.Join) {
        const message = decodeJoinMessage(bf)

        console.log(message) // { id: 1, username: "Rob" }
    }
}
```

### Manually Decoding a Bufferfish (JavaScript)

```typescript
const ws = new WebSocket("ws://127.0.0.1:3000")
ws.binaryType = "arraybuffer"

ws.onmessage = (event) => {
    const bf = new Bufferfish(event.data)
    const messageId = bf.readUint16()

    if (messageId === MessageId.Join) {
        const id = bf.readUint32()
        const username = bf.readString()

        console.log({
            id,
            username,
        }) // { id: 1, username: "Rob" }
    }
}
```

## TypeScript Code Generation

`bufferfish` provides a `generate` function that can be used in `build.rs` _(or used in a CLI script, called by server at launch, etc)_ to generate TypeScript definitions and functions from your Rust code, meaning your Rust server becomes the source of truth for all network messages, and reducing manually interacting with `bufferfish` on the client.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    bufferfish::generate("src/", "../client/src/generated/Message.ts")?;

    Ok(())
}
```

### Codegen Example

```rust
use bufferfish::Encode;

#[derive(Encode)]
pub enum MessageId {
    Join = 0,
    Leave,
    Unknown = 255,
}

impl From<MessageId> for u16 {
    fn from(id: MessageId) -> u16 {
        match id {
            MessageId::Join => 0,
            MessageId::Leave => 1,
            MessageId::Unknown => 255,
        }
    }
}

#[derive(Encode)]
#[bufferfish(MessageId::Join)]
pub struct JoinMessage {
    pub id: u8,
    pub username: String,
}

#[derive(Encode)]
#[bufferfish(MessageId::Leave)]
pub struct LeaveMessage;
```

```typescript
/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */
import { Bufferfish } from 'bufferfish';

export enum MessageId {
    Join = 0,
    Leave = 1,
    Unknown = 255,
}

export interface JoinMessage {
    id: number
    username: string
}

export const decodeJoinMessage = (bf: Bufferfish): JoinMessage => {
    return {
        id: bf.readUint8() as number,
        username: bf.readString() as string,
    }
}
```

## Encodable / Decodable Types

Supported Types             | Decodes As
----------------------------|----------------------
`u8`                        | `number`
`u16`                       | `number`
`u32`                       | `number`
`i8`                        | `number`
`i16`                       | `number`
`i32`                       | `number`
`bool`                      | `boolean`
`String`                    | `string`
`Vec<T> where T: Encodable` | `Array<T>`
`T where T: Encodable`      | `object` or primitive

_*The reverse is true for decoding._

## Feature Flags

Flag           | Default  | Description                                                     | Dependencies
---------------|----------|-----------------------------------------------------------------|---------------------------------------------------
`pretty-print` | Disabled | Enables pretty-printing of buffer output.                       | `unicode-width`
`derive`       | Disabled | Enables the `#[derive(Encode)]` and `#[derive(Decode)]` macros. | `syn`, `quote`, `proc-macro2`, `proc-macro-error` |
`with-bytes`   | Disabled | Enables the From/Into impls for the `bytes::Bytes` type.                    | `bytes`                                           |

## Notes

- I recommend using the [num_enum](https://github.com/illicitonion/num_enum) crate for deriving `IntoPrimitive` and `FromPrimitve` on enums you wish to `Encode`. This removes a lot of boilerplate.
- Enums in TypeScript are often mentioned as a "bad" feature, and this is generally true when considering typical web development use-cases. In the case of a list of "op codes" mapping to dev-friendly names, however, they are actually really useful. Modern bundlers - like `esbuild` - [can actually inline them, meaning we just get integer literals in the final output.](https://sombia.com/posts/typescript-enums).

## Security

`bufferfish` functions ensure inputs are valid as a "best effort". Internal buffers are constructed with a maximum capacity _(default of 1024 bytes)_, and will fail to construct if an input would cause the internal buffer to cross that threshold.

When reading data, you will always get the correct return type - however, you are still subject to corrupted data if the input was incorrect but technically valid. For example, if you call `read_u8` on a buffer that contains a `u16` at the cursor position, you will get a `u8` back, as the buffer has no way to know that it was originally encoded as a `u16`. It is valid data, but will very likely be an unexpected value.

This kind of problem should be dealt with before operating on the buffer.

Decoding an oversized bufferfish via the `Decode` trait will just ignore / discard the additional data, as it is only going to read specific byte lengths generated by the `Encodable` impl.

Decoding an undersized bufferfish will return a `BufferfishError::FailedWrite`.

## Contributing

`bufferfish` is open to contributions, however it should be noted that the library was created for my own game projects, and I am not interested in making it widely general-purpose. If you have a feature request or bug fix that you think would be useful to others, feel free to open an issue or PR either way.

## License

`bufferfish` source code is dual-licensed under either

- **[MIT License](/LICENSE-MIT)**
- **[Apache License, Version 2.0](/LICENSE-APACHE)**

at your option.
