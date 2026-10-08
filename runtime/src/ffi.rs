//! Low-level `#[no_mangle] extern "C"` functions exposed by `diplomat-runtime`.
//!
//! These functions are primarily intended to be called by Diplomat-generated FFI
//! bindings in foreign languages (C, C++, JavaScript/Wasm, Dart, Kotlin, .NET, etc.),
//! rather than directly from Rust code, though they are exposed here if needed.

use alloc::alloc::Layout;

pub use crate::write::{
    diplomat_buffer_write_create, diplomat_buffer_write_destroy, diplomat_buffer_write_get_bytes,
    diplomat_buffer_write_len, diplomat_simple_write,
};

/// Allocates a buffer of a given size in Rust's memory.
///
/// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
///
/// # Safety
/// - The allocated buffer must be freed with [`diplomat_free()`].
#[no_mangle]
pub unsafe extern "C" fn diplomat_alloc(size: usize, align: usize) -> *mut u8 {
    alloc::alloc::alloc(Layout::from_size_align(size, align).unwrap())
}

/// Frees a buffer that was allocated in Rust's memory.
///
/// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
///
/// # Safety
/// - `ptr` must be a pointer to a valid buffer allocated by [`diplomat_alloc()`].
#[no_mangle]
pub unsafe extern "C" fn diplomat_free(ptr: *mut u8, size: usize, align: usize) {
    alloc::alloc::dealloc(ptr, Layout::from_size_align(size, align).unwrap())
}

/// Frees a `Box<[u8]>` that was returned across FFI as a raw `(ptr, len)`
/// pair (e.g. via `DiplomatOwnedSlice<u8>`).
///
/// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
///
/// # Safety
/// - `ptr`/`len` must be the raw parts of a `Box<[u8]>` that Rust allocated and handed across
///   FFI; this reconstructs that box and drops it, so the same allocator that made it frees it.
/// - Must not be called more than once for the same `ptr`.
#[no_mangle]
pub unsafe extern "C" fn diplomat_owned_slice_u8_destroy(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        drop(alloc::boxed::Box::from_raw(
            core::ptr::slice_from_raw_parts_mut(ptr, len),
        ));
    }
}

/// Whether a `&[u8]` is a `&str`.
///
/// Primarily to be called by generated FFI bindings, not Rust code, but is available if needed.
///
/// # Safety
/// - `ptr` and `size` must be a valid `&[u8]`
#[no_mangle]
pub unsafe extern "C" fn diplomat_is_str(ptr: *const u8, size: usize) -> bool {
    core::str::from_utf8(core::slice::from_raw_parts(ptr, size)).is_ok()
}
