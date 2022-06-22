cd typescript
rm -rf dist
npm run fmt
npm run build

cd ../rust
cargo fmt
cargo clean
cargo build --release