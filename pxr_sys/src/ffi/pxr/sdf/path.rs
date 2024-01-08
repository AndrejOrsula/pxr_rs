use crate::pxr;
use cpp::cpp;

impl std::cmp::PartialEq for pxr::SdfPath {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            cpp!([self as "const pxr::SdfPath *", other as "const pxr::SdfPath *"] -> bool as "bool" {
                return *self == *other;
            })
        }
    }
}

impl std::fmt::Display for pxr::SdfPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.GetString())
    }
}
