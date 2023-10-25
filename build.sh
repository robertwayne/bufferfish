cd bufferfish-ts
rm -rf dist
bun lint
bun test
bun fmt
bun run bundle.js
bun tsc src/bufferfish.ts --declaration --emitDeclarationOnly --outfile dist/index.d.ts

cd ../bufferfish-rs
cargo fmt
cargo clippy
cargo clean
cargo build --release