//! The LabVIEW module provides the LabVIEW memory manager methods
//! abstracting the exact linking methods from the rest of the modules.
//!

use std::ffi::c_void;

use ctor::ctor;
use dlopen2::wrapper::{Container, WrapperApi};
use once_cell::sync::OnceCell;

use crate::{errors::MgErr, memory::MagicCookie};

pub(crate) static SYNC_API: OnceCell<Container<SyncApi>> = OnceCell::new();

#[derive(WrapperApi)]
pub(crate) struct SyncApi {
    #[dlopen2_name = "PostLVUserEvent"]
    post_lv_user_event: unsafe extern "C" fn(reference: MagicCookie, data: *mut c_void) -> MgErr,
    #[dlopen2_name = "Occur"]
    occur: unsafe extern "C" fn(occurance: MagicCookie) -> MgErr,
}

pub(crate) fn load_sync_api() -> Container<SyncApi> {
    let cont: Container<SyncApi> =
        unsafe { Container::load_self().expect("Could not open library or load symbols") };
    cont
}

#[ctor]
fn init_interface() {
    SYNC_API.set(load_sync_api());
}
