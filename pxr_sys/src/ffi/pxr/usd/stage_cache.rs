use crate::pxr;
use cpp::cpp;

impl pxr::UsdStageCache_Id {
    pub fn IsValid(&self) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdStageCache::Id *"] -> bool as "bool" {
                return self->IsValid();
            })
        }
    }
}

impl std::cmp::PartialEq for pxr::UsdStageCache_Id {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdStageCache::Id *", other as "const pxr::UsdStageCache::Id *"] -> bool as "bool" {
                return *self == *other;
            })
        }
    }
}

// pxr::UsdStageCache_Id <-> i64
impl From<&pxr::UsdStageCache_Id> for i64 {
    fn from(value: &pxr::UsdStageCache_Id) -> Self {
        unsafe {
            cpp!([value as "const pxr::UsdStageCache::Id *"] -> i64 as "int64_t" {
                return value->ToLongInt();
            })
        }
    }
}
impl From<i64> for pxr::UsdStageCache_Id {
    fn from(value: i64) -> Self {
        unsafe {
            cpp!([value as "int64_t"] -> pxr::UsdStageCache_Id as "pxr::UsdStageCache::Id" {
                return pxr::UsdStageCache::Id::FromLongInt(value);
            })
        }
    }
}

// std::ffi::{&CStr, CString} -> pxr::UsdStageCache_Id | pxr::UsdStageCache_Id -> std::ffi::CString
impl From<&std::ffi::CStr> for pxr::UsdStageCache_Id {
    fn from(value: &std::ffi::CStr) -> Self {
        let value_ptr = value.as_ptr();
        unsafe {
            cpp!([value_ptr as "const char *"] -> pxr::UsdStageCache_Id as "pxr::UsdStageCache::Id" {
                return pxr::UsdStageCache::Id::FromString(std::string(value_ptr));
            })
        }
    }
}
impl From<std::ffi::CString> for pxr::UsdStageCache_Id {
    fn from(value: std::ffi::CString) -> Self {
        value.as_c_str().into()
    }
}
impl From<pxr::UsdStageCache_Id> for std::ffi::CString {
    fn from(value: pxr::UsdStageCache_Id) -> Self {
        unsafe {
            std::ffi::CString::from_raw(
                cpp!([value as "pxr::UsdStageCache::Id"] -> *mut std::ffi::c_char as "const char *" {
                    return value.ToString().c_str();
                }),
            )
        }
    }
}

// pxr::UsdStageCache_Id as std::ffi::CStr
impl AsRef<std::ffi::CStr> for pxr::UsdStageCache_Id {
    fn as_ref(&self) -> &std::ffi::CStr {
        unsafe {
            std::ffi::CStr::from_ptr(
                cpp!([self as "const pxr::UsdStageCache::Id *"] -> *const std::ffi::c_char as "const char *" {
                    return self->ToString().c_str();
                }),
            )
        }
    }
}

// &str -> pxr::UsdStageCache_Id
impl TryFrom<&str> for pxr::UsdStageCache_Id {
    type Error = std::ffi::NulError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(std::ffi::CString::new(value)?.into())
    }
}

// Formatting
impl std::fmt::Debug for pxr::UsdStageCache_Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pxr::UsdStageCache::Id [{:?}]",
            std::ffi::CString::from(self.as_ref()).into_string()
        )
    }
}
impl std::fmt::Display for pxr::UsdStageCache_Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::ffi::CString::from(self.as_ref())
                .into_string()
                .unwrap()
        )
    }
}
