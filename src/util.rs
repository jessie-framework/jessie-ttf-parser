use core::ops::Range;

use crate::endian::*;
use crate::impl_max;
use crate::impl_max_be;
use crate::impl_maxes;
use crate::impl_maxes_be;

#[inline]
pub(crate) const fn slice_rest(slice: &[u8], offset: usize) -> &[u8] {
    let len = slice.len() - offset;
    unsafe { core::slice::from_raw_parts(slice.as_ptr().add(offset), len) }
}

#[inline]
pub(crate) const fn slice_range(slice: &[u8], range: Range<usize>) -> &[u8] {
    let len = range.end - range.start;
    unsafe { core::slice::from_raw_parts(slice.as_ptr().add(range.start), len) }
}

#[inline]
pub(crate) const fn slice_get<T>(slice: &[T], idx: usize) -> Option<&T> {
    if idx < slice.len() {
        return Some(&slice[idx]);
    }
    None
}

#[inline]
pub(crate) const fn slice_first<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    Some(&slice[0])
}

#[inline]
pub(crate) const fn slice_last<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    Some(&slice[slice.len() - 1])
}

pub(crate) struct SliceIter<'a, T> {
    slice: &'a [T],
    idx: usize,
}

impl<'a, T> SliceIter<'a, T> {
    pub(crate) const fn new(slice: &'a [T]) -> Self {
        Self { slice, idx: 0 }
    }

    pub(crate) const fn next(&mut self) -> Option<&'a T> {
        self.idx += 1;
        Some(&self.slice[self.idx - 1])
    }
}
impl_maxes!(
    usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8
);
impl_maxes_be!(
    (U32BE, u32, into_u32),
    (U24BE, u32, into_u32),
    (U16BE, u16, into_u16),
    (I16BE, i16, into_i16)
);

#[macro_export]
macro_rules! impl_max {
    ($ty : ident) => {
        impl<'a> SliceIter<'a, $ty> {
            #[allow(unused)]
            pub(crate) const fn max(&mut self) -> Option<$ty> {
                let mut max = None;
                while let Some(v) = self.next() {
                    if max.is_none() {
                        max = Some(*v);
                    } else {
                        if *v > max.unwrap() {
                            max = Some(*v);
                        }
                    }
                }
                max
            }
        }
    };
}

#[macro_export]
macro_rules! impl_max_be {
    ($ty_be : ident,$ty_ne : ident, $fn : ident) => {
        impl<'a> SliceIter<'a, $ty_be> {
            #[allow(unused)]
            pub(crate) const fn max(&mut self) -> Option<$ty_ne> {
                let mut max = None;
                while let Some(v) = self.next() {
                    let val = v.$fn();
                    if max.is_none() {
                        max = Some(val);
                    } else {
                        if val > max.unwrap() {
                            max = Some(val);
                        }
                    }
                }
                max
            }
        }
    };
}

#[macro_export]
macro_rules! impl_maxes {
    ($($vals : ident),* ) => {
        $(impl_max!($vals);)*
    };
}

#[macro_export]
macro_rules! impl_maxes_be {
    ($(($ty_be : ident, $ty_ne : ident , $fn : ident)),*) => {
        $(impl_max_be!($ty_be  , $ty_ne, $fn);)*
    };
}

#[macro_export]
macro_rules! att {
    ($inner : expr) => {
        match $inner {
            Some(v) => v,
            _ => return None,
        }
    };
}
