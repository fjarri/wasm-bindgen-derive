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

#[doc = "this is documentation"] // testing that a NameValue attribute is processed correctly
#[derive(TryFromJsValue)]
#[wasm_bindgen(js_name = "SomeJsName")]
#[derive(Clone)]
struct MyTypeWithNameValue(usize);


#[doc = "this is documentation"]
#[derive(TryFromJsValue)]
// An additional nested attribute to check the handling of non-`js_name` attributes.
#[wasm_bindgen(inspectable, js_name = "SomeJsName")]
#[derive(Clone)]
struct MyTypeExtraNested(usize);

fn main() {}
