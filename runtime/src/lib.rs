#![no_std]

//! The [Diplomat](https://rust-diplomat.github.io/diplomat/) runtime crate.
//!
//! This crate provides Diplomat-specific types for crates writing `#[diplomat::bridge]` modules.
//! Include this in any crate that also depends on `diplomat`, since `#[diplomat::bridge]`
//! will generate code that relies on types from here.
//!
//! # Type Overview
//!
//! - **Option and Result**: [`DiplomatResult<T, E>`] (FFI-safe `Result`) and [`DiplomatOption<T>`] (type alias for [`DiplomatResult<T, ()>`]).
//! - **Slices**: [`DiplomatSlice<'a, T>`] (`&'a [T]`), [`DiplomatSliceMut<'a, T>`] (`&'a mut [T]`), and [`DiplomatOwnedSlice<T>`] (`Box<[T]>`).
//! - **String input**: These all map to appropriate string types across FFI.
//!    - Unvalidated UTF-8: [`DiplomatStrSlice<'a>`] (`&'a [u8]`), [`DiplomatOwnedStrSlice`] (`Box<[u8]>`)
//!    - Unvalidated UTF-16: [`DiplomatStr16Slice<'a>`] (`&'a [u16]`), [`DiplomatOwnedStr16Slice`]  (`Box<[u16]>`)
//!    - Validated UTF-8: [`DiplomatUtf8StrSlice<'a>`] (`&'a str`), [`DiplomatOwnedUTF8StrSlice`] (`Box<str>`)
//!    - Unvalidated stringy DSTs: [`DiplomatStr`] (`[u8]`), [`DiplomatStr16`] (`[u16]`), for convenient use with `&` and `Box` as function parameters.
//! - **String output**: [`DiplomatWrite`]
//! - **Callbacks**: [`DiplomatCallback<ReturnType>`] (FFI-safe callback handle).
//! - **Scalars**: [`DiplomatChar`] (`u32`) and [`DiplomatByte`] (`u8`).
//!
//! Note that many of these are type aliases; using these types instead of the type alias signals that Diplomat is expected to
//! generate the equivalent type on the other side. For example, using `DiplomatChar` will signal that a language's `char` type is
//! to be used, even if the actual type seen by rust is an unvalidated `u32`.
//!
//! # Features
//!
//! The `log` feature enables logging support, currently enabled via the wasm-only `diplomat_init()`.
//!
//! The `jvm-callback-support` feature should be enabled if building Diplomat for use in the JVM, for
//! a Diplomat-based library that uses callbacks.

extern crate alloc;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm_glue;

pub mod ffi;

mod write;
pub use write::DiplomatWrite;
// TODO(#958): Remove these top-level re-exports in 1.0
#[doc(hidden)]
pub use ffi::{
    diplomat_alloc, diplomat_buffer_write_create, diplomat_buffer_write_destroy, diplomat_free,
    diplomat_is_str, diplomat_owned_slice_u8_destroy, diplomat_simple_write,
};
mod slices;
pub use slices::{
    DiplomatOwnedSlice, DiplomatOwnedStr16Slice, DiplomatOwnedStrSlice, DiplomatOwnedUTF8StrSlice,
    DiplomatSlice, DiplomatSliceMut, DiplomatStr16Slice, DiplomatStrSlice, DiplomatUtf8StrSlice,
};

mod callback;
pub use callback::DiplomatCallback;

mod result;
pub use result::{DiplomatOption, DiplomatResult};

pub mod rust_interop;

/// Like [`char`], but unvalidated.
///
/// This type will usually map to some character type in the target language, and
/// you will not need to worry about the safety of mismatched string invariants.
pub type DiplomatChar = u32;

/// Like [`str`], but unvalidated.
///
/// This is a dynamically sized type, it should be used behind an `&` or a `Box<T>`
///
/// This type will usually map to some string type in the target language, and
/// you will not need to worry about the safety of mismatched string invariants.
///
/// [`DiplomatStrSlice`] is equivalent to `&DiplomatStr`: both are provided since
/// `&DiplomatStr` is more convenient but not allowed in Diplomat structs (since it
/// is not directly FFI safe). Instead, this type can be conveniently used in function
/// parameter lists.
pub type DiplomatStr = [u8];

/// Like `Wstr`, but unvalidated.
///
/// This is a dynamically sized type, it should be used behind an `&` or a `Box<T>`
///
/// This type will usually map to some string type in the target language, and
/// you will not need to worry about the safety of mismatched string invariants.
///
/// [`DiplomatStr16Slice`] is equivalent to `&DiplomatStr16`: both are provided since
/// `&DiplomatStr16` is more convenient but not allowed in Diplomat structs (since it
/// is not directly FFI safe). Instead, this type can be conveniently used in function
/// parameter lists.
pub type DiplomatStr16 = [u16];

/// Like [`u8`], but interpreted explicitly as a raw byte as opposed to a numerical value.
///
/// This matters for languages like JavaScript or Dart, where there's only a single numeric
/// type, but special types for byte buffers.
pub type DiplomatByte = u8;
