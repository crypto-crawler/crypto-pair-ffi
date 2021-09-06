#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// Normalize a cryptocurrency trading pair.
#[no_mangle]
pub extern "C" fn normalize_pair(symbol: *const c_char, exchange: *const c_char) -> *const c_char {
    let symbol = unsafe {
        debug_assert!(!symbol.is_null());
        CStr::from_ptr(symbol).to_str().unwrap()
    };

    let exchange = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Some(pair) = crypto_pair::normalize_pair(symbol, exchange) {
            let raw = CString::new(pair).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{:?}", err);
            std::ptr::null()
        }
    }
}
