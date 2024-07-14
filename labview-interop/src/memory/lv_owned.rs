
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::{LvCopy, UHandle};
use crate::errors::{LVInteropError, Result};
use crate::labview::memory_api;

/// A value allocated in the LabVIEW memory managed by us.
///
/// This will manage the lifetime and free the handle on drop.
///
/// This is a semantic difference from handle and is transparent with the handle data.
///
/// This means it can be used in structs in place of a handle.
///
/// # Example In Struct (LStrOwned is equivalent of `LvOwned<LStr>`).
/// ```no_run
///# use labview_interop::labview_layout;
///# use labview_interop::types::LStrOwned;
///labview_layout!(
///pub struct UserEventCluster {
///    eventno: i32,
///    id: LStrOwned,
///}
///);
///```
#[repr(transparent)]
pub struct LvOwned<T: ?Sized + 'static>(UHandle<'static, T>);

impl<T: Copy + 'static> LvOwned<T> {
    /// Create a new handle to a sized value of `T`.
    ///
    /// It will copy the data from the provided value.
    pub fn new(value: &T) -> Result<Self> {
        let handle = unsafe { memory_api()?.new_handle(std::mem::size_of::<T>()) } as *mut *mut T;

        if handle.is_null() {
            Err(LVInteropError::HandleCreationFailed)
        } else {
            // Copy the value into the handle.
            // # Safety - these pointers have just been created by the memory manager and we checked null.
            unsafe { **handle = *value; }
            Ok(Self(UHandle(handle, PhantomData)))
        }
    }
}

impl<T: ?Sized> LvOwned<T> {
    /// Create a new handle to the type `T`. It will create an empty handle
    /// which you must initialise with the `init_routine`.
    /// This is useful for unsized types.
    ///
    /// # Safety
    ///
    /// * This will create a handle to un-initialized memory. The provided initialisation
    ///    routine must prepare the memory.
    pub(crate) unsafe fn new_unsized(
        init_routine: impl FnOnce(&mut UHandle<'static, T>) -> Result<()>,
    ) -> Result<Self> {
        let handle = memory_api()?.new_handle(0);
        if handle.is_null() {
            Err(LVInteropError::HandleCreationFailed)
        } else {
            let mut new_value = UHandle(handle as *mut *mut T, PhantomData);
            init_routine(&mut new_value)?;
            Ok(Self(new_value))
        }
    }

    /// Return a new handle to the inner value.
    ///
    /// This takes a mutable borrow on the owned value as you can use the handle
    /// to modify the inner value.
    ///
    /// Note: This is only if you need a true handle to put into a structure that is expecting this.
    /// Better options are :
    /// * If you can define the type, just define it with the owned value. An owned value can take the place of a handle.
    /// * If you just need access to the data then use the Deref methods to access the handle.
    ///
    /// # Safety
    ///
    /// * This needs to take a mutable reference to self and lifetime annotation on UHandle,
    ///    in order to avoid creating multiple UHandles.
    ///
    /// # Examples
    ///
    /// ## Use Handle in Struct
    ///
    /// ```no_run
    /// use labview_interop::types::{LStrHandle, LStrOwned};
    /// use labview_interop::labview_layout;
    ///
    /// // This must have a handle due to other uses.
    /// labview_layout! {
    ///   struct ClusterWithString<'a> {
    ///     string_handle: LStrHandle<'a>
    ///   }
    /// }
    ///
    /// // Mutable is required since once you have a handle you can mutate the data.
    /// let mut owned_string = LStrOwned::from_data(b"Hello World!").unwrap();
    /// let handle = owned_string.handle_to_inner();
    /// let cluster = ClusterWithString {
    ///   string_handle: handle
    /// };
    /// // Do something with the cluster.
    ///
    /// ```
    ///
    /// ## Lifetime Guarantees - Single Handle
    /// ```compile_fail,E0515
    /// use labview_interop::memory::LvOwned;
    ///
    /// let mut owned = LvOwned::<f64>::new().unwrap();
    /// let mut handle = owned.handle_to_inner();
    /// // Cannot get a second handle due to lifetime.
    /// // This fails to compile.
    /// let handle2 = owned.handle_to_inner();
    ///
    /// *handle = 1.0;
    ///
    /// ```
    ///
    /// ## Lifetime Guarantees - Owned Outlives Handle
    ///
    /// ```compile_fail,E0515
    /// use labview_interop::memory::LvOwned;
    ///
    /// let mut owned = LvOwned::<f64>::new().unwrap();
    /// let mut handle = owned.handle_to_inner();
    /// // Cannot drop owned because we have a handle.
    /// // This fails to compile.
    /// drop(owned);
    ///
    /// *handle = 1.0;
    ///
    /// ```
    pub fn handle_to_inner(&mut self) -> UHandle<'_, T> {
        UHandle(self.0 .0, PhantomData)
    }
}

impl<T: ?Sized> Deref for LvOwned<T> {
    type Target = UHandle<'static, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for LvOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> Drop for LvOwned<T> {
    fn drop(&mut self) {
        let result = memory_api()
            .map(|api| unsafe { api.dispose_handle(self.0 .0 as usize).to_result(()) });
        if let Err(e) | Ok(Err(e)) = result {
            println!("Error freeing handle from LV: {e}");
        }
    }
}

impl<T: Debug> Debug for LvOwned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::fmt_handle("LvOwned", &self.0, f)
    }
}

impl<'a, T: LvCopy + 'static> UHandle<'a, T> {
    /// Try to create an owned handle from the current handle.
    ///
    /// The owned handle will have its own handle to the data and
    /// will be responsible for freeing it.
    ///
    /// # Safety
    ///
    /// * If the source handle is null, this may cause UB.
    ///
    /// # Errors
    ///
    /// * If there is not enough memory to create the handle this may error.
    unsafe fn try_to_owned(&self) -> Result<LvOwned<T>> {
        LvOwned::new_unsized(|handle| unsafe {
            self.clone_into_pointer(handle as *mut UHandle<'static, T>)
        })
    }
}

/// # Safety
///
/// * LvOwned memory is access through UHandle which is managed by the Labview Memory Manager, which is thread safe
unsafe impl<T: ?Sized> Send for LvOwned<T> {}
unsafe impl<T: ?Sized> Sync for LvOwned<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lvowned_debug() {
        let mut value = 42;
        let mut value_ptr = std::ptr::addr_of_mut!(value);
        let handle = UHandle(std::ptr::addr_of_mut!(value_ptr), std::marker::PhantomData);
        let owned = LvOwned(handle);
        assert_eq!(format!("{:?}", owned), "LvOwned(42)");
    }
}