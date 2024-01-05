use crate::pxr;
use std::ops::{Deref, DerefMut};

// Emulate inheritance via Deref and DerefMut
impl Deref for pxr::UsdPhysicsJoint {
    type Target = pxr::UsdGeomImageable;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for pxr::UsdPhysicsDistanceJoint {
    type Target = pxr::UsdPhysicsJoint;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsDistanceJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for pxr::UsdPhysicsFixedJoint {
    type Target = pxr::UsdPhysicsJoint;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsFixedJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for pxr::UsdPhysicsPrismaticJoint {
    type Target = pxr::UsdPhysicsJoint;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsPrismaticJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for pxr::UsdPhysicsRevoluteJoint {
    type Target = pxr::UsdPhysicsJoint;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsRevoluteJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for pxr::UsdPhysicsSphericalJoint {
    type Target = pxr::UsdPhysicsJoint;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for pxr::UsdPhysicsSphericalJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
