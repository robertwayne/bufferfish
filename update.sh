echo "Updating TypeScript dependencies..."
cd bufferfish-ts
npm upgrade &> /dev/null

echo "Updating Rust dependencies..."
cd ../bufferfish-rs
cargo update --quiet

echo "Bufferfish is all up to date! ✨"