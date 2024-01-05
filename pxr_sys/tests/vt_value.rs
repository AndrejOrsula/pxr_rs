use autocxx::WithinBox;
use pxr_sys::*;

macro_rules! setup_test {
    ($value:expr => $rs_type:ty) => {
        {
            let vt_value = pxr::VtValue::from($value);
            TryInto::<$rs_type>::try_into(&vt_value).unwrap();
        }
    };
    ($value:expr => $rs_type:ty [assert]) => {
        {
            let vt_value = pxr::VtValue::from($value);
            let rs_value: $rs_type = (&vt_value).try_into().unwrap();
            assert_eq!($value, rs_value);
        }
    };
    [$($value:expr => $rs_type:ty $([$assert:tt])?),* $(,)?] => {
        $(setup_test!($value => $rs_type $([$assert])?);)*
    };
}

#[test]
fn test_vt_value() {
    setup_test![
        true => bool [assert],
        1_u8 => u8 [assert],
        -2_i32 => i32 [assert],
        3_u32 => u32 [assert],
        -4_i64 => i64 [assert],
        5_u64 => u64 [assert],
        // todo!() => pxr::GfHalf,
        half::f16::from_f32(0.5) => half::f16 [assert],
        -0.5_f32 => f32 [assert],
        0.5_f64 => f64 [assert],
        &*pxr::SdfTimeCode::new(0.2).within_box() => pxr::SdfTimeCode,
        "test_string" => &str [assert],
        &*pxr::TfToken::new3(&"test_token".into_cpp()).within_box() => pxr::TfToken,
        // todo!() => pxr::SdfAssetPath,
        &*pxr::GfMatrix2d::new1(0.0, 1.0, 2.0, 3.0).within_box() => pxr::GfMatrix2d,
        &*pxr::GfMatrix3d::new1(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0).within_box() => pxr::GfMatrix3d,
        &*pxr::GfMatrix4d::new1(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0).within_box() => pxr::GfMatrix4d,
        &*pxr::GfQuatd::new2(1.0, 0.0, 0.0, 0.0).within_box() => pxr::GfQuatd,
        &*pxr::GfQuatf::new2(1.0, 0.0, 0.0, 0.0).within_box() => pxr::GfQuatf,
        // todo!() => pxr::GfQuath,
        &*pxr::GfVec2d::new2(0.0, 1.0).within_box() => pxr::GfVec2d,
        [0.0_f64, 1.0] => [f64; 2] [assert],
        &*pxr::GfVec2f::new2(0.0, 1.0).within_box() => pxr::GfVec2f,
        [0.0_f32, 1.0] => [f32; 2] [assert],
        // todo!() => pxr::GfVec2h,
        [half::f16::from_f32(0.0), half::f16::from_f32(1.0)] => [::half::f16; 2] [assert],
        &*pxr::GfVec2i::new2(0.into(), 1.into()).within_box() => pxr::GfVec2i,
        [0_i32, 1] => [i32; 2] [assert],
        &*pxr::GfVec3d::new2(0.0, 1.0, 2.0).within_box() => pxr::GfVec3d,
        [0.0_f64, 1.0, 2.0] => [f64; 3] [assert],
        &*pxr::GfVec3f::new2(0.0, 1.0, 2.0).within_box() => pxr::GfVec3f,
        [0.0_f32, 1.0, 2.0] => [f32; 3] [assert],
        // todo!() => pxr::GfVec3h,
        [half::f16::from_f32(0.0), half::f16::from_f32(1.0), half::f16::from_f32(2.0)] => [::half::f16; 3] [assert],
        &*pxr::GfVec3i::new2(0.into(), 1.into(), 2.into()).within_box() => pxr::GfVec3i,
        [0_i32, 1, 2] => [i32; 3] [assert],
        &*pxr::GfVec4d::new2(0.0, 1.0, 2.0, 3.0).within_box() => pxr::GfVec4d,
        [0.0_f64, 1.0, 2.0, 3.0] => [f64; 4] [assert],
        &*pxr::GfVec4f::new2(0.0, 1.0, 2.0, 3.0).within_box() => pxr::GfVec4f,
        [0.0_f32, 1.0, 2.0, 3.0] => [f32; 4] [assert],
        // todo!() => pxr::GfVec4h,
        [half::f16::from_f32(0.0), half::f16::from_f32(1.0), half::f16::from_f32(2.0), half::f16::from_f32(3.0)] => [::half::f16; 4] [assert],
        &*pxr::GfVec4i::new2(0.into(), 1.into(), 2.into(), 3.into()).within_box() => pxr::GfVec4i,
        [0_i32, 1, 2, 3] => [i32; 4] [assert],
    ];
}
