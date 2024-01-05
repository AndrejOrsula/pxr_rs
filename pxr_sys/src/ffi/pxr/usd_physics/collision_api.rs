use crate::pxr;
use std::ops::{Deref, DerefMut};

// Emulate inheritance via Deref and DerefMut
impl Deref for pxr::UsdPhysicsCollisionAPI {
    type Target = pxr::UsdAPISchemaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsCollisionAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
