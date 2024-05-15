use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use wasm_bindgen::{JsCast, JsValue};

/// Converts the given optional typed value into a JS value with the custom type `T`.
///
/// The type `T` would be defined in an `extern "C"` block, e.g.
/// ```
/// use wasm_bindgen::prelude::wasm_bindgen;
/// #[wasm_bindgen]
/// extern "C" {
/// #[wasm_bindgen(typescript_type = "MyType | undefined")]
///     pub type OptionMyType; // <-- this would be `T`
/// }
/// ```
///
/// If the typed value is `None`, the result is JS `underfined`.
pub fn into_js_option<T, U>(val: Option<U>) -> T
where
    JsValue: From<U>,
    T: JsCast,
{
    let js_val = match val {
        None => JsValue::UNDEFINED,
        Some(val) => val.into(),
    };
    js_val.unchecked_into::<T>()
}

/// Attempts to unpack a JS value into a typed value,
/// returning `None` if the JS value is `undefined`.
pub fn try_from_js_option<T>(val: impl Into<JsValue>) -> Result<Option<T>, String>
where
    for<'a> T: TryFrom<&'a JsValue>,
    for<'a> <T as TryFrom<&'a JsValue>>::Error: core::fmt::Display,
{
    let js_val = val.into();
    if js_val.is_undefined() {
        return Ok(None);
    }
    T::try_from(&js_val)
        .map(Some)
        .map_err(|err| format!("{}", err))
}

/// Converts the given iterator into a JS array of the custom type `T`.
///
/// The type `T` would be defined in an `extern "C"` block, e.g.
/// ```
/// use wasm_bindgen::prelude::wasm_bindgen;
/// #[wasm_bindgen]
/// extern "C" {
///     #[wasm_bindgen(typescript_type = "MyType[]")]
///     pub type MyTypeArray; // <-- this would be `T`
/// }
/// ```
pub fn into_js_array<T, U>(value: impl IntoIterator<Item = U>) -> T
where
    JsValue: From<U>,
    T: JsCast,
{
    value
        .into_iter()
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<T>()
}

/// Attempts to unpack a JS array into a vector of typed values.
pub fn try_from_js_array<T>(val: impl Into<JsValue>) -> Result<Vec<T>, String>
where
    for<'a> T: TryFrom<&'a JsValue>,
    for<'a> <T as TryFrom<&'a JsValue>>::Error: core::fmt::Display,
{
    let js_val = val.into();
    let array: &js_sys::Array = js_val.dyn_ref().ok_or("The argument must be an array")?;
    let length: usize = array
        .length()
        .try_into()
        .map_err(|err| format!("{}", err))?;
    let mut typed_array = Vec::<T>::with_capacity(length);
    for (idx, js) in array.iter().enumerate() {
        let typed_elem =
            T::try_from(&js).map_err(|err| format!("Failed to cast item {}: {}", idx, err))?;
        typed_array.push(typed_elem);
    }
    Ok(typed_array)
}
