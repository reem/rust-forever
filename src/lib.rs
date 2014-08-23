#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

#![feature(unsafe_destructor)]

//! Shareable data that lasts forever, with no reference count.

use std::mem;

/// Shareable data that lasts forever, with no reference count.
#[unsafe_no_drop_flag]
pub struct Forever<T> {
    __data: *mut T
}

impl<T: Send + Sync> Forever<T> {
    /// Create a new, never-dropping, wrapper around T.
    ///
    /// This means T will *never* be deallocated and can be
    /// the source of memory leaks if you are not careful
    /// about when you create a Forever.
    ///
    /// For that reason, this function is marked unsafe.
    #[inline]
    pub unsafe fn new(val: T) -> Forever<T> {
        Forever { __data: mem::transmute(box val) }
    }

    /// Get an immutable reference to the contents of a Forever.
    ///
    /// This is safe because the underlying data is Send + Sync
    /// anyway.
    #[inline]
    pub fn inner(&self) -> &T {
        unsafe { mem::transmute(self.__data) }
    }

    /// Drop the value stored in Forever.
    ///
    /// This is unsafe because you can trivially easily cause dangling
    /// pointers if any other Forever's to the same data still exist.
    pub unsafe fn destroy(self) {
        drop(mem::transmute::<*mut T, Box<T>>(self.__data))
    }
}

impl<T: Send + Sync> Clone for Forever<T> {
    #[inline]
    fn clone(&self) -> Forever<T> {
        Forever { __data: self.__data }
    }
}

impl<T: Send + Sync> Deref<T> for Forever<T> {
    #[inline]
    fn deref(&self) -> &T {
        self.inner()
    }
}

#[unsafe_destructor]
impl<T: Sync + Send> Drop for Forever<T> {
    // Dropping this does nothing.
    // This can be the source of memory leaks if you are not careful!
    #[inline]
    fn drop(&mut self) {}
}

