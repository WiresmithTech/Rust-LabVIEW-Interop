//! This module wraps the "Cookie Jar" functionality of LabVIEW to allow
//! you to create your own unique references in LabVIEW.

use std::marker::PhantomData;
use crate::labview::{cookie_api};
use crate::memory::{MagicCookie, UPtr, OwnedUPtr};

pub type Result<T> = crate::errors::Result<T>;

pub struct CookieJar<T: Sync> {
    jar: crate::labview::CookieJar,
    phantom_data: PhantomData<T>
}
//todo: Need to think about these.
unsafe impl<T: Sync> Send for CookieJar<T> {}
unsafe impl<T: Sync> Sync for CookieJar<T> {}

impl<T: Sync + 'static> CookieJar<T> {
    pub fn new() -> Result<Self> {
        let jar = unsafe {
            cookie_api()?.new_big_jar(size_of::<T>() as i32)
        };
        Ok(Self {
            jar,
            phantom_data: PhantomData
        })
    }

    pub fn new_refnum(&mut self, cookie_info: UPtr<T>) -> Result<RefNum<T>> {
        let cookie = unsafe {
            cookie_api()?.new_cookie(self.jar, cookie_info.as_uptr_value())
        };
        Ok(RefNum {
            cookie,
            phantom_data: PhantomData
        })

    }

    pub fn cookie_info(&self, ref_num: RefNum<T>) -> Result<OwnedUPtr<T>> {
        unsafe {
            let ptr = OwnedUPtr::<T>::new_uninit()?;
            let uptr_raw = ptr.as_ptr().as_ref().unwrap().as_uptr_value();
            cookie_api()?.get_cookie_info(self.jar, ref_num.cookie, uptr_raw).to_specific_result(())?;
            Ok(ptr.assume_init())
        }

    }

    pub fn dispose_refnum(&mut self, ref_num: RefNum<T>) -> Result<()>{
        let info = self.cookie_info(ref_num)?;
        unsafe {
            cookie_api()?.dispose_cookie(self.jar, ref_num.cookie, info.as_uptr_value()).to_specific_result(())?;
        }
        todo!("Cleanup Routine")
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RefNum<T> {
    cookie: MagicCookie,
    phantom_data: PhantomData<T>
}

impl<T> RefNum<T> {
    pub fn null() -> Self {
        Self {
            cookie: MagicCookie::null(),
            phantom_data: PhantomData
        }
    }
}

impl<T> Clone for RefNum<T> {
    fn clone(&self) -> Self {
        Self {
            cookie: self.cookie,
            phantom_data: PhantomData
        }
    }
}

impl<T> Copy for RefNum<T> {}