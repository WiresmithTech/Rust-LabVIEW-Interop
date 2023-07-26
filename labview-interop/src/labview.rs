//! The LabVIEW module provides the LabVIEW memory manager methods
//! abstracting the exact linking methods from the rest of the modules.
//!

use std::ffi::c_void;

use ctor::ctor;
use dlopen2::wrapper::{Container, WrapperApi};

use crate::{errors::MgErr, memory::MagicCookie};

/// Represents as UHandle passed by value. Can't use the generic
/// version from the memory module else since the functions
/// aren't generic.
type UHandleValue = usize;

#[ctor]
pub static SYNC_API: Container<SyncApi> = load_sync_api();

#[ctor]
pub static MEMORY_API: Container<MemoryApi> = load_memory_api();

#[derive(WrapperApi)]
pub struct SyncApi {
    #[dlopen2_name = "PostLVUserEvent"]
    post_lv_user_event: unsafe extern "C" fn(reference: MagicCookie, data: *mut c_void) -> MgErr,
    #[dlopen2_name = "Occur"]
    occur: unsafe extern "C" fn(occurance: MagicCookie) -> MgErr,
}

pub fn load_sync_api() -> Container<SyncApi> {
    let cont: Container<SyncApi> =
        unsafe { Container::load_self().expect("Could not open library or load symbols") };
    cont
}

#[derive(WrapperApi)]
pub struct MemoryApi {
    #[dlopen2_name = "DSSetHandleSize"]
    set_handle_size: unsafe extern "C" fn(handle: UHandleValue, size: usize) -> MgErr,
}

pub fn load_memory_api() -> Container<MemoryApi> {
    let cont: Container<MemoryApi> =
        unsafe { Container::load_self().expect("Could not open library or load symbols") };
    cont
}
