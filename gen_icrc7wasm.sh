rm -rf wasm_files
mkdir wasm_files

cargo build --target wasm32-unknown-unknown --release --package icrc7
mv target/wasm32-unknown-unknown/release/icrc7.wasm wasm_files
