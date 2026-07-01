extern crate alloc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_derive_macro::TryFromJsValue;

#[derive(TryFromJsValue)] // error: can only be derived for structs
#[wasm_bindgen]
#[derive(Clone)]
pub enum MyType { Foo, Bar }

fn main() {}
