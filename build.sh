cd bufferfish-ts
rm -rf dist
npm run fmt
npm run build

cd ../bufferfish-rs
cargo fmt
cargo clean
cargo build --release