// Rust str, std::string::String, std::ffi::{CStr, CString} as C++ pxr::VtValue<std::string>
impl From<&std::ffi::CStr> for crate::pxr::VtValue {
    fn from(value: &std::ffi::CStr) -> Self {
        let value_ptr = value.as_ptr();
        unsafe {
            ::cpp::cpp!([value_ptr as "const char *"] -> crate::pxr::VtValue as "pxr::VtValue" {
                return pxr::VtValue(std::string(value_ptr));
            })
        }
    }
}
impl From<&str> for crate::pxr::VtValue {
    fn from(value: &str) -> Self {
        std::ffi::CString::new(value).unwrap().as_c_str().into()
    }
}
impl TryFrom<&crate::pxr::VtValue> for &std::ffi::CStr {
    type Error = String;
    fn try_from(value: &crate::pxr::VtValue) -> Result<Self, Self::Error> {
        if value.GetTypeName().to_string() == "string" {
            Ok(unsafe {
                std::ffi::CStr::from_ptr(
                    ::cpp::cpp!([value as "const pxr::VtValue *"] -> *const std::ffi::c_char as "const char *" {
                        return value->Get<std::string>().c_str();
                    }),
                )
            })
        } else {
            Err(format!(
                "Cannot cast C++ type pxr::VtValue<{}> to Rust type std::ffi::CStr.",
                value.GetTypeName()
            ))
        }
    }
}
impl TryFrom<&crate::pxr::VtValue> for &str {
    type Error = String;
    fn try_from(value: &crate::pxr::VtValue) -> Result<Self, Self::Error> {
        TryInto::<&std::ffi::CStr>::try_into(value).map(|cstr| cstr.to_str().unwrap())
    }
}
impl AsRef<std::ffi::CStr> for crate::pxr::VtValue {
    fn as_ref(&self) -> &std::ffi::CStr {
        unsafe {
            std::ffi::CStr::from_ptr(
                ::cpp::cpp!([self as "const pxr::VtValue *"] -> *const std::ffi::c_char as "const char *" {
                    return self->Get<std::string>().c_str();
                }),
            )
        }
    }
}
impl AsRef<str> for crate::pxr::VtValue {
    fn as_ref(&self) -> &str {
        AsRef::<std::ffi::CStr>::as_ref(self).to_str().unwrap()
    }
}
