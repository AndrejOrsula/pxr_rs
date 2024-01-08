use crate::pxr;
use cpp::cpp;
use std::pin::Pin;

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdGeomXformOp",
    "UsdGeomXformOp",
    pxr::UsdGeomXformOp
);

// TODO: Make generic
impl pxr::UsdGeomXformOp {
    #[must_use]
    pub fn set_vec3f(self: Pin<&mut Self>, value: &pxr::GfVec3f) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdGeomXformOp *", value as "const pxr::GfVec3f *"] -> bool as "bool" {
                return self->Set(*value);
            })
        }
    }
}
