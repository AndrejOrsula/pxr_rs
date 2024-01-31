use crate::pxr;
use cpp::cpp;
use std::pin::Pin;

pub enum SdfValueTypeNames {
    Bool,
    UChar,
    Int,
    UInt,
    Int64,
    UInt64,
    Half,
    Float,
    Double,
    TimeCode,
    String,
    Token,
    Asset,
    Int2,
    Int3,
    Int4,
    Half2,
    Half3,
    Half4,
    Float2,
    Float3,
    Float4,
    Double2,
    Double3,
    Double4,
    Point3h,
    Point3f,
    Point3d,
    Vector3h,
    Vector3f,
    Vector3d,
    Normal3h,
    Normal3f,
    Normal3d,
    Color3h,
    Color3f,
    Color3d,
    Color4h,
    Color4f,
    Color4d,
    Quath,
    Quatf,
    Quatd,
    Matrix2d,
    Matrix3d,
    Matrix4d,
    Frame4d,
    TexCoord2h,
    TexCoord2f,
    TexCoord2d,
    TexCoord3h,
    TexCoord3f,
    TexCoord3d,
    BoolArray,
    UCharArray,
    IntArray,
    UIntArray,
    Int64Array,
    UInt64Array,
    HalfArray,
    FloatArray,
    DoubleArray,
    TimeCodeArray,
    StringArray,
    TokenArray,
    AssetArray,
    Int2Array,
    Int3Array,
    Int4Array,
    Half2Array,
    Half3Array,
    Half4Array,
    Float2Array,
    Float3Array,
    Float4Array,
    Double2Array,
    Double3Array,
    Double4Array,
    Point3hArray,
    Point3fArray,
    Point3dArray,
    Vector3hArray,
    Vector3fArray,
    Vector3dArray,
    Normal3hArray,
    Normal3fArray,
    Normal3dArray,
    Color3hArray,
    Color3fArray,
    Color3dArray,
    Color4hArray,
    Color4fArray,
    Color4dArray,
    QuathArray,
    QuatfArray,
    QuatdArray,
    Matrix2dArray,
    Matrix3dArray,
    Matrix4dArray,
    Frame4dArray,
    TexCoord2hArray,
    TexCoord2fArray,
    TexCoord2dArray,
    TexCoord3hArray,
    TexCoord3fArray,
    TexCoord3dArray,
}

impl SdfValueTypeNames {
    #[must_use]
    pub fn into_value_type_name(self) -> Pin<Box<pxr::SdfValueTypeName>> {
        self.into()
    }
}

