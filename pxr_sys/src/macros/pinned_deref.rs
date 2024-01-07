/// Implement `as_deref()` and `map_deref()` for a type that is a C++ pointer to a known C++ type
/// The `source_type` must implement `Deref` to the `target_type`
macro_rules! impl_pinned_deref {
    ($source_type:ty => $target_type:ty) => {
        #[automatically_derived]
        impl $source_type {
            #[inline(always)]
            #[must_use] pub fn as_deref<'a>(
                self: &'a ::std::pin::Pin<Box<Self>>,
            ) -> ::std::pin::Pin<&'a $target_type> {
                self.as_ref().map_deref()
            }

            #[inline(always)]
            #[must_use] pub fn map_deref<'a>(
                self: ::std::pin::Pin<&'a Self>,
            ) -> ::std::pin::Pin<&'a $target_type> {
                unsafe { self.map_unchecked(|x| x.deref()) }
            }
        }
    };
    [$($source_type:ty => $target_type:ty),* $(,)?] => {
        $($crate::macros::impl_pinned_deref!($source_type => $target_type);)*
    };
}
pub(crate) use impl_pinned_deref;

/// Implement `as_deref_mut()` and `map_deref_mut()` for a type that is a C++ pointer to a known C++ type
/// The `source_type` must implement `DerefMut` to the `target_type`
macro_rules! impl_pinned_deref_mut {
    ($source_type:ty => $target_type:ty) => {
        $crate::macros::impl_pinned_deref!($source_type => $target_type);
        #[automatically_derived]
        impl $source_type {
            #[inline(always)]
            pub fn as_deref_mut<'a>(
                self: &'a mut ::std::pin::Pin<Box<Self>>,
            ) -> ::std::pin::Pin<&'a mut $target_type> {
                self.as_mut().map_deref_mut()
            }

            #[inline(always)]
            #[must_use] pub fn map_deref_mut<'a>(
                self: ::std::pin::Pin<&'a mut Self>,
            ) -> ::std::pin::Pin<&'a mut $target_type> {
                unsafe { self.map_unchecked_mut(|x| x.deref_mut()) }
            }
        }
    };
    [$($source_type:ty => $target_type:ty),* $(,)?] => {
        $($crate::macros::impl_pinned_deref_mut!($source_type => $target_type);)*
    };
}
pub(crate) use impl_pinned_deref_mut;
