use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_message() -> String {
    "Hello from Rust WASM!".to_string()
}