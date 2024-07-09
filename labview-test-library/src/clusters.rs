use std::ptr::{addr_of, read_unaligned};

use labview_interop::labview_layout;
use labview_interop::types::{LVArrayHandle, LVVariant, Waveform};

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
