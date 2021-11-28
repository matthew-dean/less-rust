use wasm_bindgen::prelude::*;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod utils;
pub mod parser;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    
    Ok(())
}
