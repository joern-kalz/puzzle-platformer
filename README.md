# puzzle-platformer
A puzzle-platformer game

cargo install 
cargo install wasm-bindgen-cli miniserve cargo-watch
rustup target add wasm32-unknown-unknown
miniserve . --index index.html -p 8080
cargo watch -i pkg/ -s "cargo build --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/debug/puzzle_platformer.wasm --out-dir ./pkg --target web"