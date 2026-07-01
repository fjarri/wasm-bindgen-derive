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
use syn::{Data, DeriveInput, Error, parse_macro_input};

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

    // Find the first occurrence of `#[wasm_bindgen]` or `#[wasm_bindgen(.. = ..)]
    let wasm_bindgen_attr = input.attrs.iter().find(|attr| match &attr.meta {
        syn::Meta::Path(path) => path.is_ident("wasm_bindgen"),
        syn::Meta::List(list) => list.path.is_ident("wasm_bindgen"),
        syn::Meta::NameValue(_) => false,
    });

    let Some(wasm_bindgen_attr) = wasm_bindgen_attr else {
        return derive_error!(
            "TryFromJsValue can be defined only on struct exported to wasm with #[wasm_bindgen]"
        );
    };

    let maybe_js_class = if let syn::Meta::List(list) = &wasm_bindgen_attr.meta {
        let mut js_name = None;
        if let Err(err) = list.parse_nested_meta(|meta| {
            if meta.path.is_ident("js_name") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                js_name = Some(s.value());
            }
            Ok(())
        }) {
            return err.into_compile_error().into();
        }
        js_name
    } else {
        None
    };

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
                    // All numbers in JS are float64.
                    let ptr_u32: u32 = ptr.as_f64().ok_or(::wasm_bindgen::JsValue::NULL)
                        .map_err(|err| format!("{:?}", err))?
                        as u32;
                    let ptr_abi: ::wasm_bindgen::__rt::WasmPtr<::wasm_bindgen::__rt::WasmRefCell<#name>> =
                        ::wasm_bindgen::__rt::WasmPtr::from_usize(ptr_u32 as usize);
                    let instance_ref = unsafe { #name::ref_from_abi(ptr_abi) };
                    Ok(instance_ref.clone())
                } else {
                    Err(format!("Cannot convert {} to {}", object_classname, classname))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
