use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

use wasm_bindgen_derive_tests::{option_example, vec_example, MyType};

fn into_js_option<T, U>(val: Option<U>) -> T
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

fn into_js_array<T, U>(val: impl IntoIterator<Item = U>) -> T
where
    JsValue: From<U>,
    T: JsCast,
{
    val.into_iter()
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<T>()
}

fn try_from_js_option<T>(val: impl Into<JsValue>) -> Option<T>
where
    for<'a> T: TryFrom<&'a JsValue>,
    for<'a> <T as TryFrom<&'a JsValue>>::Error: core::fmt::Debug,
{
    let js_val = val.into();
    if js_val.is_undefined() {
        return None;
    }
    Some(T::try_from(&js_val).unwrap())
}

fn try_from_js_array<T>(val: impl Into<JsValue>) -> Vec<T>
where
    for<'a> T: TryFrom<&'a JsValue>,
    for<'a> <T as TryFrom<&'a JsValue>>::Error: core::fmt::Debug,
{
    let js_array: js_sys::Array = val.into().dyn_into().unwrap();
    js_array
        .iter()
        .map(|js| T::try_from(&js).unwrap())
        .collect::<Vec<_>>()
}

#[wasm_bindgen_test]
fn test_option_example_some() {
    let arg = Some(MyType::new(1));
    let arg_js = into_js_option(arg.clone());

    let result_js = option_example(&arg_js).unwrap();
    let result = try_from_js_option::<MyType>(result_js);
    assert!(result == arg);
}

#[wasm_bindgen_test]
fn test_option_example_none() {
    let arg = None;
    let arg_js = into_js_option(arg.clone());

    let result_js = option_example(&arg_js).unwrap();
    let result = try_from_js_option::<MyType>(result_js);
    assert!(result == arg);
}

#[wasm_bindgen_test]
fn test_vec_example() {
    let arg = vec![MyType::new(0), MyType::new(1), MyType::new(2)];
    let arg_js = into_js_array(arg.clone());

    let result_js = vec_example(&arg_js).unwrap();
    let result = try_from_js_array::<MyType>(result_js);
    assert!(result == arg);
}
