echo "Updating TypeScript dependencies..."
cd typescript
npm upgrade &> /dev/null

echo "Updating Rust dependencies..."
cd ../rust
cargo update --quiet

echo "Bufferfish is all up to date! ✨"