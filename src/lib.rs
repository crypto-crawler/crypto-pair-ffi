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

/// Deallocate a string.
#[no_mangle]
pub extern "C" fn deallocate_string(pointer: *const c_char) {
    unsafe {
        if pointer.is_null() {
            return;
        }
        CString::from_raw(pointer as *mut c_char)
    };
}

#[cfg(test)]
mod tests {
    use super::{deallocate_string, normalize_pair};
    use std::ffi::{CStr, CString};

    #[test]
    fn test_normalize_pair() {
        let (string_ptr, string_str) = {
            let symbol = CString::new("BTCUSDT").unwrap();
            let exchange = CString::new("binance").unwrap();

            let string_ptr = normalize_pair(symbol.as_ptr(), exchange.as_ptr());
            let string_c_str = unsafe {
                debug_assert!(!string_ptr.is_null());
                CStr::from_ptr(string_ptr)
            };

            (string_ptr, string_c_str.to_str().unwrap())
        };

        assert_eq!(string_str, "BTC/USDT");

        deallocate_string(string_ptr);
    }
}
