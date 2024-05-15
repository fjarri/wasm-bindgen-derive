/*!
This is a specialized crate exporting a derive macro [`TryFromJsValue`]
that serves as a basis for workarounds for some lapses of functionality in
[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).


## Optional arguments and return values

`wasm-bindgen` supports method arguments of the form `Option<T>`,
where `T` is an exported type, but it has an unexpected side effect on the JS side:
the value passed to a method this way gets consumed (mimicking Rust semantics).
See [wasm-bindgen#2370](https://github.com/rustwasm/wasm-bindgen/issues/2370).
`Option<&T>` is not currently supported, but an equivalent behavior can be implemented manually.

```
extern crate alloc;
use js_sys::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_derive::{TryFromJsValue, try_from_js_option, into_js_option};

// Derive `TryFromJsValue` for the target structure (note that it has to come
// before the `[#wasm_bindgen]` attribute, and requires `Clone`):
#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Clone)]
struct MyType(usize);

// To have a correct typing annotation generated for TypeScript, declare a custom type.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MyType | undefined")]
    pub type OptionMyType;
}

// Use this type in the function signature.
#[wasm_bindgen]
pub fn option_example(value: &OptionMyType) -> Result<OptionMyType, Error> {
    // Use a helper to extract the typed value
    let typed_value = try_from_js_option::<MyType>(value).map_err(|err| Error::new(&err))?;

    // Now we have `typed_value: Option<MyType>`.

    // Return it
    // Note that if `typed_value` is `None`, `into_js_option()` creates a `JsValue::UNDEFINED`.
    Ok(into_js_option(typed_value))
}
```

## Vector arguments

With the closing of [wasm-bindgen#111](https://github.com/rustwasm/wasm-bindgen/issues/111),
it is now possible to return `Vec<MyType>` values.
Having an argument of type `Vec<MyType>` compiles, but results in an unexpected behavior
similar to that with `Option<MyType>`: all the elements of the input array
(but not the array itself) are invalidated on the JS side.
So this crate can still be used to take array arguments without invalidating them.

```
extern crate alloc;
use js_sys::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_derive::{TryFromJsValue, try_from_js_array, into_js_array};

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Clone)]
struct MyType(usize);

// To have a correct typing annotation generated for TypeScript, declare a custom type.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MyType[]")]
    pub type MyTypeArray;
}

// Use this type in the function signature.
#[wasm_bindgen]
pub fn vec_example(val: &MyTypeArray) -> Result<MyTypeArray, Error> {
    // Use a helper to extract the typed array
    let typed_array = try_from_js_array::<MyType>(val).map_err(|err| Error::new(&err))?;

    // Now we have `typed_array: Vec<MyType>`.

    // Return the array
    Ok(into_js_array(typed_array))
}

// Post wasm-bindgen 0.2.91, we can just return a vector
#[wasm_bindgen]
pub fn vec_example_simplified(val: &MyTypeArray) -> Result<Vec<MyType>, Error> {
    // Use a helper to extract the typed array
    let typed_array = try_from_js_array::<MyType>(val).map_err(|err| Error::new(&err))?;

    // Now we have `typed_array: Vec<MyType>`.

    // Return the array
    Ok(typed_array)
}
```
*/
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-derive")]
#![no_std]

// Ensure it is present. Needed for the generated code to work.
extern crate alloc;

mod helpers;

pub use helpers::{into_js_array, into_js_option, try_from_js_array, try_from_js_option};
pub use wasm_bindgen_derive_macro::TryFromJsValue;
