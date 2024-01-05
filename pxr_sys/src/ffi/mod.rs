//! Unsafe FFI bindings that are either generated via `bindgen` and `autocxx`
//! or manually designed via `cpp` macros.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;
pub mod pxr;
