use labview_interop::labview_layout;
use labview_interop::types::{LVArrayHandle, LVTime, Waveform};

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
    array: LVArrayHandle<1, f64>,
    first: *mut f64,
    last: *mut f64,
) {
    unsafe {
        let array = array.as_ref();
        let array_data = array.data_as_slice();
        *first = array_data[0];
        *last = array_data[array_data.len() - 1];
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
        //let waveform = test.waveform.as_ref();
        let waveform_data = test.waveform.data.as_ref().data_as_slice();
        *one = test.one;
        *two = test.two;
        *three = test.three;
        *wv_first = waveform_data[0];
        *wv_last = waveform_data[waveform_data.len() - 1]
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
        let array1 = (*input).array1.as_ref();
        let array2 = (*input).array2.as_ref();

        let array1_data = array1.data_as_slice();
        let array2_data = array2.data_as_slice();

        *array1_first = array1_data[0];
        *array2_first = array2_data[0];
    }
}
