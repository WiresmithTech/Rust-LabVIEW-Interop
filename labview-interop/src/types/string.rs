//! Handle the various string times that the LabVIEW
//! interface provides.
//!

#[cfg(feature = "link")]
use crate::errors::Result;
use crate::labview_layout;
#[cfg(feature = "link")]
use crate::memory::OwnedUHandle;
use crate::memory::{LVCopy, UHandle, UPtr};
use encoding_rs::Encoding;
use std::borrow::Cow;

#[cfg(target_os = "windows")]
fn get_encoding() -> &'static Encoding {
    #[link(name = "kernel32")]
    extern "stdcall" {
        fn GetACP() -> u32;
    }

    //SAFETY: No real concerns with this call.
    let code_page = unsafe { GetACP() };

    // Crap - ctor errors again. I think it is reasonably safe
    // to assume LabVIEW isn't going to hit anything to unusual
    // due to it's level of support.
    codepage::to_encoding(code_page as u16).expect("Unknown code page.")
}

#[cfg(target_os = "linux")]
fn get_encoding() -> &'static Encoding {
    encoding_rs::WINDOWS_1252
}

#[cfg(target_os = "macos")]
fn get_encoding() -> &'static Encoding {
    encoding_rs::UTF_8
}

#[ctor::ctor]
/// The encoding that LabVIEW uses on the current platform.
pub(crate) static LV_ENCODING: &'static Encoding = get_encoding();

labview_layout!(
    /// Internal LabVIEW string structure.
    ///
    /// This is the recommended type when interfacing with LabVIEW
    /// as it is also the internal format so no translation is needed.
    pub struct LStr {
        size: i32,
        data: [u8],
    }
);

/// Copyable inside a handle.
impl LVCopy for LStr {}

/// Definition of a handle to an LabVIEW String. Helper for FFI definition and
/// required for any functions that need to resize the string.
pub type LStrHandle<'a> = UHandle<'a, LStr>;
/// Definition of a pointer to an LabVIEW String. Helper for FFI definition.
pub type LStrPtr = UPtr<LStr>;
/// Definition of an owned LStr Handle.
#[cfg(feature = "link")]
pub type LStrOwned = OwnedUHandle<LStr>;

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

    /// Get the size of this LStr instance.
    /// Would LStr ever be padded?
    pub fn size(&self) -> usize {
        std::mem::size_of::<i32>() + self.data.len()
    }

    /// Get the size of LStr given a specific data slice.
    /// Would LStr ever be padded?
    pub fn size_with_data(data: &[u8]) -> usize {
        std::mem::size_of::<i32>() + data.len()
    }

    /// Uses a system appropriate decoder to return a rust compatible string.
    ///
    /// This returns a [`std::borrow::Cow`] to avoid any allocations if the
    /// input is already valid UTF8.
    pub fn to_rust_string_with_encoding(&self, encoding: &'static Encoding) -> Cow<str> {
        let (result, _, _) = encoding.decode(self.as_slice());
        result
    }

    /// Uses a system appropriate decoder to return a rust compatible string.
    ///
    /// This returns a [`std::borrow::Cow`] to avoid any allocations if the
    /// input is already valid UTF8.
    ///
    /// # Example
    /// ```
    /// use labview_interop::types::LStrHandle;
    /// use labview_interop::errors::MgErr;
    /// #[no_mangle]
    /// pub extern "C" fn string_check(mut string: LStrHandle) -> MgErr {
    ///    let string_value = string.to_string();
    ///    format!("Read value: {string_value}");
    ///    MgErr::NO_ERROR
    /// }
    ///```
    pub fn to_rust_string(&self) -> Cow<str> {
        self.to_rust_string_with_encoding(&LV_ENCODING)
    }
}

impl std::fmt::Display for LStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rust_string())
    }
}

impl std::fmt::Debug for LStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.to_rust_string())
    }
}

impl PartialEq for LStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

/// Implement features that require a full string handle rather than just the [`LStr`]
/// type.
///
/// Requires the link feature.
#[cfg(feature = "link")]
impl<'a> LStrHandle<'a> {
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
    /// use labview_interop::types::LStrHandle;
    /// use labview_interop::errors::MgErr;
    /// #[no_mangle]
    /// pub extern "C" fn hello_world(mut string: LStrHandle) -> MgErr {
    ///    let result = string.set(b"Hello World");
    ///    result.into()
    /// }
    //```
    pub fn set(&mut self, value: &[u8]) -> Result<()> {
        let input_length = value.len();
        let struct_size = LStr::size_with_data(value);

        unsafe {
            //Safety: Is this alignment ever wrong. Would it even pad between the size and data.
            // I believe not.
            self.resize(struct_size)?;

            let l_str = self.as_ref_mut()?;
            l_str.size = input_length as i32;
            for (value, output) in value.iter().zip(l_str.data.iter_mut()) {
                *output = *value;
            }
        }

        Ok(())
    }