impl From<SdfValueTypeNames> for Pin<Box<pxr::SdfValueTypeName>> {
    fn from(value: SdfValueTypeNames) -> Self {
        Box::pin(unsafe {
            match value {
                SdfValueTypeNames::Bool => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Bool;
                    })
                }

                SdfValueTypeNames::UChar => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->UChar;
                    })
                }
                SdfValueTypeNames::Int => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int;
                    })
                }
                SdfValueTypeNames::UInt => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->UInt;
                    })
                }
                SdfValueTypeNames::Int64 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int64;
                    })
                }
                SdfValueTypeNames::UInt64 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->UInt64;
                    })
                }
                SdfValueTypeNames::Half => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half;
                    })
                }
                SdfValueTypeNames::Float => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float;
                    })
                }
                SdfValueTypeNames::Double => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double;
                    })
                }
                SdfValueTypeNames::TimeCode => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TimeCode;
                    })
                }
                SdfValueTypeNames::String => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->String;
                    })
                }
                SdfValueTypeNames::Token => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Token;
                    })
                }
                SdfValueTypeNames::Asset => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Asset;
                    })
                }
                SdfValueTypeNames::Int2 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int2;
                    })
                }
                SdfValueTypeNames::Int3 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int3;
                    })
                }
                SdfValueTypeNames::Int4 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int4;
                    })
                }
                SdfValueTypeNames::Half2 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half2;
                    })
                }
                SdfValueTypeNames::Half3 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half3;
                    })
                }
                SdfValueTypeNames::Half4 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half4;
                    })
                }
                SdfValueTypeNames::Float2 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float2;
                    })
                }
                SdfValueTypeNames::Float3 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float3;
                    })
                }
                SdfValueTypeNames::Float4 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float4;
                    })
                }
                SdfValueTypeNames::Double2 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double2;
                    })
                }
                SdfValueTypeNames::Double3 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double3;
                    })
                }
                SdfValueTypeNames::Double4 => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double4;
                    })
                }
                SdfValueTypeNames::Point3h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Point3h;
                    })
                }
                SdfValueTypeNames::Point3f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Point3f;
                    })
                }
                SdfValueTypeNames::Point3d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Point3d;
                    })
                }
                SdfValueTypeNames::Vector3h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Vector3h;
                    })
                }
                SdfValueTypeNames::Vector3f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Vector3f;
                    })
                }
                SdfValueTypeNames::Vector3d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Vector3d;
                    })
                }
                SdfValueTypeNames::Normal3h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Normal3h;
                    })
                }
                SdfValueTypeNames::Normal3f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Normal3f;
                    })
                }
                SdfValueTypeNames::Normal3d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Normal3d;
                    })
                }
                SdfValueTypeNames::Color3h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color3h;
                    })
                }
                SdfValueTypeNames::Color3f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color3f;
                    })
                }
                SdfValueTypeNames::Color3d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color3d;
                    })
                }
                SdfValueTypeNames::Color4h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color4h;
                    })
                }
                SdfValueTypeNames::Color4f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color4f;
                    })
                }
                SdfValueTypeNames::Color4d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color4d;
                    })
                }
                SdfValueTypeNames::Quath => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Quath;
                    })
                }
                SdfValueTypeNames::Quatf => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Quatf;
                    })
                }
                SdfValueTypeNames::Quatd => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Quatd;
                    })
                }
                SdfValueTypeNames::Matrix2d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Matrix2d;
                    })
                }
                SdfValueTypeNames::Matrix3d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Matrix3d;
                    })
                }
                SdfValueTypeNames::Matrix4d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Matrix4d;
                    })
                }
                SdfValueTypeNames::Frame4d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Frame4d;
                    })
                }
                SdfValueTypeNames::TexCoord2h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord2h;
                    })
                }
                SdfValueTypeNames::TexCoord2f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord2f;
                    })
                }
                SdfValueTypeNames::TexCoord2d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord2d;
                    })
                }
                SdfValueTypeNames::TexCoord3h => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord3h;
                    })
                }
                SdfValueTypeNames::TexCoord3f => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord3f;
                    })
                }
                SdfValueTypeNames::TexCoord3d => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord3d;
                    })
                }
                SdfValueTypeNames::BoolArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->BoolArray;
                    })
                }
                SdfValueTypeNames::UCharArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->UCharArray;
                    })
                }
                SdfValueTypeNames::IntArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->IntArray;
                    })
                }
                SdfValueTypeNames::UIntArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->UIntArray;
                    })
                }
                SdfValueTypeNames::Int64Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int64Array;
                    })
                }
                SdfValueTypeNames::UInt64Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->UInt64Array;
                    })
                }
                SdfValueTypeNames::HalfArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->HalfArray;
                    })
                }
                SdfValueTypeNames::FloatArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->FloatArray;
                    })
                }
                SdfValueTypeNames::DoubleArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->DoubleArray;
                    })
                }
                SdfValueTypeNames::TimeCodeArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TimeCodeArray;
                    })
                }
                SdfValueTypeNames::StringArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->StringArray;
                    })
                }
                SdfValueTypeNames::TokenArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TokenArray;
                    })
                }
                SdfValueTypeNames::AssetArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->AssetArray;
                    })
                }
                SdfValueTypeNames::Int2Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int2Array;
                    })
                }
                SdfValueTypeNames::Int3Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int3Array;
                    })
                }
                SdfValueTypeNames::Int4Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Int4Array;
                    })
                }
                SdfValueTypeNames::Half2Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half2Array;
                    })
                }
                SdfValueTypeNames::Half3Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half3Array;
                    })
                }
                SdfValueTypeNames::Half4Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Half4Array;
                    })
                }
                SdfValueTypeNames::Float2Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float2Array;
                    })
                }
                SdfValueTypeNames::Float3Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float3Array;
                    })
                }
                SdfValueTypeNames::Float4Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Float4Array;
                    })
                }
                SdfValueTypeNames::Double2Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double2Array;
                    })
                }
                SdfValueTypeNames::Double3Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double3Array;
                    })
                }
                SdfValueTypeNames::Double4Array => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Double4Array;
                    })
                }
                SdfValueTypeNames::Point3hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Point3hArray;
                    })
                }
                SdfValueTypeNames::Point3fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Point3fArray;
                    })
                }
                SdfValueTypeNames::Point3dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Point3dArray;
                    })
                }
                SdfValueTypeNames::Vector3hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Vector3hArray;
                    })
                }
                SdfValueTypeNames::Vector3fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Vector3fArray;
                    })
                }
                SdfValueTypeNames::Vector3dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Vector3dArray;
                    })
                }
                SdfValueTypeNames::Normal3hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Normal3hArray;
                    })
                }
                SdfValueTypeNames::Normal3fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Normal3fArray;
                    })
                }
                SdfValueTypeNames::Normal3dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Normal3dArray;
                    })
                }
                SdfValueTypeNames::Color3hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color3hArray;
                    })
                }
                SdfValueTypeNames::Color3fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color3fArray;
                    })
                }
                SdfValueTypeNames::Color3dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color3dArray;
                    })
                }
                SdfValueTypeNames::Color4hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color4hArray;
                    })
                }
                SdfValueTypeNames::Color4fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color4fArray;
                    })
                }
                SdfValueTypeNames::Color4dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Color4dArray;
                    })
                }
                SdfValueTypeNames::QuathArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->QuathArray;
                    })
                }
                SdfValueTypeNames::QuatfArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->QuatfArray;
                    })
                }
                SdfValueTypeNames::QuatdArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->QuatdArray;
                    })
                }
                SdfValueTypeNames::Matrix2dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Matrix2dArray;
                    })
                }
                SdfValueTypeNames::Matrix3dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Matrix3dArray;
                    })
                }
                SdfValueTypeNames::Matrix4dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Matrix4dArray;
                    })
                }
                SdfValueTypeNames::Frame4dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->Frame4dArray;
                    })
                }
                SdfValueTypeNames::TexCoord2hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord2hArray;
                    })
                }
                SdfValueTypeNames::TexCoord2fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord2fArray;
                    })
                }
                SdfValueTypeNames::TexCoord2dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord2dArray;
                    })
                }
                SdfValueTypeNames::TexCoord3hArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord3hArray;
                    })
                }
                SdfValueTypeNames::TexCoord3fArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord3fArray;
                    })
                }
                SdfValueTypeNames::TexCoord3dArray => {
                    cpp!([] -> pxr::SdfValueTypeName as "pxr::SdfValueTypeName" {
                        return pxr::SdfValueTypeNames->TexCoord3dArray;
                    })
                }
            }
        })
    }
}
