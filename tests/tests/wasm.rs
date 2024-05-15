use wasm_bindgen_test::wasm_bindgen_test;

use wasm_bindgen_derive::{into_js_array, into_js_option, try_from_js_array, try_from_js_option};
use wasm_bindgen_derive_tests::{option_example, vec_example, MyType};

#[wasm_bindgen_test]
fn test_option_example_some() {
    let arg = Some(MyType::new(1));
    let arg_js = into_js_option(arg.clone());

    let result_js = option_example(&arg_js).unwrap();
    let result = try_from_js_option::<MyType>(result_js);
    assert!(result == Ok(arg));
}

#[wasm_bindgen_test]
fn test_option_example_none() {
    let arg = None;
    let arg_js = into_js_option(arg.clone());

    let result_js = option_example(&arg_js).unwrap();
    let result = try_from_js_option::<MyType>(result_js);
    assert!(result == Ok(arg));
}

#[wasm_bindgen_test]
fn test_vec_example() {
    let arg = vec![MyType::new(0), MyType::new(1), MyType::new(2)];
    let arg_js = into_js_array(arg.clone());

    let result_js = vec_example(&arg_js).unwrap();
    let result = try_from_js_array::<MyType>(result_js);
    assert!(result == Ok(arg));
}
