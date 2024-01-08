use crate::pxr;
use cpp::cpp;

pub struct UsdPrimRangeIterator {
    pub(crate) it: pxr::UsdPrimRange_iterator,
    pub(crate) end: pxr::UsdPrimRange_iterator,
}

impl UsdPrimRangeIterator {
    pub(crate) fn new(begin: pxr::UsdPrimRange_iterator, end: pxr::UsdPrimRange_iterator) -> Self {
        Self { it: begin, end }
    }
}

impl std::iter::Iterator for UsdPrimRangeIterator {
    type Item = pxr::UsdPrim;
    fn next(&mut self) -> Option<Self::Item> {
        if self.it.eq(&self.end) {
            None
        } else {
            let result = self.it._dereference();
            self.it._increment();
            Some(result)
        }
    }
}

impl pxr::UsdPrimRange {
    #[must_use]
    pub fn iter(&self) -> UsdPrimRangeIterator {
        let begin = unsafe {
            cpp!([self as "pxr::UsdPrimRange *"] -> pxr::UsdPrimRange_iterator as "pxr::UsdPrimRange::iterator" {
                return self->begin();
            })
        };
        let end = unsafe {
            cpp!([self as "pxr::UsdPrimRange *"] -> pxr::UsdPrimRange_iterator as "pxr::UsdPrimRange::iterator" {
                return self->end();
            })
        };
        UsdPrimRangeIterator::new(begin, end)
    }
}

impl pxr::UsdPrimRange_iterator {
    pub fn _increment(&mut self) {
        unsafe {
            cpp!([self as "pxr::UsdPrimRange::iterator *"] {
                ++(*self);
            });
        }
    }

    #[must_use]
    pub fn _dereference(&self) -> pxr::UsdPrim {
        unsafe {
            cpp!([self as "const pxr::UsdPrimRange::iterator *"]
                -> pxr::UsdPrim as "pxr::UsdPrim" {
                return *(*self);
            })
        }
    }
}

impl std::cmp::PartialEq for pxr::UsdPrimRange_iterator {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdPrimRange::iterator *",
                  rhs as "const pxr::UsdPrimRange::iterator *"]
                -> bool as "bool" {
                return (*self) == (*rhs);
            })
        }
    }
}
