//! The arrays module covers LabVIEW multidimensional array.
//!
//!

use crate::memory::UHandle;

/// Internal LabVIEW array representation.
///
/// todo: does this follow cluster packing rules?
#[repr(C)]
pub struct LVArray<const D: usize, T> {
    dim_sizes: [i32; D],
    data: *mut T,
}

impl<const D: usize, T> LVArray<D, T> {
    /// Get the total number of elements in the array across all dimensions.
    pub fn get_data_size(&self) -> usize {
        let size = self.dim_sizes.iter().fold(1, |size, dim| size * dim);
        size as usize
    }

    /// Get the data component as a slice.
    ///
    /// Note: for muti-dimension arrays this is a raw structure so you will
    /// need to understand the dimenisons and data ordering.
    ///
    /// For 1D arrays this can just be used as the data contents.
    pub fn data_as_slice(&self) -> &[T] {
        let size = self.get_data_size();
        // Safety: Dimensions are set by LabVIEW to be valid.
        unsafe { std::slice::from_raw_parts(self.data, size) }
    }

    /// Get the data component as a muteable slice.
    ///
    /// Note: for muti-dimension arrays this is a raw structure so you will
    /// need to understand the dimenisons and data ordering.
    ///
    /// For 1D arrays this can just be used as the data contents.
    pub fn data_as_slice_mut(&mut self) -> &mut [T] {
        let size = self.get_data_size();
        // Safety: Dimensions are set by LabVIEW to be valid.
        unsafe { std::slice::from_raw_parts_mut(self.data, size) }
    }
}

/// Definition of a handle to an array. Helper for FFI definitin.
pub type LVArrayHandle<const D: usize, T> = UHandle<LVArray<D, T>>;
