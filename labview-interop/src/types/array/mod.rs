//! The arrays module covers LabVIEW multidimensional array.
//!
//! todo: empty array can be an null handle. Detect and use.

#[cfg(feature = "ndarray")]
mod ndarray;
#[cfg(feature = "link")]
mod memory;

use crate::labview_layout;
use crate::memory::UHandle;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LvArrayDims<const D: usize>([i32; D]);

impl<const D: usize> LvArrayDims<D> {
    pub fn element_count(&self) -> usize {
        self.0.iter().fold(1, |size, dim| size * *dim as usize)
    }
}

impl<const D: usize> From<[i32; D]> for LvArrayDims<D> {
    fn from(dim_sizes: [i32; D]) -> Self {
        Self(dim_sizes)
    }
}

labview_layout!(
    /// Internal LabVIEW array representation.
    ///
    /// todo: does this follow cluster packing rules? yes but lots breaks.
    pub struct LVArray<const D: usize, T> {
        dim_sizes: LvArrayDims<D>,
        // For 64 bit use the DST syntax which is more correct to what we
        // are representing.
        #[cfg(target_pointer_width = "64")]
        data: [T],
        // DST not supported in packing used for 32 bit.
        #[cfg(target_pointer_width = "32")]
        data: T,
    }
);

///implement a basic, unsafe API that works for packed usage on 32 bit targets.
///
/// It is copy only as we must copy out of the pointers.
impl<const D: usize, T> LVArray<D, T> {
    /// Get the data size. Works with the packed structures found in the 32 bit interface.
    pub fn get_data_size(&self) -> usize {
        let mut size: usize = 1;

        for index in 0..D {
            let element_ptr = std::ptr::addr_of!(self.dim_sizes.0[index]);
            // Safety: the indexes must be in range due to the const generic value.
            let dim_size = unsafe { std::ptr::read_unaligned(element_ptr) };
            size *= dim_size as usize;
        }

        size
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

        //self.data[index]
    }
}

#[cfg(target_pointer_width = "64")]
impl<const D: usize, T> LVArray<D, T> {
    /// Get the total number of elements in the array across all dimensions.
    pub fn element_count(&self) -> usize {
       self.dim_sizes.element_count()
    }

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

    /// Get the data component as a muteable slice.
    ///
    /// Note: for muti-dimension arrays this is a raw structure so you will
    /// need to understand the dimenisons and data ordering.
    ///
    /// For 1D arrays this can just be used as the data contents.
    pub fn data_as_slice_mut(&mut self) -> &mut [T] {
        let size = self.element_count();
        // Safety: Dimensions are set by LabVIEW to be valid.
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr(), size) }
    }
}

/// Definition of a handle to an array. Helper for FFI definitin.
pub type LVArrayHandle<const D: usize, T> = UHandle<LVArray<D, T>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dimension_element_count() {
        let dims = LvArrayDims::<3>([2, 3, 4]);
        assert_eq!(dims.element_count(), 24);

        let dims = LvArrayDims::<2>([2, 3]);
        assert_eq!(dims.element_count(), 6);

        let dims = LvArrayDims::<1>([2]);
        assert_eq!(dims.element_count(), 2);
    }

    #[test]
    fn test_dim_equality() {
        let dims1 = LvArrayDims::<3>([2, 3, 4]);
        let dims2 = LvArrayDims::<3>([2, 3, 4]);
        assert_eq!(dims1, dims2);
    }
}