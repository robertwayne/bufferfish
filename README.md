# Bufferfish

Bufferfish is a schemaless binary protocol designed for game networking.
Specifically, this library focuses on communication between TypeScript /
JavaScript clients and Rust servers via WebSockets.

It makes some compromises in order to accomodate this use-case, for example,
discarding any versioning / backwards compatability overhead. It does prefer to
sacrifice pure compactness in order to remain usable as partially read,
in-memory byte arrays - another primary goal.

There are two seperate libraries in this repo: one for Rust and one for
TypeScript / JavaScript. Neither of the libraries have any required
dependencies. The Rust version uses the `unicode-width` crate _(enabled by
default with the "pretty-print" feature)_ for formatting buffer output.

_This is currently a work-in-progress. Please don't use this._

**I strongly recommend you use a popular binary protocol instead, like
protobufs, msgpack, flatbuffers, etc.**

## Install

<!-- markdownlint-disable -->

<details>
<summary>Rust</summary>

    cargo add bufferfish --git https://github.com/robertwayne/bufferfish

### Feature Flags

<!-- markdownlint-disable -->
| Flag          | Default  | Description                                                                                    | Dependencies          |
|---------------|----------|------------------------------------------------------------------------------------------------|-----------------------|
| `pretty-print`| Enabled  | Enables pretty-printing for the Display impl.                                                  | `unicode-width`       |
| `impl-bytes`  | Disabled | Adds `From<Bytes>`, `From<BytesMut>`, and `From<Bufferfish>` impls for the `bytes` crate.      | `bytes`               |
<!-- markdownlint-enable -->

</details>

<details>
  <summary>TypeScript / JavaScript</summary>

NPM doesn't support pointing to sub-directories in git repositories, so you have
to download either `dist/bufferfish.es.js` or `dist/bufferfish.umd.js` directly
and include it locally. Make sure to include the `index.d.ts` file in your
project as well.

</details>

## Usage

<details>
<summary>Rust</summary>

```rust
// src/main.rs
use bufferfish::Bufferfish;

fn main() {
    let mut buf = Bufferfish::new();
    buf.write_string("Hello, world!")?;
    println!("{}", buf);

    let s = buf.read_string()?;
    println!("{}", s);

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
  // ...or...
  const bufferfish = require("bufferfish")

  const buf = new Bufferfish()
  buf.writeUint16(65535)
  console.table(buf.view())

  const n = buf.readUint16()
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

There's obviously a million binary protocols out there, most of them probably
better-designed than anything I could ever hope to create. That said, I wasn't
happy with the output of many of the language-agnostic, schemaless protocols.
They often included too much information _(of the ones I tried)_, and of the
language-specific ones I tried, I would have had to write a library to cover the
other language I needed regardless.

Some of my goals:

- Working with in-memory buffers should be easy; no need to deserialize an
  entire byte array before operating on its data.
- Defining specific values as fixed-length or variable-length is important.
- Writing to buffers should be painless; whether serializing an entire object or
  manually constructing a buffer.
- As little overhead as possible while still being operable on partial buffer
  reads without a schema.
- Remove backward compatability / versioning overhead.

## Current Restrictions

- Verbose. You have to manually construct and read your own buffers using the
  write_\* and read_\* functions.
- Selective bitpacking. By default, all fields are packed with a fixed-length.
  You can use the write_packed_\* and read_packed_\* functions to squish your
  packets down, but it is fairly limited.

## Todo

- Implement serializers/deserializers for both libraries.
- Implement variable-length integer encoding functions.
- Implement 64-bit numbers.
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

_**TODO:** Serializer/deserializer implementations will need to be more
context-aware of expected buffer formats._

## Spec

See [here](/SPECIFICATION.md).

## Contributing

Bufferfish welcomes any and all contributions; please open an issue before you
work on any new features, though. Just note that the scope of this project is
fairly tight, and I am not looking to cover a wider 'general' use-case; there
are plenty of other full-featured options for that.

## Self Notes

- Run tests with `cargo test -- --show-output` and `npm run test` from their
  respective directories.
- Run `./build.sh` to build both the Rust and TypeScript libraries.

## License

Bufferfish source code is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
