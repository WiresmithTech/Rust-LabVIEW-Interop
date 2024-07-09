#![allow(clippy::not_unsafe_ptr_arg_deref)]

use labview_interop::errors::MgErr;
use labview_interop::labview_layout;
use labview_interop::memory::{UHandle, UPtr};
use labview_interop::sync::{LVUserEvent, Occurence};
use labview_interop::types::string::{LStrHandle, LStrOwned};
#[cfg(target_pointer_width = "64")]
use labview_interop::types::{ErrorClusterPtr, ToLvError};
use labview_interop::types::{LVArrayHandle, LVBool, LVTime, LVVariant, Waveform};

use std::ffi::{c_char, CStr};
use std::ptr::{addr_of, read_unaligned};

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

#[no_mangle]
pub extern "C" fn timestamp_to_epoch(timestamp: *const LVTime) -> f64 {
    unsafe { (*timestamp).to_lv_epoch() }
}

#[no_mangle]
pub extern "C" fn timestamp_from_epoch(seconds_since_epoch: f64, timestamp: *mut LVTime) {
    let timestamp = unsafe { timestamp.as_mut().unwrap() };
    *timestamp = LVTime::from_lv_epoch(seconds_since_epoch);
}

#[no_mangle]
pub extern "C" fn timestamp_from_le_bytes(bytes: *const u8, timestamp: *mut LVTime) {
    // Safety: for this simple test we can assume we have the right size bytes.
    let mut buf = [0u8; 16];
    let byte_slice = unsafe { std::slice::from_raw_parts(bytes, 16) };
    buf.copy_from_slice(byte_slice);
    unsafe {
        *timestamp = LVTime::from_le_bytes(buf);
    }
}

#[no_mangle]
pub extern "C" fn timestamp_from_be_bytes(bytes: *const u8, timestamp: *mut LVTime) {
    // Safety: for this simple test we can assume we have the right size bytes.
    let mut buf = [0u8; 16];
    let byte_slice = unsafe { std::slice::from_raw_parts(bytes, 16) };
    buf.copy_from_slice(byte_slice);
    unsafe {
        *timestamp = LVTime::from_be_bytes(buf);
    }
}

