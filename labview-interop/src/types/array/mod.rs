//! The arrays module covers LabVIEW multidimensional array.
//!

mod dimensions;
#[cfg(feature = "link")]
mod memory;
#[cfg(all(feature = "ndarray", target_pointer_width = "64"))]
mod ndarray;

use crate::labview_layout;
#[cfg(feature = "link")]
pub use crate::memory::{OwnedUHandle };
#[cfg(feature = "link")]
pub use memory::NumericArrayResizable;
use crate::memory::{LVCopy, UHandle};
pub use dimensions::LVArrayDims;

labview_layout!(
    /// Internal LabVIEW array representation.
    ///
    /// todo: does this follow cluster packing rules? yes but lots breaks.
    pub struct LVArray<const D: usize, T> {
        dim_sizes: LVArrayDims<D>,
        // For 64 bit use the DST syntax which is more correct to what we
        // are representing.
        #[cfg(target_pointer_width = "64")]
        data: [T],
        // DST not supported in packing used for 32 bit.
        #[cfg(target_pointer_width = "32")]
        data: T,
    }
);

impl<const D: usize, T> LVCopy for LVArray<D, T> {}

///implement a basic, unsafe API that works for packed usage on 32 bit targets.
///
/// It is copy only as we must copy out of the pointers.
impl<const D: usize, T> LVArray<D, T> {
    /// Get the dimensions of the array.
    #[cfg(target_pointer_width = "32")]
    pub fn dimension_sizes(&self) -> LVArrayDims<D> {
        // This 32 bit version must make potentially unaligned accesses in the structure
        // so this is a little more convoluted.
        // Because these lead the struct they should infact always be aligned.
        let mut dimensions = [0i32; D];

        for (index, value) in dimensions.iter_mut().enumerate() {
            let element_ptr = std::ptr::addr_of!(self.dim_sizes.0[index]);
            // Safety: the indexes must be in range due to the const generic value.
            let dim_size = unsafe { std::ptr::read_unaligned(element_ptr) };
            *value = dim_size;
        }

        dimensions.into()
    }

    /// Get the dimensions of the array.
    #[cfg(target_pointer_width = "64")]
    pub fn dimension_sizes(&self) -> LVArrayDims<D> {
        self.dim_sizes
    }

    /// Get the total number of elements in the array across all dimensions.
    pub fn element_count(&self) -> usize {
        self.dimension_sizes().element_count()
    }

    /// Get the value directly from the array. This is an unsafe method used on
    /// 32 bit targets where the packed structure means we cannot access a slice.
    ///
    /// On 64 bit targets use [`LVArray::data_as_slice`] instead.
    ///
    /// # Safety
    ///
    /// If the index is out of the range then it is undefined behaviour.
    pub unsafe fn get_value_unchecked(&self, index: usize) -> T {
        let data_ptr = std::ptr::addr_of!(self.data) as *const T;
        let element_ptr = data_ptr.add(index);
        std::ptr::read_unaligned(element_ptr)

        //self.data[index]`
    }

    /// Set the value at the index. This is an unsafe method used on 32 bit targets
    /// where the packed structure means we cannot access a slice.
    ///
    /// On 64 bit targets use [`LVArray::data_as_slice_mut`] instead.
    ///
    /// # Safety
    ///
    /// If the index is out of range then it is undefined behaviour.
    pub unsafe fn set_value_unchecked(&mut self, index: usize, value: T) {
        let data_ptr = std::ptr::addr_of_mut!(self.data) as *mut T;
        let element_ptr = data_ptr.add(index);
        std::ptr::write_unaligned(element_ptr, value);
    }
}

#[cfg(target_pointer_width = "64")]
impl<const D: usize, T> LVArray<D, T> {
    /// Get the data component as a slice.
    ///
    /// Note: for muti-dimension arrays this is a raw structure so you will
    /// need to understand the dimenisons and data ordering.
    ///
    /// For 1D arrays this can just be used as the data contents.
    pub fn data_as_slice(&self) -> &[T] {
        let size = self.element_count();
        // Safety: Dimensions are set by LabVIEW to be valid.
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), size) }
    }

    /// Get the data component as a mutable slice.
    ///
    /// Note: for multi-dimension arrays this is a raw structure so you will
    /// need to understand the dimensions and data ordering.
    ///
    /// For 1D arrays this can just be used as the data contents.
    pub fn data_as_slice_mut(&mut self) -> &mut [T] {
        let size = self.element_count();
        // Safety: Dimensions are set by LabVIEW to be valid.
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr(), size) }
    }
}

/// Definition of a handle to an array. Helper for FFI definition.
pub type LVArrayHandle<'a, const D: usize, T> = UHandle<'a, LVArray<D, T>>;

/// Definition of an owned handle to an array. Helper for FFI definition.
#[cfg(feature = "link")]
pub type LVArrayOwned<const D: usize, T> = OwnedUHandle<LVArray<D, T>>;
