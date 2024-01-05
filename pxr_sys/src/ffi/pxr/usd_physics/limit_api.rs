use crate::pxr;
use std::ops::{Deref, DerefMut};

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdPhysicsLimitAPI",
    "UsdPhysicsLimitAPI",
    pxr::UsdPhysicsLimitAPI
);

// Emulate inheritance via Deref and DerefMut
impl Deref for pxr::UsdPhysicsLimitAPI {
    type Target = pxr::UsdAPISchemaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsLimitAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
