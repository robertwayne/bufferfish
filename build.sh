cd bufferfish-ts
rm -rf dist
turbo lint tsc fmt test build

cd ../bufferfish-rs
cargo fmt
cargo clippy
cargo clean
cargo build --release