extern crate alloc;
use wasm_bindgen_derive_macro::TryFromJsValue;

#[derive(TryFromJsValue)]
// error: missing `#[wasm_bindgen]` annotation
#[derive(Clone)]
struct MyType(usize);

fn main() {}
