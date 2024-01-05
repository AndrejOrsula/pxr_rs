macro_rules! gen_vt_value {
    ($rs_type:ty as $cpp_type:expr $(=> $cpp_alias_type:expr)?) => {
        {
            let code = r#"
            // Rust $rs_type as C++ pxr::VtValue<$cpp_type>
            #[automatically_derived]
            impl From<$rs_type> for crate::pxr::VtValue {
                fn from(value: $rs_type) -> Self {
                    unsafe {
                        ::cpp::cpp!([value as "$cpp_type"] -> crate::pxr::VtValue as "pxr::VtValue" {
                            return pxr::VtValue(value);
                        })
                    }
                }
            }
            #[automatically_derived]
            impl From<&$rs_type> for crate::pxr::VtValue {
                fn from(value: &$rs_type) -> Self {
                    unsafe {
                        ::cpp::cpp!([value as "const $cpp_type *"] -> crate::pxr::VtValue as "pxr::VtValue" {
                            return pxr::VtValue(*value);
                        })
                    }
                }
            }
            #[automatically_derived]
            impl TryFrom<crate::pxr::VtValue> for $rs_type {
                type Error = String;
                fn try_from(value: crate::pxr::VtValue) -> Result<Self, Self::Error> {
                    if value.GetTypeName().to_string() == "$cpp_alias_type" {
                        Ok(unsafe {
                            ::cpp::cpp!([value as "pxr::VtValue"] -> $rs_type as "$cpp_type" {
                                return value.Get<$cpp_type>();
                            })
                        })
                    } else {
                        Err(format!(
                            "Cannot cast C++ type pxr::VtValue<{}> to Rust type $rs_type.",
                            value.GetTypeName().to_string()
                        ))
                    }
                }
            }
            #[automatically_derived]
            impl TryFrom<&crate::pxr::VtValue> for $rs_type {
                type Error = String;
                fn try_from(value: &crate::pxr::VtValue) -> Result<Self, Self::Error> {
                    if value.GetTypeName().to_string() == "$cpp_alias_type" {
                        Ok(unsafe {
                            ::cpp::cpp!([value as "const pxr::VtValue *"] -> $rs_type as "$cpp_type" {
                                return value->Get<$cpp_type>();
                            })
                        })
                    } else {
                        Err(format!(
                            "Cannot cast C++ type pxr::VtValue<{}> to Rust type $rs_type.",
                            value.GetTypeName().to_string()
                        ))
                    }
                }
            }
            #[automatically_derived]
            impl AsRef<$rs_type> for crate::pxr::VtValue {
                fn as_ref(&self) -> &$rs_type {
                    unsafe {
                        ::cpp::cpp!([self as "const pxr::VtValue *"] -> &$rs_type as "const $cpp_type *" {
                            return &(self->Get<$cpp_type>());
                        })
                    }
                }
            }
            "#;

            let code = code.replace("$rs_type", stringify!($rs_type));
            let code = code.replace("$cpp_type", $cpp_type);

            #[allow(unused_variables)]
            let cpp_alias_type = $cpp_type;
            $(let cpp_alias_type = $cpp_alias_type;)?
            let code = code.replace("$cpp_alias_type", cpp_alias_type);

            code
        }
    };
    [$($rs_type:ty as $cpp_type:expr $(=> $cpp_alias_type:expr)?),* $(,)?] => {
        {
            let code = String::new();
            $(
                let code = format!("{code}\n{}", gen_vt_value!($rs_type as $cpp_type $(=> $cpp_alias_type)?));
            )*
            code
        }
    };
}

pub fn codegen(out: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let code = gen_vt_value![
        bool as "bool",
        u8 as "uint8_t" => "unsigned char",
        i32 as "int32_t" => "int",
        u32 as "uint32_t" => "unsigned int",
        i64 as "int64_t" => "long",
        u64 as "uint64_t" => "unsigned long",
        crate::pxr::GfHalf as "pxr::GfHalf" => "pxr_half::half",
        ::half::f16 as "pxr::GfHalf" => "pxr_half::half",
        f32 as "float",
        f64 as "double",
        crate::pxr::SdfTimeCode as "pxr::SdfTimeCode" => "SdfTimeCode",
        // String is implemented manually
        crate::pxr::TfToken as "pxr::TfToken" => "TfToken",
        crate::pxr::SdfAssetPath as "pxr::SdfAssetPath" => "SdfAssetPath",
        crate::pxr::GfMatrix2d as "pxr::GfMatrix2d" => "GfMatrix2d",
        crate::pxr::GfMatrix3d as "pxr::GfMatrix3d" => "GfMatrix3d",
        crate::pxr::GfMatrix4d as "pxr::GfMatrix4d" => "GfMatrix4d",
        crate::pxr::GfQuatd as "pxr::GfQuatd" => "GfQuatd",
        crate::pxr::GfQuatf as "pxr::GfQuatf" => "GfQuatf",
        crate::pxr::GfQuath as "pxr::GfQuath" => "GfQuath",
        crate::pxr::GfVec2d as "pxr::GfVec2d" => "GfVec2d",
        [f64; 2] as "pxr::GfVec2d" => "GfVec2d",
        crate::pxr::GfVec2f as "pxr::GfVec2f" => "GfVec2f",
        [f32; 2] as "pxr::GfVec2f" => "GfVec2f",
        crate::pxr::GfVec2h as "pxr::GfVec2h" => "GfVec2h",
        [::half::f16; 2] as "pxr::GfVec2h" => "GfVec2h",
        crate::pxr::GfVec2i as "pxr::GfVec2i" => "GfVec2i",
        [i32; 2] as "pxr::GfVec2i" => "GfVec2i",
        crate::pxr::GfVec3d as "pxr::GfVec3d" => "GfVec3d",
        [f64; 3] as "pxr::GfVec3d" => "GfVec3d",
        crate::pxr::GfVec3f as "pxr::GfVec3f" => "GfVec3f",
        [f32; 3] as "pxr::GfVec3f" => "GfVec3f",
        crate::pxr::GfVec3h as "pxr::GfVec3h" => "GfVec3h",
        [::half::f16; 3] as "pxr::GfVec3h" => "GfVec3h",
        crate::pxr::GfVec3i as "pxr::GfVec3i" => "GfVec3i",
        [i32; 3] as "pxr::GfVec3i" => "GfVec3i",
        crate::pxr::GfVec4d as "pxr::GfVec4d" => "GfVec4d",
        [f64; 4] as "pxr::GfVec4d" => "GfVec4d",
        crate::pxr::GfVec4f as "pxr::GfVec4f" => "GfVec4f",
        [f32; 4] as "pxr::GfVec4f" => "GfVec4f",
        crate::pxr::GfVec4h as "pxr::GfVec4h" => "GfVec4h",
        [::half::f16; 4] as "pxr::GfVec4h" => "GfVec4h",
        crate::pxr::GfVec4i as "pxr::GfVec4i" => "GfVec4i",
        [i32; 4] as "pxr::GfVec4i" => "GfVec4i",
    ];
    crate::codegen_write(&code, out)
}
