//! Handle the various string times that the LabVIEW
//! interface provides.
//!

use crate::errors::{LVInteropError, Result};
use crate::labview_layout;
use crate::memory::{UHandle, UPtr};

labview_layout!(
    /// Internal LabVIEW string
    pub struct LStr {
        size: i32,
        data: [u8],
    }
);

/// Definition of a handle to an LabVIEW String. Helper for FFI definition and
/// required for any functions that need to resize the string.
pub type LStrHandle = UHandle<LStr>;
/// Definition of a pointer to an LabVIEW String. Helper for FFI definition.
pub type LStrPtr = UPtr<LStr>;

impl LStr {
    /// Access the data from the string as a binary slice.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.size as usize) }
    }

    /// Access the data from the string as a mutable slice.
    ///
    /// Use this function for modifying the data without changing the size.
    ///
    /// If you need to change the size you must access the handle that contains
    /// the data and access [`LStrHandle::set`]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.size as usize) }
    }
}

#[cfg(feature = "link")]
impl LStrHandle {
    /// Set the string as a binary value against the handle.
    ///
    /// This function will resize the handle based on the size of the input value.
    ///
    /// # Errors
    ///
    /// * This will error if the string handle is invalid (likely a null pointer).
    ///
    /// # Example
    /// ```
    /// #[no_mangle]
    /// pub extern "C" fn hello_world(mut string: LStrHandle) -> MgErr {
    ///    let result = string.set(b"Hello World");
    ///    result.into()
    /// }
    //```
    pub fn set(&mut self, value: &[u8]) -> Result<()> {
        let input_length = value.len();

        unsafe {
            //Safety: Is this alignment ever wrong. Would it even pad between the size and data.
            // I believe not.
            let struct_size = input_length + 4;
            self.resize(struct_size)?;

            let l_str = self.as_mut().ok_or(LVInteropError::InvalidHandle)?;
            l_str.size = input_length as i32;
            for (value, output) in value.iter().zip(l_str.data.iter_mut()) {
                *output = *value;
            }
        }

        Ok(())
    }
}
