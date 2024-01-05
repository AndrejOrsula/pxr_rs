use cpp::{cpp, cpp_class};
use std::pin::Pin;

cpp_class!(
    pub unsafe struct SdfCleanupEnabler as "pxr::SdfCleanupEnabler"
);

crate::macros::impl_extern_type! {
    [Opaque]
    SdfCleanupEnabler = "pxrInternal_v0_22__pxrReserved__::SdfCleanupEnabler"
}

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$SdfCleanupEnabler",
    "SdfCleanupEnabler",
    SdfCleanupEnabler
);

impl SdfCleanupEnabler {
    pub fn new() -> Pin<Box<SdfCleanupEnabler>> {
        Box::pin(unsafe {
            cpp!([] -> SdfCleanupEnabler as "pxr::SdfCleanupEnabler" {
                return pxr::SdfCleanupEnabler();
            })
        })
    }

    pub fn IsCleanupEnabled(&self) -> bool {
        unsafe {
            cpp!([self as "const pxr::SdfCleanupEnabler *"] -> bool as "bool" {
                return self->IsCleanupEnabled();
            })
        }
    }
}
