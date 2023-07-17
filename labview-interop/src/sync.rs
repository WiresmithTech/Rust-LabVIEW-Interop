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

/// Representation of a LabVIEW user event reference with type data.
///
/// Where the reference is passed into Rust you can use this typed form
/// to then allow proper type completions of the values.
///
/// From LabVIEW you can set the terminal to be `adapt to type` and `handles by value`
///
/// # Example
/// ```
/// # use labview_interop::sync::LVUserEvent;
/// # use labview_interop::errors::MgErr;
///#[no_mangle]
///pub extern "C" fn generate_event_3(lv_user_event: *mut LVUserEvent<i32>) -> MgErr {
///    let event = unsafe { *lv_user_event };
///    let result = event.post(&mut 3);
///    match result {
///        Ok(_) => MgErr::NO_ERROR,
///        Err(err) => err,
///    }
///}
/// ```
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct LVUserEvent<T> {
    reference: LVUserEventRef,
    _marker: PhantomData<T>,
}

impl<T> LVUserEvent<T> {
    /// Generate the user event with the provided data.
    ///
    /// Right now the data needs to be a mutable reference as the
    /// LabVIEW API does not specify whether it will not be modified.
    pub fn post(&self, data: &mut T) -> Result<()> {
        let mg_err = unsafe { PostLVUserEvent(self.reference, data as *mut T as *mut c_void) };
        mg_err.to_result(())
    }
}

/// A LabVIEW occurence which can be used to provide synchronisation
/// between execution of Rust and LabVIEW code.
///
/// From LabVIEW you can set the terminal to be `adapt to type` and `handles by value`
///
/// # Example
/// ```
/// # use labview_interop::sync::LVUserEvent;
/// # use labview_interop::errors::MgErr;
/// #[no_mangle]
///pub extern "C" fn generate_occurence(occurence: *mut Occurence) -> MgErr {
///    let result = unsafe { (*occurence).set() };
///    match result {
///        Ok(_) => MgErr::NO_ERROR,
///        Err(err) => err,
///    }
///}
/// ```
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Occurence(MagicCookie);

impl Occurence {
    /// "set" generates the occurence event which can be detected by LabVIEW.
    pub fn set(&self) -> Result<()> {
        let mg_err = unsafe { Occur(self.0) };
        mg_err.to_result(())
    }
}
