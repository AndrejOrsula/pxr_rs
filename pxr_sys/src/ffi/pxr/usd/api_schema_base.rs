use crate::pxr;
use std::ops::{Deref, DerefMut};

// Emulate inheritance via Deref and DerefMut
impl Deref for pxr::UsdAPISchemaBase {
    type Target = pxr::UsdSchemaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdAPISchemaBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
