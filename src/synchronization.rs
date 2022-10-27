// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2022 Andre Richter <andre.o.richter@gmail.com>

//! Synchronization primitives.
//!
//! # Resources
//!
//!   - <https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html>
//!   - <https://stackoverflow.com/questions/59428096/understanding-the-send-trait>
//!   - <https://doc.rust-lang.org/std/cell/index.html>

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Synchronization interfaces.
pub mod interface {
    // Any object implementing this trait guarantees exclusive access to the data wrapped within
    // the Mutex for the duration of the provided closure.
    pub trait Mutex {
        /// The type of the data that is wrapped by this mutex.
        type Data;
        /// Locks the mutex and grants the closure temporary mutable access to the wrapped data.
        fn lock<'a, R>(&'a self, func: impl FnOnce(&'a mut Self::Data) -> R) -> R;
    }
}

//
// Implement SpinLock
//

pub struct SpinLock<T>
where
    T: ?Sized,
{
    is_lock: UnsafeCell<AtomicBool>,
    data: UnsafeCell<T>
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

// implement Send, Sync trait for this Lock allow this Lock could be used among threads/processes
unsafe impl<T> Send for SpinLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for SpinLock<T> where T: ?Sized + Send {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        return Self {
            is_lock: UnsafeCell::new(AtomicBool::new(false)),
            data: UnsafeCell::new(data)
        };
    }
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------
impl<T> interface::Mutex for SpinLock<T> {
    type Data = T;

    fn lock<'a, R>(&'a self, func: impl FnOnce(&'a mut Self::Data) -> R) -> R {

        unsafe {
            let cell_is_lock = self.is_lock.get();
            loop {
                // protect another thread if access self.data if one thread accessed this data.
                if (*cell_is_lock).compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                    // we have right to access data
                    let data = &mut *self.data.get();
                    let result = func(data);
                    // clear the lock
                    (*cell_is_lock).store(false, Ordering::Relaxed);
                    return result;
                }
            }
        }
    }
}
