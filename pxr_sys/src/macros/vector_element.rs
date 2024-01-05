/// Implement VectorElement for a type (this should be done automatically by autocxx but it doesn't work for certain types - no idea why)
macro_rules! impl_vector_element {
    ([$kind:ident] $segment:expr, $name:expr, $ty:ty) => {
        cxx::const_assert_eq!(0, std::mem::size_of::<cxx::CxxVector<$ty>>());
        cxx::const_assert_eq!(1, std::mem::align_of::<cxx::CxxVector<$ty>>());

        unsafe impl cxx::private::VectorElement for $ty {
            fn __typename(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str($name)
            }
            fn __vector_new() -> *mut cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$new")]
                    fn __vector_new() -> *mut cxx::CxxVector<$ty>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &cxx::CxxVector<$ty>) -> usize {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$size")]
                    fn __vector_size(_: &cxx::CxxVector<$ty>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut cxx::CxxVector<$ty>, pos: usize) -> *mut $ty {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$get_unchecked")]
                    fn __get_unchecked(_: *mut cxx::CxxVector<$ty>, _: usize) -> *mut $ty;
                }
                unsafe { __get_unchecked(v, pos) }
            }
            fn __unique_ptr_null() -> std::mem::MaybeUninit<*mut std::ffi::c_void> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$null")]
                    fn __unique_ptr_null(this: *mut std::mem::MaybeUninit<*mut std::ffi::c_void>);
                }
                let mut repr = std::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(raw: *mut cxx::CxxVector<Self>) -> std::mem::MaybeUninit<*mut std::ffi::c_void> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$raw")]
                    fn __unique_ptr_raw(this: *mut std::mem::MaybeUninit<*mut std::ffi::c_void>, raw: *mut cxx::CxxVector<$ty>);
                }
                let mut repr = std::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_raw(&mut repr, raw) }
                repr
            }
            unsafe fn __unique_ptr_get(repr: std::mem::MaybeUninit<*mut std::ffi::c_void>) -> *const cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$get")]
                    fn __unique_ptr_get(this: *const std::mem::MaybeUninit<*mut std::ffi::c_void>) -> *const cxx::CxxVector<$ty>;
                }
                unsafe { __unique_ptr_get(&repr) }
            }
            unsafe fn __unique_ptr_release(mut repr: std::mem::MaybeUninit<*mut std::ffi::c_void>) -> *mut cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$release")]
                    fn __unique_ptr_release(this: *mut std::mem::MaybeUninit<*mut std::ffi::c_void>) -> *mut cxx::CxxVector<$ty>;
                }
                unsafe { __unique_ptr_release(&mut repr) }
            }
            unsafe fn __unique_ptr_drop(mut repr: std::mem::MaybeUninit<*mut std::ffi::c_void>) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$drop")]
                    fn __unique_ptr_drop(this: *mut std::mem::MaybeUninit<*mut std::ffi::c_void>);
                }
                unsafe { __unique_ptr_drop(&mut repr) }
            }
            $crate::macros::impl_vector_element!(@inner [$kind] $segment, $ty);
        }
    };
    (@inner [Opaque] $segment:expr, $ty:ty) => {};
    (@inner [Trivial] $segment:expr, $ty:ty) => {
        unsafe fn __push_back(
            v: std::pin::Pin<&mut cxx::CxxVector<$ty>>,
            value: &mut std::mem::ManuallyDrop<$ty>,
        ) {
            extern "C" {
                #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$push_back")]
                fn __push_back(
                    _: std::pin::Pin<&mut cxx::CxxVector<$ty>>,
                    _: &mut std::mem::ManuallyDrop<$ty>,
                );
            }
            unsafe { __push_back(v, value) }
        }
        unsafe fn __pop_back(
            v: std::pin::Pin<&mut cxx::CxxVector<$ty>>,
            out: &mut std::mem::MaybeUninit<$ty>,
        ) {
            extern "C" {
                #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$pop_back")]
                fn __pop_back(
                    _: std::pin::Pin<&mut cxx::CxxVector<$ty>>,
                    _: &mut std::mem::MaybeUninit<$ty>,
                );
            }
            unsafe { __pop_back(v, out) }
        }
    };
}
pub(crate) use impl_vector_element;
