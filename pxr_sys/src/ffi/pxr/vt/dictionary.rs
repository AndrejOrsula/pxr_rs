use cpp::{cpp, cpp_class};
use std::pin::Pin;

cpp_class!(
    pub unsafe struct VtDictionary as "pxr::VtDictionary"
);

crate::macros::impl_extern_type! {
    [Opaque]
    VtDictionary = "pxrInternal_v0_22__pxrReserved__::VtDictionary"
}

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$VtDictionary",
    "VtDictionary",
    VtDictionary
);

impl VtDictionary {
    #[must_use]
    pub fn new() -> Pin<Box<VtDictionary>> {
        let stage_cache_context = unsafe {
            cpp!([] -> VtDictionary as "pxr::VtDictionary" {
                return pxr::VtDictionary();
            })
        };
        Box::into_pin(Box::new(stage_cache_context))
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Pin<Box<VtDictionary>> {
        let stage_cache_context = unsafe {
            cpp!([capacity as "size_t"] -> VtDictionary as "pxr::VtDictionary" {
                return pxr::VtDictionary(capacity);
            })
        };
        Box::into_pin(Box::new(stage_cache_context))
    }
}
