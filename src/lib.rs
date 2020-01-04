#![no_std]
#![allow(non_camel_case_types)]
#![feature(static_nobundle)]
#![feature(link_args)]
use core::ffi::c_void;
use libc::size_t;

extern "C" {
    pub fn for_rust_free(ptr: *mut c_void) -> c_void;
    pub fn for_rust_malloc(size: size_t) -> *mut c_void;
    pub fn for_rust_memalign(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn for_rust_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn for_rust_realloc_aligned(ptr: *mut c_void, size: size_t, alignment: size_t) -> *mut c_void;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Add;

    #[test]
    fn it_frees_memory_malloc() {
        let ptr = unsafe { for_rust_memalign(8, 8) } as *mut u8;
        unsafe { for_rust_free(ptr as *mut c_void) };
    }

    #[test]
    fn it_frees_memory_realloc() {
        let ptr = unsafe { for_rust_memalign(8, 8) } as *mut u8;
        let ptr = unsafe { for_rust_realloc_aligned(ptr as *mut c_void, 13, 16) } as *mut u8;
        unsafe { for_rust_free(ptr as *mut c_void) };
    }
}