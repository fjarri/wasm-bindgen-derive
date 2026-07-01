extern crate alloc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_derive_macro::TryFromJsValue;

#[derive(TryFromJsValue)]
#[wasm_bindgen(js_name = 1)] // error: expected js_name to be a string
#[derive(Clone)]
struct MyTypeCustomName(usize);

fn main() {}
