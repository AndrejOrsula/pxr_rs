use crate::pxr;
use std::{
    mem::transmute,
    ops::{Deref, DerefMut},
    pin::Pin,
};

crate::macros::impl_pinned_deref_mut![
    pxr::SdfLayerRefPtr => pxr::SdfLayer,
    pxr::SdfLayerHandle => pxr::SdfLayer,
];

// SdfLayerRefPtr as SdfLayer
impl Deref for pxr::SdfLayerRefPtr {
    type Target = pxr::SdfLayer;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*(self as *const Self).cast::<*const Self::Target>()) }
    }
}
impl DerefMut for pxr::SdfLayerRefPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(*(self as *mut Self).cast::<*mut Self::Target>()) }
    }
}

// SdfLayerHandle as SdfLayer
impl Deref for pxr::SdfLayerHandle {
    type Target = pxr::SdfLayer;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*(self as *const Self).cast::<*const Self::Target>()) }
    }
}
impl DerefMut for pxr::SdfLayerHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(*(self as *mut Self).cast::<*mut Self::Target>()) }
    }
}

// SdfLayerRefPtr -> SdfLayerHandle
impl AsRef<pxr::SdfLayerHandle> for pxr::SdfLayerRefPtr {
    fn as_ref(&self) -> &pxr::SdfLayerHandle {
        unsafe { transmute(self) }
    }
}
impl AsMut<pxr::SdfLayerHandle> for pxr::SdfLayerRefPtr {
    fn as_mut(&mut self) -> &mut pxr::SdfLayerHandle {
        unsafe { transmute(self) }
    }
}
impl AsRef<pxr::SdfLayerHandle> for Pin<Box<pxr::SdfLayerRefPtr>> {
    fn as_ref(&self) -> &pxr::SdfLayerHandle {
        self.deref().as_ref()
    }
}

// SdfLayerHandle -> SdfLayerRefPtr
impl AsRef<pxr::SdfLayerRefPtr> for pxr::SdfLayerHandle {
    fn as_ref(&self) -> &pxr::SdfLayerRefPtr {
        unsafe { transmute(self) }
    }
}
impl AsMut<pxr::SdfLayerRefPtr> for pxr::SdfLayerHandle {
    fn as_mut(&mut self) -> &mut pxr::SdfLayerRefPtr {
        unsafe { transmute(self) }
    }
}
impl AsRef<pxr::SdfLayerRefPtr> for Pin<Box<pxr::SdfLayerHandle>> {
    fn as_ref(&self) -> &pxr::SdfLayerRefPtr {
        self.deref().as_ref()
    }
}
