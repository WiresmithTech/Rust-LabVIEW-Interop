use labview_interop::errors::MgErr;
use labview_interop::labview_layout;
use labview_interop::sync::{LVUserEvent, Occurence};
use labview_interop::types::string::LStrHandle;
use labview_interop::types::{LVArrayHandle, LVTime, LVVariant, Waveform};

use std::ptr::{addr_of, read_unaligned};

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
        let element_count = array_data.get_data_size();
        *first = array_data.get_value_unchecked(0);
        *last = array_data.get_value_unchecked(element_count - 1);
    }
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
        *wv_last = waveform_data.get_value_unchecked(waveform_data.get_data_size() - 1);
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
pub extern "C" fn count_words(string: LStrHandle, count: &mut i32) -> MgErr {
    let rust_string = string.to_rust_string();
    *count = rust_string.split_ascii_whitespace().count() as i32;
    MgErr::NO_ERROR
}

pub fn test() {
    use std::ptr::{addr_of, read_unaligned};
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
    let three_ref = &value.three;

    unsafe {
        let three_ptr: *const u32 = addr_of!(value.three);
        let three: u32 = read_unaligned(three_ptr);
    }
}
