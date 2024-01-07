use crate::pxr;
use cpp::cpp_class;
use std::{
    mem::transmute,
    ops::{Deref, DerefMut},
    pin::Pin,
};

cpp_class!(
    pub unsafe struct UsdStageWeakPtr as "pxr::UsdStageWeakPtr"
);

crate::macros::impl_extern_type! {
    [Opaque]
    pxr::UsdStageWeakPtr = "pxrInternal_v0_22__pxrReserved__::UsdStageWeakPtr"
}

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdStageWeakPtr",
    "UsdStageWeakPtr",
    pxr::UsdStageWeakPtr
);

crate::macros::impl_pinned_deref_mut![
    pxr::UsdStageRefPtr => pxr::UsdStage,
    pxr::UsdStageWeakPtr => pxr::UsdStage,
];

// UsdStageRefPtr as UsdStage
impl Deref for pxr::UsdStageRefPtr {
    type Target = pxr::UsdStage;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*(self as *const pxr::UsdStageRefPtr).cast::<*const pxr::UsdStage>()) }
    }
}
impl DerefMut for pxr::UsdStageRefPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(*(self as *mut pxr::UsdStageRefPtr).cast::<*mut pxr::UsdStage>()) }
    }
}

// UsdStageWeakPtr as UsdStage
impl Deref for pxr::UsdStageWeakPtr {
    type Target = pxr::UsdStage;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*(self as *const pxr::UsdStageWeakPtr).cast::<*const pxr::UsdStage>()) }
    }
}
impl DerefMut for pxr::UsdStageWeakPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(*(self as *mut pxr::UsdStageWeakPtr).cast::<*mut pxr::UsdStage>()) }
    }
}

// UsdStageRefPtr -> UsdStageWeakPtr
impl AsRef<pxr::UsdStageWeakPtr> for pxr::UsdStageRefPtr {
    fn as_ref(&self) -> &pxr::UsdStageWeakPtr {
        unsafe { transmute(self) }
    }
}
impl AsMut<pxr::UsdStageWeakPtr> for pxr::UsdStageRefPtr {
    fn as_mut(&mut self) -> &mut pxr::UsdStageWeakPtr {
        unsafe { transmute(self) }
    }
}
impl AsRef<pxr::UsdStageWeakPtr> for Pin<Box<pxr::UsdStageRefPtr>> {
    fn as_ref(&self) -> &pxr::UsdStageWeakPtr {
        self.deref().as_ref()
    }
}

// UsdStageWeakPtr -> UsdStageRefPtr
impl AsRef<pxr::UsdStageRefPtr> for pxr::UsdStageWeakPtr {
    fn as_ref(&self) -> &pxr::UsdStageRefPtr {
        unsafe { transmute(self) }
    }
}
impl AsMut<pxr::UsdStageRefPtr> for pxr::UsdStageWeakPtr {
    fn as_mut(&mut self) -> &mut pxr::UsdStageRefPtr {
        unsafe { transmute(self) }
    }
}
impl AsRef<pxr::UsdStageRefPtr> for Pin<Box<pxr::UsdStageWeakPtr>> {
    fn as_ref(&self) -> &pxr::UsdStageRefPtr {
        self.deref().as_ref()
    }
}

// UsdStageRefPtr -> UsdStageHandle (pxr::TfWeakPtr<pxr::UsdStage>)
impl AsRef<pxr::UsdStageHandle> for pxr::UsdStageRefPtr {
    fn as_ref(&self) -> &pxr::UsdStageHandle {
        unsafe { transmute(self) }
    }
}
impl AsMut<pxr::UsdStageHandle> for pxr::UsdStageRefPtr {
    fn as_mut(&mut self) -> &mut pxr::UsdStageHandle {
        unsafe { transmute(self) }
    }
}
impl AsRef<pxr::UsdStageHandle> for Pin<Box<pxr::UsdStageRefPtr>> {
    fn as_ref(&self) -> &pxr::UsdStageHandle {
        self.deref().as_ref()
    }
}

// UsdStageWeakPtr -> UsdStageHandle (pxr::TfWeakPtr<pxr::UsdStage>)
impl AsRef<pxr::UsdStageHandle> for pxr::UsdStageWeakPtr {
    fn as_ref(&self) -> &pxr::UsdStageHandle {
        unsafe { transmute(self) }
    }
}
impl AsMut<pxr::UsdStageHandle> for pxr::UsdStageWeakPtr {
    fn as_mut(&mut self) -> &mut pxr::UsdStageHandle {
        unsafe { transmute(self) }
    }
}
impl AsRef<pxr::UsdStageHandle> for Pin<Box<pxr::UsdStageWeakPtr>> {
    fn as_ref(&self) -> &pxr::UsdStageHandle {
        self.deref().as_ref()
    }
}
