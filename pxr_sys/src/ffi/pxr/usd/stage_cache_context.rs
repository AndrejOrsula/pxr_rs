use crate::pxr;
use cpp::{cpp, cpp_class};
use std::pin::Pin;

cpp_class!(
    pub unsafe struct UsdStageCacheContext as "pxr::UsdStageCacheContext"
);

crate::macros::impl_extern_type! {
    [Opaque]
    UsdStageCacheContext = "pxrInternal_v0_22__pxrReserved__::UsdStageCacheContext"
}

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdStageCacheContext",
    "UsdStageCacheContext",
    UsdStageCacheContext
);

impl UsdStageCacheContext {
    pub fn new(cache: &pxr::UsdStageCache) -> Pin<Box<UsdStageCacheContext>> {
        let stage_cache_context = unsafe {
            cpp!([cache as "pxr::UsdStageCache *"] -> UsdStageCacheContext as "pxr::UsdStageCacheContext" {
                return pxr::UsdStageCacheContext(*cache);
            })
        };
        Box::into_pin(Box::new(stage_cache_context))
    }
}
