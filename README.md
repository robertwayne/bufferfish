# Bufferfish

Bufferfish is a schemaless binary protocol designed for game networking focusing
on communication between Rust and TypeScript / JavaScript.

## Repository Overview

There are two seperate libraries in this repo: one for Rust and one for
TypeScript / JavaScript. Neither of the libraries have any required
dependencies. The Rust version optionally uses the `unicode-width` crate for
formatting buffer output when `pretty-print` is enabled. Additionally, the Rust
version has a `derive` feature that enables a `#[derive(Serialize)]` macro.

The Rust crate is broken into three seperate crates: `bufferfish` is a re-export
of the other crates. This is what users will interact with directly.
`bufferfish_derive` is where the proc macro code for the `#[derive(Serialize)]`
lives, and `bufferfish_internal` is the core library implementation _(trait /
type definitions, logic, errors)_.

_This is currently a work-in-progress with a wildly unstable API. **Please don't
use this**. I strongly recommend a popular binary protocol instead, like
protobufs, msgpack, flatbuffers, etc._

## Basic Example
```rust
use bufferfish::{ToBufferfish, Serialize};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};

#[derive(Serialize)]
struct Foo {
    bar: String,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(process(stream));
    }
}

async fn process(steam: TcpStream) {
    let mut ws = tokio_tungstenite::accept_async(steam).await.unwrap();

    let foo = Foo {
        bar: "Hello World!".to_string(),
    };
    let bf = foo.to_bufferfish().unwrap();

    ws.send(tokio_tungstenite::tungstenite::Message::Binary(bf.into()))
        .await
        .unwrap()
}
```

```js
const ws = new WebSocket("ws://127.0.0.1:3000")

ws.onmessage = (event) => {
  const bf = new Bufferfish(event.data)
  const message = bf.readString()

  console.log(message) // "Hello World!"
}
```

## Installation

<!-- markdownlint-disable -->

<details>
<summary>Rust</summary>

    cargo add bufferfish --git https://github.com/robertwayne/bufferfish

### Feature Flags

<!-- markdownlint-disable -->
| Flag           | Default  | Description                                   | Dependencies    |
|----------------|----------|-----------------------------------------------|-----------------|
| `pretty-print` | Disabled | Enables pretty-printing for the Display impl. | `unicode-width` |
| `derive`       | Disabled | Enables the `#derive(Serialize)` macro.`       | `bufferfish_derive` |
<!-- markdownlint-enable -->

</details>

<details>
  <summary>TypeScript / JavaScript</summary>

Node doesn't support pointing to sub-directories in git repositories, so the
simplest way to use Bufferfish would be to clone the repository and run `pnpm
link <path>`. You should end up with something like this in your `package.json`:
`"bufferfish": "link:../bufferfish/bufferfish-ts"` - based on wherever your
cloned repo is located.

</details>

## Usage

<details>
<summary>Rust</summary>

### Manually Writing a Bufferfish

```rust
// src/main.rs
use bufferfish::Bufferfish;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Manually writing to a Bufferfish 
    let mut bf = Bufferfish::new();
    bf.write_string("Hello, world!")?;
    println!("{}", bf);

    let s = bf.read_string()?;
    println!("{}", s);

    Ok(())
}
```

### Deriving `Serialize`

```rust
// src/main.rs
use bufferfish::Bufferfish;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Serialize)]
    struct Foo {
        bar: u8,
    }

    let foo = Foo { bar: 42 };
    let bf: Bufferfish = foo.write()?;

    println!("{}", bf);

    Ok(())
}
```

Output:

     Byte:  0  13  72  101  108  108  111  44  32  119  111  114  108  100  33
    Index:  0   1   2    3    4    5    6   7   8    9   10   11   12   13  14

    Hello, world!

</details>

<details>
  <summary>TypeScript / JavaScript</summary>

  ```ts
  import { Bufferfish } from "bufferfish"

  const bf = new Bufferfish()
  bf.writeUint16(65535)
  console.table(bf.view())

  const n = bf.readUint16()
  console.log(n)
  ```

  Output:

    ┌─────────┬────────┐
    │ (index) │ Values │
    ├─────────┼────────┤
    │    0    │  255   │
    └─────────┴────────┘

    65535

</details>

<!-- markdownlint-enable -->

## Why?

There's a million binary protocols out there, most of them probably
better-designed than anything I could ever hope to create. That said, I wasn't
happy with the output of many of the language-agnostic, schemaless protocols.
They often included too much information _(of the ones I tried)_, and of the
language-specific ones I tried, I would have had to write a library to cover the
other language I needed.

Some of my goals:

- Working with in-memory buffers should be easy; no need to deserialize an
  entire byte array before operating on its data. Peeking without advancing a
  cursor should be possible.
- Defining specific values as fixed-length or variable-length is important.
- Writing to buffers should be painless; whether serializing an entire object or
  manually constructing a buffer.
- As little overhead as possible while still being operable on partial buffer
  reads without a schema.
- Remove backward compatability / versioning overhead.

## Todo

- Implement serializers/deserializers for both libraries.
- Implement variable-length integer encoding functions.
- Implement 64-bit numbers. Maybe? JS makes this weird.
- Implement a simple bitpacker; make it seamless & automatic.
- How to solve a desire for fixed-length mixed with bitpacked values?
- Can we use VL encoding on packet IDs? _(7 bit + 8 bit)_ with the first bit as
  a flag for 1-byte or 2-byte encoding?
- Best way to automatically detect the last value in a serialized buffer and
  skip length encoding?
- Define a proper spec for the structure.
- Add WebSocket server-client example (basic chat server?).

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