    /// Set string takes a Rust string and puts it into the LabVIEW String.
    ///
    /// This is a two step process:
    /// 1. Encode from Rust (UTF8) to LabVIEW encoding (based on system code page on Windows).
    /// 2. Write this encoding into the LabVIEW string.
    ///
    /// If the input is valid ASCII then no additional data copies are made. If not then this will
    /// allocate a new intermediate buffer to hold the decoded results before writing to the
    /// LabVIEW string.
    pub fn set_str(&mut self, value: &str) -> Result<()> {
        self.set_str_with_encoding(&LV_ENCODING, value)
    }

    /// Set string with encoder takes a Rust string and puts it into the LabVIEW String.
    ///
    /// This is a two step process:
    /// 1. Encode from Rust (UTF8) to LabVIEW encoding with the provided encoder.
    /// 2. Write this encoding into the LabVIEW string.
    ///
    /// If the input is valid ASCII then no additional data copies are made. If not then this will
    /// allocate a new intermediate buffer to hold the decoded results before writing to the
    /// LabVIEW string.
    ///
    /// The encoder should be an encoder provided by the encoding_rs crate.
    pub fn set_str_with_encoding(&mut self, encoder: &'static Encoding, value: &str) -> Result<()> {
        let (buffer, _, _) = encoder.encode(value);
        self.set(&buffer)
    }
}

#[cfg(feature = "link")]
impl LStrOwned {
    /// Create a new owned `LStr` with a size of zero.
    pub fn empty_string() -> Result<Self> {
        unsafe { OwnedUHandle::<LStr>::new_unsized(|handle| handle.set(&[])) }
    }
    ///
    /// # Example
    /// ```
    /// use labview_interop::types::{LStrHandle, LStrOwned};
    /// use labview_interop::errors::MgErr;
    /// #[no_mangle]
    /// pub extern "C" fn hello_world(mut strn: String, output_string: *mut LStrHandle) {
    ///    let handle = LStrOwned::from_data(strn.as_bytes()).unwrap();
    ///    unsafe {
    ///        handle.clone_into_pointer(output_string).unwrap();
    ///    }
    /// }
    /// ```
    pub fn from_data(data: &[u8]) -> Result<Self> {
        unsafe { OwnedUHandle::<LStr>::new_unsized(|handle| handle.set(data)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::{alloc, Layout, LayoutError};

    /// Implements a questionable allocation strategy based on
    /// https://www.reddit.com/r/rust/comments/mq3kqe/is_this_the_best_way_to_do_custom_dsts_unsized/
    ///
    /// For test purposes this is useful though.
    ///
    /// # Safety
    /// These can be used for read-only testing. Writing will want to resize which is unavailable here.
    impl LStr {
        pub(crate) fn layout_of(n: usize) -> std::result::Result<Layout, LayoutError> {
            // Build a layout describing an instance of this DST.
            let (layout, _) = Layout::new::<i32>().extend(Layout::array::<u8>(n)?)?;
            let layout = layout.pad_to_align();
            Ok(layout)
        }

        pub(crate) unsafe fn boxed_uninit(n: usize) -> Box<Self> {
            // Find the layout with a helper function.
            let layout = Self::layout_of(n).unwrap();
            // Make a heap allocation.
            let ptr = alloc(layout);
            // Construct a fat pointer by making a fake slice.
            // The first argument is the pointer, the second argument is the metadata.
            // In this case, its just the length of the slice.
            let ptr = core::slice::from_raw_parts(ptr, n);
            // Transmute the slice into the real fat pointer type.
            let ptr = std::mem::transmute::<_, *mut LStr>(ptr);
            // Build a box from the raw pointer.
            let b = Box::from_raw(ptr);
            // Make sure its the correct size.
            debug_assert_eq!(std::mem::size_of_val(&*ptr), layout.size());
            b
        }

        pub fn boxed_from_str(value: &str) -> Box<LStr> {
            let length = value.len();
            let bytes = value.as_bytes();
            let mut boxed = unsafe { Self::boxed_uninit(length) };
            boxed.size = length as i32;
            for (i, byte) in bytes.iter().enumerate() {
                boxed.data[i] = *byte;
            }
            boxed
        }
    }

    #[test]
    fn test_lstr_handle_debug() {
        let string = LStr::boxed_from_str("Hello World");
        let mut pointer = Box::into_raw(string);
        let raw_handle = std::ptr::addr_of_mut!(pointer);
        let handle = LStrHandle::from_raw(raw_handle);
        let debug = format!("{:?}", handle);
        assert!(debug.contains("Hello World"));
    }
}
