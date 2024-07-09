use labview_interop::memory::UHandle;
pub use labview_interop::types::{string::LStrHandle, LVBool};

/// This will check the provided handle is valid and also provide results from a null and made up handle
/// to confirm the validity check works.
#[no_mangle]
pub extern "C" fn handle_validity_checks(
    valid_handle: LStrHandle,
    null: *mut LVBool,
    valid: *mut LVBool,
    random_address: *mut LVBool,
) {
    unsafe {
        *valid = valid_handle.valid().into();
        let null_handle: UHandle<f64> = UHandle(std::ptr::null_mut());
        *null = null_handle.valid().into();
        // This crashes if the initial handle is garbage so create a valid pointer to an invalid pointer.
        let mut made_up_ptr = 0xDEAD as *mut f64;
        let madeup_handle: UHandle<f64> = UHandle(&mut made_up_ptr as *mut *mut f64);
        *random_address = madeup_handle.valid().into();
    }
}
