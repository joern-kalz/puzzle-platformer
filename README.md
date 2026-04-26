# puzzle-platformer

A puzzle-platformer game

## Development Setup

Install [Rust](https://rust-lang.org/).

Install required Rust tools.

```sh
cargo install wasm-bindgen-cli miniserve cargo-watch
```

Enable the WebAssembly compilation target.

```sh
rustup target add wasm32-unknown-unknown
```

Build WebAssembly binary.

```sh
cargo build --target wasm32-unknown-unknown
```

Build JavaScript glue code.


```sh
wasm-bindgen target/wasm32-unknown-unknown/debug/puzzle_platformer.wasm \
    --out-dir ./pkg \
    --target web
```

Serve website locally.

```sh
miniserve . --index index.html -p 8080
```

Open [http://localhost:8080/](http://localhost:8080/) in your browser.

Watch for file changes and automatically rebuild.

```sh
cargo watch -i pkg/ -s "
    cargo build --target wasm32-unknown-unknown && \
    wasm-bindgen target/wasm32-unknown-unknown/debug/puzzle_platformer.wasm \
      --out-dir ./pkg \
      --target web"
```