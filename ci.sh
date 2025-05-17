cd rust
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo test -p bufferfish-core -p bufferfish -p bufferfish-derive --lib --all-features --all-targets

cd ../typescript
bun tsc
bun fmt:check
bun test