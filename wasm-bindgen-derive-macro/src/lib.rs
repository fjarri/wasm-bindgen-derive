//! A proc-macro to be re-exported by `wasm-bindgen-derive`.
//! We need this trampoline to enforce the correct bounds on the `wasm-bindgen` and `js-sys`
//! dependencies, but those are technically not the dependencies of this crate,
//! but only of the code it generates.

#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::ToString;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

/** Derives a `TryFrom<&JsValue>` for a type exported using `#[wasm_bindgen]`.

Note that:
* this derivation must be be positioned before `#[wasm_bindgen]`;
* the type must implement [`Clone`].
* `extern crate alloc` must be declared in scope.

The macro is authored by [**@AlexKorn**](https://github.com/AlexKorn)
based on the idea of [**@aweinstock314**](https://github.com/aweinstock314).
See [this](https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-656293288)
and [this](https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-1169658111)
GitHub comments.
*/
#[proc_macro_derive(TryFromJsValue)]
pub fn derive_try_from_jsvalue(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    match data {
        Data::Struct(_) => {}
        _ => return derive_error!("TryFromJsValue may only be derived on structs"),
    };

    let wasm_bindgen_meta = input.attrs.iter().find_map(|attr| {
        attr.parse_meta()
            .ok()
            .and_then(|meta| match meta.path().is_ident("wasm_bindgen") {
                true => Some(meta),
                false => None,
            })
    });
    if wasm_bindgen_meta.is_none() {
        return derive_error!(
            "TryFromJsValue can be defined only on struct exported to wasm with #[wasm_bindgen]"
        );
    }

    let maybe_js_class = wasm_bindgen_meta
        .and_then(|meta| match meta {
            syn::Meta::List(list) => Some(list),
            _ => None,
        })
        .and_then(|meta_list| {
            meta_list.nested.iter().find_map(|nested_meta| {
                let maybe_meta = match nested_meta {
                    syn::NestedMeta::Meta(meta) => Some(meta),
                    _ => None,
                };

                maybe_meta
                    .and_then(|meta| match meta {
                        syn::Meta::NameValue(name_value) => Some(name_value),
                        _ => None,
                    })
                    .and_then(|name_value| match name_value.path.is_ident("js_name") {
                        true => Some(name_value.lit.clone()),
                        false => None,
                    })
                    .and_then(|lit| match lit {
                        syn::Lit::Str(str) => Some(str.value()),
                        _ => None,
                    })
            })
        });

    let wasm_bindgen_macro_invocaton = match maybe_js_class {
        Some(class) => format!(
            "::wasm_bindgen::prelude::wasm_bindgen(js_class = \"{}\")",
            class
        ),
        None => "::wasm_bindgen::prelude::wasm_bindgen".to_string(),
    }
    .parse::<TokenStream2>()
    .unwrap();

    let expanded = quote! {
        impl #name {
            pub fn __get_classname() -> &'static str {
                ::core::stringify!(#name)
            }
        }

        #[#wasm_bindgen_macro_invocaton]
        impl #name {
            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "__getClassname")]
            pub fn __js_get_classname(&self) -> String {
                use ::alloc::borrow::ToOwned;
                ::core::stringify!(#name).to_owned()
            }
        }

        impl ::core::convert::TryFrom<&::wasm_bindgen::JsValue> for #name {
            type Error = String;

            fn try_from(js: &::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
                use ::alloc::borrow::ToOwned;
                use ::alloc::string::ToString;
                use ::wasm_bindgen::JsCast;
                use ::wasm_bindgen::convert::RefFromWasmAbi;

                let classname = Self::__get_classname();

                if !js.is_object() {
                    return Err(format!("Value supplied as {} is not an object", classname));
                }

                let no_get_classname_msg = concat!(
                    "no __getClassname method specified for object; ",
                    "did you forget to derive TryFromJsObject for this type?");

                let get_classname = ::js_sys::Reflect::get(
                    js,
                    &::wasm_bindgen::JsValue::from("__getClassname"),
                )
                .or(Err(no_get_classname_msg.to_string()))?;

                if get_classname.is_undefined() {
                    return Err(no_get_classname_msg.to_string());
                }

                let get_classname = get_classname
                    .dyn_into::<::js_sys::Function>()
                    .map_err(|err| format!("__getClassname is not a function, {:?}", err))?;

                let object_classname: String = ::js_sys::Reflect::apply(
                        &get_classname,
                        js,
                        &::js_sys::Array::new(),
                    )
                    .ok()
                    .and_then(|v| v.as_string())
                    .ok_or_else(|| "Failed to get classname".to_owned())?;

                if object_classname.as_str() == classname {
                    // Note: using an undocumented implementation detail of `wasm-bindgen`:
                    // the pointer property has the name `__wbg_ptr` (since wasm-bindgen 0.2.85)
                    let ptr = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("__wbg_ptr"))
                        .map_err(|err| format!("{:?}", err))?;
                    let ptr_u32: u32 = ptr.as_f64().ok_or(::wasm_bindgen::JsValue::NULL)
                        .map_err(|err| format!("{:?}", err))?
                        as u32;
                    let instance_ref = unsafe { #name::ref_from_abi(ptr_u32) };
                    Ok(instance_ref.clone())
                } else {
                    Err(format!("Cannot convert {} to {}", object_classname, classname))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
