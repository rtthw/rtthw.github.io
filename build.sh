rm ./docs/rtthw.wasm

# As far as I can tell, this is EXACTLY the same as:
# `wasm-pack build --out-dir docs --target no-modules`
cargo build -r --lib --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rtthw.wasm docs/rtthw.wasm
wasm-bindgen docs/rtthw.wasm --target no-modules --out-dir docs
