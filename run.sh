cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/linalg.wasm ./pkg/
typst compile pkg/main.typ
