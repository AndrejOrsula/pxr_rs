use crate::pxr;
use std::ops::{Deref, DerefMut};

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdRelationship",
    "UsdRelationship",
    pxr::UsdRelationship
);

// Emulate inheritance via Deref and DerefMut
impl Deref for pxr::UsdRelationship {
    type Target = pxr::UsdProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
