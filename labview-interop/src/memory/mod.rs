//! The memory module handles the LabVIEW memory manager
//! functions and types.
//!
//! todo: get to reference without panics.
#[cfg(feature = "link")]
mod owned_handle;
mod uptr;
mod uhandle;

use std::fmt::Debug;

/// A trait which defines that a type should be copyable inside
/// of a LabVIEW handle.
///
/// This is unique from `Copy` since unsized types can be inside a handle
/// but they can't implement `Copy`.
pub trait LVCopy {}

/// Rust copy types should be copyable in LabVIEW.
impl<T: Copy> LVCopy for T {}

/// Magic cookie type used for various reference types in the memory manager.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
#[doc(hidden)]
pub struct MagicCookie(u32);

pub use uptr::UPtr;
pub use uhandle::UHandle;
#[cfg(feature = "link")]
pub use owned_handle::OwnedUHandle;

/// Extracted formatting logic which can be used for handles or owned values.
fn fmt_handle<T: Debug>(label: &str, handle: &UHandle<T>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match unsafe { handle.as_ref() } {
        Ok(inner) => write!(f, "{label}({inner:?})"),
        Err(_) => write!(f, "{label}(Invalid)"),
    }
}