/// Declare external types for `ffi_autocxx` module (for types that autocxx cannot generate as of now)
macro_rules! impl_extern_type {
    ($([$kind:ident] $($(#[$($attr:tt)*])* $ty:path = $cxxpath:literal)*)*) => {
        $($(
            $(#[$($attr)*])*
            unsafe impl cxx::ExternType for $ty {
                #[allow(unused_attributes)]
                #[doc(hidden)]
                type Id = cxx::type_id!($cxxpath);
                type Kind = cxx::kind::$kind;
            }
        )*)*
    };
}
pub(crate) use impl_extern_type;
