// We cannot define unit tests within a proc-macro crate, so we have to use integration tests.
// Also it is non-trivial to check for compilation errors, so we only check the happy paths for now.
// TODO: see how compilation errors are tested in `auto_impl` crate.

extern crate alloc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_derive_macro::TryFromJsValue;

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Clone)]
struct MyType(usize);

#[derive(TryFromJsValue)]
#[wasm_bindgen(js_name = "SomeJsName")]
#[derive(Clone)]
struct MyTypeCustomName(usize);

#[test]
fn pass() {
    let _x = MyType(1);
    let _y = MyTypeCustomName(2);
}
