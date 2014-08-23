#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

#![feature(unsafe_destructor)]

//! Shareable data that lasts forever, with no reference count.

use std::mem;
use std::kinds::marker;

/// Shareable data that lasts forever, with no reference count.
pub struct Forever<T> {
    __data: *mut T,
    __marker: marker::NoCopy
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
    pub fn new(val: T) -> Forever<T> {
        Forever {
            __data: unsafe { mem::transmute(box val) },
            __marker: marker::NoCopy
        }
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
        Forever {
            __data: self.__data,
            __marker: marker::NoCopy
        }
    }
}

impl<T: Send + Sync> Deref<T> for Forever<T> {
    #[inline]
    fn deref(&self) -> &T {
        self.inner()
    }
}

#[test] fn test_lasts() {
    let a = Forever::new(7u); let b = a.clone();
    spawn(proc() {
        assert_eq!(*b, 7u);
    });
}

