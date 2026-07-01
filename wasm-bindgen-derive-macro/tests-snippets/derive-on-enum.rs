use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_derive::TryFromJsValue;

#[derive(TryFromJsValue)] // error: can only be derived for structs
#[wasm_bindgen]
#[derive(Clone)]
pub enum MyType { Foo, Bar }

fn main() {}
