//! The sync module provides access to the
//! functions which allow for synchronising
//! back to labview.
//!

use std::ffi::c_void;
use std::marker::PhantomData;

use crate::errors::{MgErr, Result};
use crate::labview::{load_sync_api, SYNC_API};
use crate::memory::MagicCookie;

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
        let mg_err = unsafe {
            SYNC_API
                .get_unchecked()
                .post_lv_user_event(self.reference, data as *mut T as *mut c_void)
        };
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
        let mg_err = unsafe { SYNC_API.get_unchecked().occur(self.0) };
        mg_err.to_result(())
    }
}
