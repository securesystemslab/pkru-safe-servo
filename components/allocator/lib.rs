/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Selecting the default global allocator for Servo

//#[global_allocator]
//static ALLOC: Allocator = Allocator;

#![feature(use_extern_macros)]
pub extern crate pkmallocator;
pub use crate::platform::*;

//#[cfg(not(windows))]
//pub use jemalloc_sys;

#[cfg(not(windows))]
mod platform {
    //use jemalloc_sys as ffi;
    pub use super::pkmallocator;
    pub use super::pkmallocator::untrusted;

    use std::alloc::{GlobalAlloc, Layout};
    use std::os::raw::{c_int, c_void};

    /// Get the size of a heap block.
    pub unsafe extern "C" fn usable_size(ptr: *const c_void) -> usize {
        //ffi::malloc_usable_size(ptr as *const _)
        pkmallocator::libc_compat::malloc_usable_size(ptr as *const _)
    }

    /// Memory allocation APIs compatible with libc
    pub mod libc_compat {
        pub use super::pkmallocator::libc_compat::{free, malloc, realloc};
    }
}

#[cfg(windows)]
mod platform {
    pub use std::alloc::System as Allocator;
    use std::os::raw::c_void;
    use winapi::um::heapapi::{GetProcessHeap, HeapSize, HeapValidate};

    /// Get the size of a heap block.
    pub unsafe extern "C" fn usable_size(mut ptr: *const c_void) -> usize {
        let heap = GetProcessHeap();

        if HeapValidate(heap, 0, ptr) == 0 {
            ptr = *(ptr as *const *const c_void).offset(-1);
        }

        HeapSize(heap, 0, ptr) as usize
    }
}
