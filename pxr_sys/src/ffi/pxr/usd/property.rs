use crate::pxr;
use std::ops::{Deref, DerefMut};

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdProperty",
    "UsdProperty",
    pxr::UsdProperty
);

// Emulate inheritance via Deref and DerefMut
impl Deref for pxr::UsdProperty {
    type Target = pxr::UsdObject;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
