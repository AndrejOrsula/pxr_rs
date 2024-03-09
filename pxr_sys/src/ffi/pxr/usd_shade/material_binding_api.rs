use crate::pxr;

crate::macros::impl_vector_element!(
    [Opaque]
    "pxrInternal_v0_22__pxrReserved__$UsdShadeMaterialBindingAPI_CollectionBinding",
    "UsdShadeMaterialBindingAPI_CollectionBinding",
    pxr::UsdShadeMaterialBindingAPI_CollectionBinding
);

impl pxr::UsdShadeMaterialBindingAPI {
    pub fn bind(self: std::pin::Pin<&mut Self>, material: &pxr::UsdShadeMaterial) {
        unsafe {
            cpp::cpp!([self as "pxr::UsdShadeMaterialBindingAPI *", material as "const pxr::UsdShadeMaterial *"] {
                self->Bind(*material);
            });
        };
    }
}
