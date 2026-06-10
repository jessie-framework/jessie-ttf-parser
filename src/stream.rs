use crate::{f2dot14::F2Dot14, tag::Tag};

pub(crate) struct Stream<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) idx: usize,
}

impl<'a> Stream<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, idx: 0 }
    }

    #[inline]
    pub(crate) const fn parse_tag(&mut self) -> Option<Tag> {
        self.idx += 4;
        if self.idx < self.bytes.len() {
            let data: [u8; 4] =
                unsafe { core::ptr::read(&raw const self.bytes[self.idx - 4] as *const _) };
            return Some(Tag(data));
        }
        None
    }

    #[inline]
    pub(crate) const fn parse_u8(&mut self) -> Option<u8> {
        self.idx += 1;
        if self.idx < self.bytes.len() {
            return Some(self.bytes[self.idx - 1]);
        }
        None
    }

    #[inline]
    pub(crate) const fn parse_i8(&mut self) -> Option<i8> {
        self.idx += 1;
        if self.idx < self.bytes.len() {
            return Some(self.bytes[self.idx - 1] as i8);
        }
        None
    }

    pub(crate) const fn parse_u16(&mut self) -> Option<u16> {
        self.idx += 2;
        if self.idx < self.bytes.len() {
            let data: [u8; 2] =
                unsafe { core::ptr::read(&raw const self.bytes[self.idx - 2] as *const _) };
            return Some(u16::from_be_bytes(data));
        }
        None
    }

    pub(crate) const fn parse_i16(&mut self) -> Option<i16> {
        self.idx += 2;
        if self.idx < self.bytes.len() {
            let data: [u8; 2] =
                unsafe { core::ptr::read(&raw const self.bytes[self.idx - 2] as *const _) };
            return Some(i16::from_be_bytes(data));
        }
        None
    }

    pub(crate) const fn parse_f2_dot_14(&mut self) -> Option<F2Dot14> {
        match self.parse_u16() {
            Some(v) => Some(F2Dot14::from_u16(v)),
            None => None,
        }
    }

    pub(crate) const fn parse_i64(&mut self) -> Option<i64> {
        self.idx += 8;
        if self.idx < self.bytes.len() {
            let data: [u8; 8] =
                unsafe { core::ptr::read(&raw const self.bytes[self.idx - 8] as *const _) };
            return Some(i64::from_be_bytes(data));
        }
        None
    }

    pub(crate) const fn parse_u32(&mut self) -> Option<u32> {
        self.idx += 4;
        if self.idx < self.bytes.len() {
            let data: [u8; 4] =
                unsafe { core::ptr::read(&raw const self.bytes[self.idx - 4] as *const _) };
            return Some(u32::from_be_bytes(data));
        }
        None
    }

    pub(crate) const fn parse_i32(&mut self) -> Option<i32> {
        self.idx += 4;
        if self.idx < self.bytes.len() {
            let data: [u8; 4] =
                unsafe { core::ptr::read(&raw const self.bytes[self.idx - 4] as *const _) };
            return Some(i32::from_be_bytes(data));
        }
        None
    }

    pub(crate) const fn parse_slice<T>(&mut self, length: usize) -> Option<&'a [T]> {
        let ptr = &raw const self.bytes[self.idx] as *const T;
        let out = unsafe { core::slice::from_raw_parts(ptr, length) };
        let size = size_of::<T>() * length;
        if self.idx + size <= self.bytes.len() {
            self.idx += size;
            Some(out)
        } else {
            None
        }
    }

    pub(crate) const fn parse_utf8(&mut self, length: usize) -> Option<&'a str> {
        let ptr = unsafe { self.bytes.as_ptr().add(self.idx) };
        let out =
            unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, length)) };
        if self.idx + length <= self.bytes.len() {
            self.idx += length;
            Some(out)
        } else {
            None
        }
    }

    pub(crate) const fn parse_slice_rest<T>(&mut self) -> &'a [T] {
        let ptr = &raw const self.bytes[self.idx] as *const T;
        let len = (self.bytes.len() - self.idx) / size_of::<T>();
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}
