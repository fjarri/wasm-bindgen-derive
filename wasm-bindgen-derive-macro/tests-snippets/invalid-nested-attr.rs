use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_derive::TryFromJsValue;

#[derive(TryFromJsValue)]
#[wasm_bindgen(js_name "SomeJsName")] // nested attributes parsing will fail here
#[derive(Clone)]
struct MyTypeCustomName(usize);

fn main() {}
