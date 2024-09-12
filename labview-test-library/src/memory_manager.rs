use labview_interop::types::LStrOwned;
pub use labview_interop::types::{string::LStrHandle, LVBool};
use labview_interop::{labview_layout, memory::UHandle};
use labview_interop::{memory::UPtr, sync::LVUserEvent, types::LVStatusCode};

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
        let null_handle: UHandle<f64> = UHandle(std::ptr::null_mut(), Default::default());
        *null = null_handle.valid().into();
        // This crashes if the initial handle is garbage so create a valid pointer to an invalid pointer.
        let mut made_up_ptr = 0xDEAD as *mut f64;
        let madeup_handle: UHandle<f64> =
            UHandle(&mut made_up_ptr as *mut *mut f64, Default::default());
        *random_address = madeup_handle.valid().into();
    }
}

labview_layout!(
    pub struct UserEventCluster<'a> {
        eventno: i32,
        id: LStrHandle<'a>,
    }
);

/// Confirm the expected workflow where we get a handle to an owned value.
#[no_mangle]
pub extern "C" fn generate_event_cluster_handle_from_owned(
    lv_user_event: UPtr<LVUserEvent<UserEventCluster>>,
) -> LVStatusCode {
    let mut mystr = LStrOwned::from_data(b"Hello World!").unwrap();
    let mystr_handle = mystr.handle_to_inner();
    let mut eventdata = UserEventCluster {
        eventno: 2,
        id: mystr_handle,
    };
    let result = lv_user_event.post(&mut eventdata);

    result.into()
}

labview_layout! {
    pub struct WrappedClusterWithString<'a> {
        pub string: LStrHandle<'a>,
    }
}

/// Copy the data from Rust to a LabVIEW cluster.
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn copy_cluster_data(output_cluster: *mut WrappedClusterWithString) -> LVStatusCode {
    let string = LStrOwned::from_data(b"Hello World!").unwrap();
    let result = unsafe {
        let inner_string = &mut (*output_cluster).string;
        string.clone_into_pointer(inner_string as *mut LStrHandle)
    };
    result.into()
}

/// Clone a handle and change with no change to the original.
///
/// We should be able to change one without the other changing.
#[no_mangle]
pub extern "C" fn handle_to_owned(input: LStrHandle, output: *mut LStrHandle) -> LVStatusCode {
    let result = unsafe {
        let mut owned_input = input.try_to_owned().unwrap();
        owned_input.set_str("Changed!").unwrap();
        owned_input.clone_into_pointer(output)
    };
    result.into()
}

/// Cloning a handle should not change the original.
#[no_mangle]
pub extern "C" fn clone_handle(out1: *mut LStrHandle, out2: *mut LStrHandle) {
    let original = LStrOwned::from_data(b"Original").unwrap();
    let mut changed = original.clone();
    changed.set_str("Changed").unwrap();
    unsafe {
        original.clone_into_pointer(out1).unwrap();
        changed.clone_into_pointer(out2).unwrap();
    };
}
