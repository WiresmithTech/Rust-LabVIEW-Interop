use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU8, Ordering};
use labview_interop::errors::LVInteropError;
use labview_interop::memory::{RefNum, CookieJar, OwnedUPtr};
use labview_interop::types::ErrorClusterPtr;

static MUTEX_JAR: OnceLock<Mutex<CookieJar<AtomicU8>>> = OnceLock::new();

/// Setup a refnum of a mutex.
#[unsafe(no_mangle)]
pub extern "C" fn create_mutex(mut error_cluster_ptr: ErrorClusterPtr) -> RefNum<AtomicU8> {
    error_cluster_ptr.wrap_function(RefNum::null(), || {
        let jar = MUTEX_JAR.get_or_init(|| Mutex::new(CookieJar::new().unwrap()));
        let mutex = OwnedUPtr::new(AtomicU8::new(0u8))?;
        jar.lock().unwrap().new_refnum(mutex.as_inner())
    })
}

/// Set the mutex value.
#[unsafe(no_mangle)]
pub extern "C" fn set_mutex_value(mut error_cluster_ptr: ErrorClusterPtr, refnum: RefNum<AtomicU8>, value: u8) {
    println!("Refnum: {refnum:?}");
    error_cluster_ptr.wrap_function((), || -> Result<(), LVInteropError> {
        let jar = MUTEX_JAR.get().unwrap();
        let inner = jar.lock().unwrap().cookie_info(refnum)?;
        inner.store(value, Ordering::SeqCst);
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn get_mutex_value(mut error_cluster_ptr: ErrorClusterPtr, refnum: RefNum<AtomicU8>) -> u8 {
    println!("Refnum: {refnum:?}");
    error_cluster_ptr.wrap_function(0u8, || -> Result<u8, LVInteropError> {
        let jar = MUTEX_JAR.get().unwrap();
        let inner = jar.lock().unwrap().cookie_info(refnum)?;
        let value = inner.load(Ordering::SeqCst);
        Ok(value)
    })
}