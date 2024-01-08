use crate::pxr;
use cpp::cpp;

impl std::cmp::PartialEq for pxr::TfToken {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            cpp!([self as "const pxr::TfToken *", other as "const pxr::TfToken *"] -> bool as "bool" {
                return *self == *other;
            })
        }
    }
}

impl std::fmt::Display for pxr::TfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.GetString())
    }
}
