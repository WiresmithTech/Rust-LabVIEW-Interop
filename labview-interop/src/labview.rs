//! The LabVIEW module provides the LabVIEW memory manager methods
//! abstracting the exact linking methods from the rest of the modules.
//!

use std::ffi::c_void;

use ctor::ctor;
use dlopen2::wrapper::{Container, WrapperApi};

use crate::{
    errors::{LVInteropError, LVStatusCode, MgErr, Result},
    memory::MagicCookie,
};

/// Represents as UHandle passed by value. Can't use the generic
/// version from the memory module else since the functions
/// aren't generic.
pub(crate) type UHandleValue = usize;

/// Represents as UPtr passed by value. Can't use the generic
/// version from the memory module else since the functions
/// aren't generic.
pub(crate) type UPtrValue = usize;

#[ctor]
static SYNC_API: Option<Container<SyncApi>> = unsafe { Container::load_self().ok() };

pub fn sync_api() -> Result<&'static Container<SyncApi>> {
    SYNC_API.as_ref().ok_or(LVInteropError::NoLabviewApi)
}

#[ctor]
static MEMORY_API: Option<Container<MemoryApi>> = unsafe { Container::load_self().ok() };

pub fn memory_api() -> Result<&'static Container<MemoryApi>> {
    MEMORY_API.as_ref().ok_or(LVInteropError::NoLabviewApi)
}

/// The LabVIEW synchronisation features are part of the Support Manager API.
///
/// The [official documentation](https://www.ni.com/docs/en-US/bundle/labview-api-ref/page/properties-and-methods/lv-manager/support-manager-functions.html) for the Support Manager can be found (last verified 2024-jul-09) on the webpage of National Instruments.
#[derive(WrapperApi)]
pub struct SyncApi {
    /// Posts the given user event. The event and associated data are queued for all event structures registered for the event.
    ///
    /// ```C
    /// MgErr PostLVUserEvent(LVUserEventRef ref, void *data);
    /// ```
    ///
    /// - `ref`: `LVUserEventRef`, Event refnum for the event for which you want to post data.
    /// - `*data`: `void*`, Address of the data to post. The data must match the type used to create the user event.
    ///
    /// Returns `MgErr`: `NoErr` or `mgArgErr` (corresponds to gen. err. code 1: not a valid user event)
    #[dlopen2_name = "PostLVUserEvent"]
    post_lv_user_event: unsafe extern "C" fn(reference: MagicCookie, data: *mut c_void) -> MgErr,

    /// Triggers the specified occurrence. All block diagrams that are waiting for this occurrence stop waiting.
    ///
    /// ```C
    /// MgErr Occur(Occurrence occ);
    /// ```
    ///
    ///  - `occ`: `Occurrence`, refnum you want to trigger.
    ///
    ///  Returns `MgErr`: `NoErr` or `mgArgErr` (corresponds to gen. err. code 1: not a valid user event)
    #[dlopen2_name = "Occur"]
    occur: unsafe extern "C" fn(occurance: MagicCookie) -> MgErr,
}

/// The [official documentation](https://www.ni.com/docs/en-US/bundle/labview-api-ref/page/properties-and-methods/lv-manager/memory-manager-functions.html) for the LabVIEW Memory Manager can be found (last verified 2024-jul-09) on the webpage of National Instruments.
#[derive(WrapperApi)]
pub struct MemoryApi {
    /// Verifies that the specified handle is a handle. If it is not a handle, this function returns mZoneErr.
    /// ```C
    /// MgErr DSCheckHandle(handle);
    /// ```
    /// - `handle`: `UHandle`, handle you want to verify.
    ///
    /// Returns `MgErr`: `noErr` or `mZoneErr`
    #[dlopen2_name = "DSCheckHandle"]
    check_handle: unsafe extern "C" fn(handle: UHandleValue) -> MgErr,

    /// Verifies that the specified pointer is allocated with XX NewPtr or XX NewPClr. If it is not a pointer, this function returns mZoneErr.
    ///
    /// ```C
    /// MgErr DSCheckPtr(ptr);
    /// ```
    ///  - `ptr`: `UPtr`, Pointer you want to verify.
    ///
    /// Returns `MgErr`: `noErr` or `mZoneErr`
    #[dlopen2_name = "DSCheckPtr"]
    check_ptr: unsafe extern "C" fn(ptr: UPtrValue) -> MgErr,

    /// Creates a new handle to a relocatable block of memory of the specified size.
    ///
    /// The routine aligns all handles and pointers in DS to accommodate the largest possible data
    /// representations for the platform in use.
    ///
    /// ```C
    /// UHandle DSNewHandle(size_t size);
    /// ```
    ///  - `size`: `size_t`, Size, in bytes, of the handle you want to create.
    ///
    /// Returns `UHandle` or `NULL` on error
    ///
    /// The allocated memory is uninitialized.
    /// `DSNewHClr` is an alternative that also initialized the memory to zero.
    #[dlopen2_name = "DSNewHandle"]
    new_handle: unsafe extern "C" fn(size: usize) -> *mut *mut std::ffi::c_void,