#[no_mangle]
pub extern "C" fn extract_from_array(
    array_handle: LVArrayHandle<1, f64>,
    first: *mut f64,
    last: *mut f64,
) {
    unsafe {
        let array_data = array_handle.as_ref().unwrap();
        let element_count = array_data.element_count();
        *first = array_data.get_value_unchecked(0);
        *last = array_data.get_value_unchecked(element_count - 1);
    }
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn extract_from_array_ndarray(
    array_handle: LVArrayHandle<2, f64>,
    end_of_first_row: *mut f64,
    start_of_last_row: *mut f64,
) {
    let array = array_handle.ndarray_view();
    let rows = array.nrows();
    let columns = array.ncols();
    unsafe {
        *end_of_first_row = *array.get([0, columns - 1]).unwrap();
        *start_of_last_row = *array.get([rows - 1, 0]).unwrap();
    }
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn copy_from_ndarray(mut array_handle: LVArrayHandle<2, i32>) -> MgErr {
    let ndarray = ndarray::arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let result = array_handle.copy_from_ndarray(&ndarray);
    result.into()
}

#[no_mangle]
pub extern "C" fn resize_array(mut array_handle: LVArrayHandle<2, f64>) -> MgErr {
    let result = array_handle.resize_array([3, 3].into());

    match result {
        Ok(()) => {
            for index in 0..9 {
                unsafe {
                    array_handle.set_value_unchecked(index, index as f64);
                }
            }
            MgErr::NO_ERROR
        }
        Err(e) => e.into(),
    }
}

#[no_mangle]
pub extern "C" fn is_array_empty(array_handle: LVArrayHandle<1, f64>, empty: *mut LVBool) -> MgErr {
    let size = array_handle.element_count();
    unsafe { *empty = (size == 0).into() }
    MgErr::NO_ERROR
}

labview_layout!(
    pub struct TestStruct {
        one: u8,
        two: u16,
        waveform: Waveform<f64>,
        three: u32,
    }
);

///Similar to above we have a seperate 32 bit and 64 bit version
/// to test the different access methods available.
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn extract_test_struct_with_waveform(
    test_struct: *const TestStruct,
    one: *mut u8,
    two: *mut u16,
    three: *mut u32,
    wv_first: *mut f64,
    wv_last: *mut f64,
) {
    let _result = std::panic::catch_unwind(|| unsafe {
        let test = test_struct.as_ref().unwrap();
        let waveform_data = test.waveform.data.as_ref().unwrap().data_as_slice();
        *one = test.one;
        *two = test.two;
        *three = test.three;
        *wv_first = waveform_data[0];
        *wv_last = waveform_data[waveform_data.len() - 1]
    });
}

///Similar to above we have a seperate 32 bit and 64 bit version
/// to test the different access methods available.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn extract_test_struct_with_waveform(
    test_struct: *const TestStruct,
    one: *mut u8,
    two: *mut u16,
    three: *mut u32,
    wv_first: *mut f64,
    wv_last: *mut f64,
) {
    let _result = std::panic::catch_unwind(|| unsafe {
        let waveform_ptr = std::ptr::addr_of!((*test_struct).waveform.data);
        let waveform_data = std::ptr::read_unaligned(waveform_ptr);
        let waveform_data = waveform_data.as_ref().unwrap();
        *one = (*test_struct).one;
        *two = (*test_struct).two;
        *three = (*test_struct).three;
        *wv_first = waveform_data.get_value_unchecked(0);
        *wv_last = waveform_data.get_value_unchecked(waveform_data.element_count() - 1);
    });
}

labview_layout!(
    pub struct ClusterHandles {
        array1: LVArrayHandle<1, u8>,
        array2: LVArrayHandle<2, u32>,
    }
);

#[no_mangle]
pub extern "C" fn extract_cluster_handles(
    input: *const ClusterHandles,
    array1_first: *mut u8,
    array2_first: *mut u32,
) {
    unsafe {
        let array1_ptr = addr_of!((*input).array1);
        let array2_ptr = addr_of!((*input).array2);
        let array1 = read_unaligned(array1_ptr);
        let array2 = read_unaligned(array2_ptr);

        let array1_data = array1.as_ref().unwrap();
        let array2_data = array2.as_ref().unwrap();

        *array1_first = array1_data.get_value_unchecked(0);
        *array2_first = array2_data.get_value_unchecked(0);
    }
}

labview_layout!(
    pub struct ClusterVariant {
        one: u64,
        variant: LVVariant,
        two: u32,
    }
);

#[no_mangle]
/// This is designed to see if the strange padding we've seen in waveforms
/// is specific to variants and needs to be handled there.
pub extern "C" fn extract_cluster_variant(
    input: *const ClusterVariant,
    one: *mut u64,
    two: *mut u32,
) {
    unsafe {
        *one = (*input).one;
        *two = (*input).two;
    }
}

#[no_mangle]
pub extern "C" fn generate_event_3(lv_user_event: *mut LVUserEvent<i32>) -> MgErr {
    let event = unsafe { *lv_user_event };
    let result = event.post(&mut 3);
    result.into()
}

labview_layout!(
    pub struct UserEventCluster {
        eventno: i32,
        id: LStrOwned,
    }
);

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn update_cluster(cluster: UPtr<UserEventCluster>) -> MgErr {
    let clust = unsafe { cluster.as_ref_mut().unwrap() };
    clust.eventno = 5;
    clust.id.set("This is the new string".as_bytes()).unwrap();

    MgErr::NO_ERROR
}

#[no_mangle]
pub extern "C" fn generate_event_cluster(
    lv_user_event: UPtr<LVUserEvent<UserEventCluster>>,
) -> MgErr {
    let mystr = LStrOwned::from_data(b"Hello World!").unwrap();
    let mut eventdata = UserEventCluster {
        eventno: 2,
        id: mystr,
    };
    let result = lv_user_event.post(&mut eventdata);

    result.into()
}

#[no_mangle]
pub extern "C" fn generate_occurence(occurence: *mut Occurence) -> MgErr {
    let result = unsafe { (*occurence).set() };
    result.into()
}

#[no_mangle]
pub extern "C" fn hello_world(mut string: LStrHandle) -> MgErr {
    let result = string.set_str("Hello World");
    result.into()
}

#[no_mangle]
pub extern "C" fn hello_world_owned(string: *mut LStrHandle) -> MgErr {
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

/// A simple type for testing the error integration.
struct ErrorText(&'static str);

#[cfg(target_pointer_width = "64")]
impl ToLvError for ErrorText {
    fn source(&self) -> std::borrow::Cow<'_, str> {
        "Rust".into()
    }

    fn description(&self) -> std::borrow::Cow<'_, str> {
        self.0.into()
    }
}

#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub extern "C" fn set_error_cluster(error_cluster: ErrorClusterPtr) -> MgErr {
    let error = ErrorText("This is a test");
    error.write_error(error_cluster).into()
}

pub fn test() {
    labview_layout!(
        pub struct TestStruct {
            one: u8,
            two: u16,
            three: u32,
        }
    );

    let value = TestStruct {
        one: 1,
        two: 2,
        three: 3,
    };

    // Not allowed.
    //let three_ref = &value.three;

    unsafe {
        let three_ptr: *const u32 = addr_of!(value.three);
        let _three: u32 = read_unaligned(three_ptr);
    }
}
