//! The memory module handles the LabVIEW memory manager
//! functions and types.
//!
//! todo: get to reference without panics.
#[cfg(feature = "link")]
mod owned_handle;
mod uhandle;
mod uptr;

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

#[cfg(feature = "link")]
pub use owned_handle::OwnedUHandle;
pub use uhandle::UHandle;
pub use uptr::UPtr;

/// Extracted formatting logic which can be used for handles or owned values.
fn fmt_handle<T: Debug + ?Sized>(
    label: &str,
    handle: &UHandle<T>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    match unsafe { handle.as_ref() } {
        Ok(inner) => write!(f, "{label}({inner:?})"),
        Err(_) => write!(f, "{label}(Invalid)"),
    }
}
