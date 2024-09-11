//! Memory manager functions for arrays.

use super::{LVArrayDims, LVArrayHandle, LVArrayOwned};
use crate::errors::Result;
use crate::memory::OwnedUHandle;

impl<const D: usize, T: NumericArrayResizable + Sized + Copy> LVArrayOwned<D, T> {
    /// Create a new empty owned array. Once created you can use
    /// the existing handle methods to manipulate the data.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use labview_interop::types::LVArrayOwned;
    ///
    /// let mut array = LVArrayOwned::<1, f64>::new_empty().unwrap();
    /// array.resize_array([10].into()).unwrap();
    /// ```
    ///
    pub fn new_empty() -> Result<Self> {
        unsafe {
            OwnedUHandle::new_unsized(|handle: &mut LVArrayHandle<D, T>| {
                handle.resize_array(LVArrayDims::new_empty())?;
                Ok(())
            })
        }
    }
}

pub trait NumericArrayResizable {
    /// The code used by the LabVIEW memory manager to represent the type.
    const TYPE_CODE: i32;
}

impl NumericArrayResizable for i8 {
    const TYPE_CODE: i32 = 0x01;
}

impl NumericArrayResizable for i16 {
    const TYPE_CODE: i32 = 0x02;
}

impl NumericArrayResizable for i32 {
    const TYPE_CODE: i32 = 0x03;
}

impl NumericArrayResizable for i64 {
    const TYPE_CODE: i32 = 0x04;
}

impl NumericArrayResizable for u8 {
    const TYPE_CODE: i32 = 0x05;
}

impl NumericArrayResizable for u16 {
    const TYPE_CODE: i32 = 0x06;
}

impl NumericArrayResizable for u32 {
    const TYPE_CODE: i32 = 0x07;
}

impl NumericArrayResizable for u64 {
    const TYPE_CODE: i32 = 0x08;
}

impl NumericArrayResizable for f32 {
    const TYPE_CODE: i32 = 0x09;
}

impl NumericArrayResizable for f64 {
    const TYPE_CODE: i32 = 0x0A;
}

impl<'array, const D: usize, T: NumericArrayResizable> LVArrayHandle<'array, D, T> {
    /// Resize the array to the new size.
    pub fn resize_array(&mut self, new_dims: LVArrayDims<D>) -> Result<()> {
        // Check if they match so resize isn't needed.
        // We can't perform this unaligned read on 32 bit so skip it.
        #[cfg(target_pointer_width = "64")]
        if new_dims == self.dim_sizes {
            return Ok(());
        }

        let new_size = new_dims.element_count();
        let mg_err = unsafe {
            crate::labview::memory_api()?.numeric_array_resize(
                T::TYPE_CODE,
                D as i32,
                self as *mut LVArrayHandle<D, T> as *mut usize as *mut crate::labview::UHandleValue,
                new_size,
            )
        };
        let result = mg_err.to_specific_result(());

        if result.is_ok() {
            self.dim_sizes = new_dims;
        }
        result
    }
}
