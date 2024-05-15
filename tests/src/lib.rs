extern crate alloc;

use js_sys::Error;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_derive::{
    into_js_array, into_js_option, try_from_js_array, try_from_js_option, TryFromJsValue,
};

// Derive `TryFromJsValue` for the target structure (note that it has to come
// before the `[#wasm_bindgen]` attribute, and requires `Clone`):
#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq)]
pub struct MyType(usize);

// A constructor for testing purposes
impl MyType {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

// Option<MyType> example

// To have a correct typing annotation generated for TypeScript, declare a custom type.
#[wasm_bindgen]
extern "C" {
    /// `MyType` or `undefined`
    #[wasm_bindgen(typescript_type = "MyType | undefined")]
    pub type OptionMyType;
}

// Use this type in the function signature.
pub fn option_example(value: &OptionMyType) -> Result<OptionMyType, Error> {
    // Use a helper to extract the typed value
    let typed_value = try_from_js_option::<MyType>(value).map_err(|err| Error::new(&err))?;

    // Now we have `typed_value: Option<MyType>`.

    // Return it
    // Note that if `typed_value` is `None`, `into_js_option()` creates a `JsValue::UNDEFINED`.
    Ok(into_js_option(typed_value))
}

// Vec<MyType> example

// To have a correct typing annotation generated for TypeScript, declare a custom type.
#[wasm_bindgen]
extern "C" {
    /// An array of `MyType`
    #[wasm_bindgen(typescript_type = "MyType[]")]
    pub type MyTypeArray;
}

// Use this type in the function signature.
pub fn vec_example(val: &MyTypeArray) -> Result<MyTypeArray, Error> {
    // Use a helper to extract the typed array
    let typed_array = try_from_js_array::<MyType>(val).map_err(|err| Error::new(&err))?;

    // Now we have `typed_array: Vec<MyType>`.

    // Return the array
    Ok(into_js_array(typed_array))
}
