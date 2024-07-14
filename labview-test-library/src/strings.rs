//! Test functions for string interfaces.
//!
use std::ffi::{c_char, CStr};

use labview_interop::{
    errors::MgErr,
    types::string::{LStrHandle, LStrOwned},
};
#[no_mangle]
pub extern "C" fn hello_world(mut string: LStrHandle) -> MgErr {
    let result = string.set_str("Hello World");
    result.into()
}

#[no_mangle]
pub extern "C" fn hello_world_owned(string: *mut LStrHandle<'static>) -> MgErr {
    let result = LStrOwned::from_data("Hello World".as_bytes());

    match result {
        Ok(strok) => {
            let clone_result = unsafe { strok.clone_into_pointer(string) };
            clone_result.into()
        }
        Err(err) => err.into(),
    }
}

#[no_mangle]
pub extern "C" fn count_words(string: LStrHandle, count: &mut i32) -> MgErr {
    let rust_string = string.to_rust_string();
    *count = rust_string.split_ascii_whitespace().count() as i32;
    MgErr::NO_ERROR
}

#[no_mangle]
pub extern "C" fn count_words_lossy(string: LStrHandle, count: &mut i32) -> MgErr {
    let rust_string = String::from_utf8_lossy(string.as_slice());
    *count = rust_string.split_ascii_whitespace().count() as i32;
    MgErr::NO_ERROR
}

#[no_mangle]
pub extern "C" fn count_words_c_string(string: *const c_char, count: &mut i32) -> MgErr {
    let rust_string = unsafe { CStr::from_ptr(string).to_string_lossy() };
    *count = rust_string.split_ascii_whitespace().count() as i32;
    MgErr::NO_ERROR
}
