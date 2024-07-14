//! Test calls for array functions.
//!
//!

use labview_interop::{
    errors::MgErr,
    types::{LVArrayHandle, LVBool},
};
use labview_interop::errors::LVInteropError;
use labview_interop::types::array::LVArrayOwned;

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

#[no_mangle]
pub extern "C" fn empty_owned_array_initialisation(array_handle: *mut LVArrayHandle<1, f64>) -> MgErr {

    fn inner(array_handle: *mut LVArrayHandle<1, f64>) -> Result<(), LVInteropError> {
        let mut array = LVArrayOwned::<1, f64>::new_empty()?;
        array.resize_array([5].into())?;
        for index in 0..5 {
            // keeps it compatible with 32 bit.
            unsafe {
                array.set_value_unchecked(index, index as f64);
            }
        }
        unsafe {
            array.clone_into_pointer(array_handle)?;
        }

        Ok(())

    }
    inner(array_handle).into()
}
