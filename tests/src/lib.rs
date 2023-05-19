extern crate alloc;

use js_sys::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_derive::TryFromJsValue;

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
    #[wasm_bindgen(typescript_type = "MyType | null")]
    pub type OptionMyType;
}

// Use this type in the function signature.
pub fn option_example(value: &OptionMyType) -> Result<OptionMyType, Error> {
    let js_value: &JsValue = value.as_ref();
    let typed_value: Option<MyType> = if js_value.is_null() {
        None
    } else {
        Some(MyType::try_from(js_value).map_err(|err| Error::new(&err))?)
    };

    // Now we have `typed_value: Option<MyType>`.

    // Return it
    // Note that by default `JsValue::from(None)` creates a `JsValue::UNDEFINED`.
    // We want it to be `null` (as we declared in the TS type).
    Ok(typed_value
        .map(JsValue::from)
        .unwrap_or(JsValue::NULL)
        .unchecked_into::<OptionMyType>())
}

// Vec<MyType> example

// To have a correct typing annotation generated for TypeScript, declare a custom type.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MyType[]")]
    pub type MyTypeArray;
}

// Use this type in the function signature.
pub fn vec_example(val: &MyTypeArray) -> Result<MyTypeArray, Error> {
    // Unpack the array

    let js_val: &JsValue = val.as_ref();
    let array: &js_sys::Array = js_val
        .dyn_ref()
        .ok_or_else(|| Error::new("The argument must be an array"))?;
    let length: usize = array
        .length()
        .try_into()
        .map_err(|err| Error::new(&format!("{}", err)))?;
    let mut typed_array = Vec::<MyType>::with_capacity(length);
    for js in array.iter() {
        let typed_elem = MyType::try_from(&js).map_err(|err| Error::new(&err))?;
        typed_array.push(typed_elem);
    }

    // Now we have `typed_array: Vec<MyType>`.

    // Return the array

    Ok(typed_array
        .into_iter()
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<MyTypeArray>())
}
