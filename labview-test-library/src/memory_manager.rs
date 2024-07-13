
use labview_interop::types::LStrOwned;
pub use labview_interop::types::{string::LStrHandle, LVBool};
use labview_interop::{errors::MgErr, memory::UPtr, sync::LVUserEvent};
use labview_interop::{labview_layout, memory::UHandle};

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
) -> MgErr {
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
#[no_mangle]
pub extern "C" fn copy_cluster_data(
    output_cluster: *mut WrappedClusterWithString,
) -> MgErr {
    let string = LStrOwned::from_data(b"Hello World!").unwrap();
    let result = unsafe {
        let inner_string = &mut (*output_cluster).string;
        string.clone_into_pointer(inner_string as *mut LStrHandle)
    };
    result.into()
}