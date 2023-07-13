//! The sync module provides access to the
//! functions which allow for synchronising
//! back to labview.
//!

// todo add user event and occurance.

use libloading::{Library, Symbol};
use std::ffi::c_void;
use std::marker::PhantomData;

use crate::errors::{MgErr, Result};
use crate::memory::MagicCookie;

/*
struct SyncMethods<'lib> {
    pub post_lv_user_event:
        Symbol<'lib, unsafe extern "C" fn(reference: LVUserEventRef, data: *mut c_void) -> MgErr>,
}

impl<'lib> SyncMethods<'lib> {
    unsafe fn load() -> Self {
        let lib = Library::new("labview").unwrap();
        let post_lv_user_event = lib.get(b"PostLVUserEvent").unwrap();
        return SyncMethods { post_lv_user_event };
    }
}
*/

extern "C" {
    fn PostLVUserEvent(reference: LVUserEventRef, data: *mut c_void) -> MgErr;
    fn Occur(occurance: MagicCookie) -> MgErr;
}

type LVUserEventRef = MagicCookie;

#[repr(transparent)]
pub struct LVUserEvent<T> {
    reference: LVUserEventRef,
    _marker: PhantomData<T>,
}

impl<T> LVUserEvent<T> {
    pub fn post(&self, data: &mut T) -> Result<()> {
        let mg_err = unsafe { PostLVUserEvent(self.reference, data as *mut T as *mut c_void) };
        mg_err.to_result(())
    }
    /*
    pub fn post_dynamic(&self, data: &mut T) -> Result<()> {
        let mg_err = unsafe {
            let lv = Library::new("labview").map_err(|_| 1)?;
            let post: Symbol<
                unsafe extern "C" fn(reference: LVUserEventRef, data: *mut c_void) -> MgErr,
            > = lv.get(b"PostLVUserEvent").map_err(|_| 2)?;
            post(self.reference, data as *mut T as *mut c_void)
        };
        if mg_err != 0 {
            Err(mg_err)
        } else {
            Ok(())
        }
    }
    */
}

#[repr(transparent)]
pub struct Occurence(MagicCookie);

impl Occurence {
    pub fn set(&self) -> Result<()> {
        let mg_err = unsafe { Occur(self.0) };
        mg_err.to_result(())
    }
}
