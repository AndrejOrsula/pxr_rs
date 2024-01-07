use crate::pxr;
use cpp::cpp;
use std::pin::Pin;

impl pxr::UsdUtilsStageCache {
    #[must_use]
    pub fn Get1() -> Pin<Box<pxr::UsdStageCache>> {
        let stage_cache = unsafe {
            cpp!([]
                -> pxr::UsdStageCache as "pxr::UsdStageCache" {
                return pxr::UsdUtilsStageCache::Get();
            })
        };
        Box::into_pin(Box::new(stage_cache))
    }
}
