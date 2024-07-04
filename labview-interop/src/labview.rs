//! The LabVIEW module provides the LabVIEW memory manager methods
//! abstracting the exact linking methods from the rest of the modules.
//!

use std::ffi::c_void;

use ctor::ctor;
use dlopen2::wrapper::{Container, WrapperApi};

use crate::{
    errors::{LVInteropError, MgErr, Result},
    memory::MagicCookie,
};

/// Represents as UHandle passed by value. Can't use the generic
/// version from the memory module else since the functions
/// aren't generic.
pub(crate) type UHandleValue = usize;

#[ctor]
static SYNC_API: Option<Container<SyncApi>> = unsafe { Container::load_self().ok() };

pub fn sync_api() -> Result<&'static Container<SyncApi>> {
    SYNC_API.as_ref().ok_or(LVInteropError::NoLabviewApi)
}

#[ctor]
static MEMORY_API: Option<Container<MemoryApi>> = unsafe { Container::load_self().ok() };

pub fn memory_api() -> Result<&'static Container<MemoryApi>> {
    MEMORY_API.as_ref().ok_or(LVInteropError::NoLabviewApi)
}

#[derive(WrapperApi)]
pub struct SyncApi {
    #[dlopen2_name = "PostLVUserEvent"]
    post_lv_user_event: unsafe extern "C" fn(reference: MagicCookie, data: *mut c_void) -> MgErr,
    #[dlopen2_name = "Occur"]
    occur: unsafe extern "C" fn(occurance: MagicCookie) -> MgErr,
}

#[derive(WrapperApi)]
pub struct MemoryApi {
    #[dlopen2_name = "DSNewHandle"]
    new_handle: unsafe extern "C" fn(size: usize) -> *mut *mut std::ffi::c_void,
    #[dlopen2_name = "DSCopyHandle"]
    copy_handle: unsafe extern "C" fn(ph: *mut UHandleValue, hsrc: UHandleValue) -> MgErr,
    #[dlopen2_name = "DSDisposeHandle"]
    dispose_handle: unsafe extern "C" fn(handle: UHandleValue) -> MgErr,
    #[dlopen2_name = "DSSetHandleSize"]
    set_handle_size: unsafe extern "C" fn(handle: UHandleValue, size: usize) -> MgErr,
    #[dlopen2_name = "NumericArrayResize"]
    numeric_array_resize: unsafe extern "C" fn(
        type_code: i32,
        number_of_dims: i32,
        handle_ptr: *mut UHandleValue,
        total_new_size: usize,
    ) -> MgErr,
}
