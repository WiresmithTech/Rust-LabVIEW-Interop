use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use crate::errors::LVInteropError;
use crate::labview::memory_api;
use crate::memory::UPtr;

/// An owned UPtr which means we implement drop to free the memory.
///
/// This should be used with caution to avoid double free.
///
/// Functionally this is equivalent to a Box<T>
#[derive(Debug)]
pub struct OwnedUPtr<T: 'static>(UPtr<'static, T>);

impl<T> OwnedUPtr<T> {
    pub fn new_uninit() -> Result<MaybeUninit<Self>, LVInteropError> {
        unsafe {
            let allocated = memory_api()?.new_ptr(size_of::<T>()) as *mut T;
            Ok(MaybeUninit::new(Self(UPtr::from_raw(allocated))))
        }
    }
    /// Allocate a new UPtr with the provided data. This then operates
    /// like the `Box` type.
    ///
    /// You can then use this like a UPtr type but it will be freed on drop.
    ///
    /// ## Errors
    ///
    /// This can error if the memory API is unavailable.
    pub fn new(data: T) -> Result<Self, LVInteropError> {
        let pointer = unsafe {
            let allocated = memory_api()?.new_ptr(size_of::<T>()) as *mut T;
            *allocated = data;
            UPtr::from_raw(allocated)
        };
        Ok(Self(pointer))
    }

    /// Get back to an owned type for a UPtr that you know
    /// you need to free when you are finished with it.
    ///
    /// This is analogous to `Box::from_raw` and requires
    /// management across FFI boundaries.
    ///
    /// ## Safety
    ///
    /// This will be freed when dropped. Ensure you are
    /// responsible for the drop to avoid double frees.
    pub unsafe fn from_raw(ptr: UPtr<'static, T>) -> Self {
        Self(ptr)
    }
    
    /// Get the value as a UPtr
    pub fn as_inner(&self) -> UPtr<'_, T> {
        unsafe {
            UPtr::from_raw(self.0.as_uptr_value() as *mut T)
        }
    }
}

impl<T: Sized> Drop for OwnedUPtr<T> {
    fn drop(&mut self) {
        if let Some(memory_api) = memory_api().ok() {
            unsafe {
                memory_api.dispose_ptr(self.0.as_uptr_value());
            }
        }
    }
}

impl<T> Deref for OwnedUPtr<T> {
    type Target = UPtr<'static, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OwnedUPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
