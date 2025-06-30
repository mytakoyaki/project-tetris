use wasm_bindgen::prelude::*;

mod game;
mod achievement;
mod utils;

pub use game::*;
pub use achievement::*;
pub use utils::*;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust Tetris Engine!", name)
} 