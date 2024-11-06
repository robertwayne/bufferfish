cd typescript
rm -rf dist
bun test
bun fmt
bun run bundle.js
bun build-types

cd ../rust
cargo fmt
cargo clippy
cargo clean
cargo build --release