    /// Copies the data referenced by the handle hsrc into the handle pointed to by ph or a new handle if ph points to NULL.
    ///
    /// ```C
    /// MgErr DSCopyHandle(void *ph, const void *hsrc)
    /// ```
    ///
    /// - `*ph`: `UHandle*`, Pointer to the handle to copy the data into. This must point to a valid handle or NULL. If it points to NULL, a new handle is allocated.
    /// - `hsrc`: `UHandle`, The handle containing the data to copy.
    ///
    /// Returns `MgErr`: `NoErr`, `mZoneErr` or `mFullErr` (corresponds to gen. err. code 2))
    ///
    /// There is no further clarification in the official documentation, we wonder what happens if:
    /// - ... if the memory the handle points too, is too small to receive hsrc? --> Test?
    /// - ... if the memory contains another handle? Is it a deep copy, or a shallow copy? Guess: Shallow Copy,  --> Test?
    #[dlopen2_name = "DSCopyHandle"]
    copy_handle: unsafe extern "C" fn(ph: *mut UHandleValue, hsrc: UHandleValue) -> MgErr,

    /// Releases the memory referenced by the specified handle.
    ///
    /// ```C
    /// MgErr DSDisposeHandle(h);
    /// ```
    ///
    ///  - `h`: `UHandle`, Handle you want to dispose of.
    ///
    /// Returns `MgErr`: `NoErr` or `mZoneErr`
    #[dlopen2_name = "DSDisposeHandle"]
    dispose_handle: unsafe extern "C" fn(handle: UHandleValue) -> MgErr,

    /// Changes the size of the block of memory referenced by the specified handle.
    ///
    /// To use this function to resize an array handle, you must calculate how many bytes the resized array requires. Many platforms have memory alignment requirements that make it difficult to determine the correct size for the resulting array. Learn about how LabVIEW stores data in memory to calculate the size and alignment of array elements, especially for arbitrary data types such as clusters.
    ///
    /// To resize a handle for a numeric array, use the `NumericArrayResize` manager function instead of `DSSetHandleSize``.
    ///
    /// **Do not use this function on a locked handle.**
    ///
    /// ```C
    /// MgErr DSSetHandleSize(h, size);
    /// ```
    ///
    ///   - `h`: `UHandle`, Handle you want to resize.
    ///   - `size`: `size_t`, New size, in bytes, of the handle.
    ///
    ///   Returns `MgErr`: `noErr`, `mZoneErr`, `mFullErr` (corresponds to gen. err. code 2))
    #[dlopen2_name = "DSSetHandleSize"]
    set_handle_size: unsafe extern "C" fn(handle: UHandleValue, size: usize) -> MgErr,

    /// Converts a numeric error code to the associated text description. This function recognizes error codes from any installed National Instruments product.
    ///
    /// ```C
    /// LVBoolean NIGetOneErrorCode(int32 errCode, LStrHandle *errText);
    /// ```
    ///
    /// - `errCode`: `int32`, Numeric error code
    /// - `*errText`: `LStrHandle*`, Output: Address at which `NIGetOneErrorCode` stores the error code description. This parameter is a pointer.
    ///
    /// Returns `LVBoolean`: false -> This function did not find the value of errCode in any of the error text files.
    ///                      true -> This function found the value of errCode in one of the error text files.
    error_code_description:
        unsafe extern "C" fn(error_code: i32, error_text: *mut UHandleValue) -> bool,

    /// Resizes a data handle that refers to a numeric array. This routine also accounts for alignment issues. It does not set the array dimension field. If *dataHP is NULL, LabVIEW allocates a new array handle in *dataHP.
    ///
    /// ```C
    /// MgErr NumericArrayResize (int32 typeCode, int32 numDims, Uhandle *dataHP, size_t totalNewSize)
    /// ```
    ///
    ///  - `typeCode`: `int32`, Data type for the array you want to resize.
    ///  - `numDims`: `int32`, Number of dimensions in the data structure to which the handle refers.
    ///  - `*dataHP`: `UHandle*`, UHandle pointer to the handle you want to resize.
    ///  - `totalNewSize`: `size_t`, New number of elements to which the handle refers.
    ///
    ///  Returns MgErr:  `MgErr`: `noErr`, `mZoneErr`, `mFullErr` (corresponds to gen. err. code 2))
    ///
    /// valid type codes:
    ///
    /// |   code   | type |
    /// | -------- | ---- |
    /// | 01 or iB |   8-bit integer|
    /// | 02 or iW |  16-bit integer|
    /// | 03 or iL |  32-bit integer|
    /// | 04 or iQ |  64-bit integer|
    /// | 05 or uB |   8-bit unsigned integer|
    /// | 06 or uW |  16-bit unsigned integer|
    /// | 07 or uL |  32-bit unsigned integer|
    /// | 08 or uQ |  64-bit unsigned integer|
    /// | 09 or fs |  Single-precision, floating-point number|
    /// | 0A or fD |  Double-precision, floating-point number|
    /// | 0B or fX |  Extended-precision, floating-point number|
    /// | 0C or cS |  Complex single-precision, floating-point number|
    /// | 0D or cD |  Complex double-precision, floating-point number|
    /// | 0E or cX |  Complex extended-precision, floating-point number|
    ///
    #[dlopen2_name = "NumericArrayResize"]
    numeric_array_resize: unsafe extern "C" fn(
        type_code: i32,
        number_of_dims: i32,
        handle_ptr: *mut UHandleValue,
        total_new_size: usize,
    ) -> MgErr,
}
