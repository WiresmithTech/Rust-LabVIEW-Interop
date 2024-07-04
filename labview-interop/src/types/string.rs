//! Handle the various string times that the LabVIEW
//! interface provides.
//!

use encoding_rs::Encoding;
use std::borrow::Cow;

use crate::errors::Result;
use crate::labview_layout;
use crate::memory::{LvOwned, UHandle, UPtr};

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

/// Definition of a handle to an LabVIEW String. Helper for FFI definition and
/// required for any functions that need to resize the string.
pub type LStrHandle = UHandle<LStr>;
/// Definition of a pointer to an LabVIEW String. Helper for FFI definition.
pub type LStrPtr = UPtr<LStr>;
/// Definition of an owned LStr Handle.
pub type LStrOwned = LvOwned<LStr>;

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
    pub fn new() -> Result<Self> {
        unsafe { LvOwned::<LStr>::new_unsized(|handle| handle.set(&[])) }
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
        unsafe { LvOwned::<LStr>::new_unsized(|handle| handle.set(data)) }
    }
}
