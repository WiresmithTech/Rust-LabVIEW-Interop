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
type UHandleValue = usize;

#[ctor]
static SYNC_API: Option<Container<SyncApi>> = Container::load_self().ok();

pub fn sync_api() -> Result<&'static Container<SyncApi>> {
    SYNC_API.as_ref().ok_or(LVInteropError::NoLabviewApi)
}

#[ctor]
static MEMORY_API: Option<Container<MemoryApi>> = Container::load_self().ok();

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
    #[dlopen2_name = "DSSetHandleSize"]
    set_handle_size: unsafe extern "C" fn(handle: UHandleValue, size: usize) -> MgErr,
}
