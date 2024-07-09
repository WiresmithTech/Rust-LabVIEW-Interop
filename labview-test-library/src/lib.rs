#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod arrays;
mod clusters;
#[cfg(target_pointer_width = "64")]
mod error_clusters;
mod memory_manager;
mod strings;
mod sync;
mod timestamps;

/// tests for arrays.
pub use crate::arrays::*;
/// Tests for clusters
pub use crate::clusters::*;
/// Tests for error cluster (64 bit only)
#[cfg(target_pointer_width = "64")]
pub use crate::error_clusters::*;
/// Tests for memory manager functions.
pub use crate::memory_manager::*;
/// Tests for strings
pub use crate::strings::*;
/// Tests for sync functions
pub use crate::sync::*;
/// Tests for timestamps.
pub use crate::timestamps::*;
