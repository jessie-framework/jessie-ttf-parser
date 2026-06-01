use crate::{f2dot14::F2Dot14, parser::Tag};

pub(crate) struct Stream<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) idx: usize,
}

impl<'a> Stream<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, idx: 0 }
    }

    pub(crate) fn parse_tag(&mut self) -> Option<Tag> {
        self.idx += 4;
        let bytes = self.bytes.get(self.idx - 4..self.idx);
        if let Some(v) = bytes {
            let bytes: [u8; 4] = v.try_into().ok()?;
            return Some(Tag(bytes));
        }
        None
    }

    pub(crate) fn parse_u8(&mut self) -> Option<u8> {
        self.idx += 1;
        self.bytes.get(self.idx - 1).copied()
    }

    pub(crate) fn parse_i8(&mut self) -> Option<i8> {
        self.idx += 1;
        self.bytes
            .get(self.idx - 1)
            .copied()
            .map(|v| i8::from_ne_bytes([v]))
    }

    pub(crate) fn parse_u16(&mut self) -> Option<u16> {
        self.idx += 2;
        self.bytes
            .get(self.idx - 2..self.idx)
            .map(|v| u16::from_be_bytes(v.try_into().unwrap()))
    }

    pub(crate) fn parse_i16(&mut self) -> Option<i16> {
        self.idx += 2;
        self.bytes
            .get(self.idx - 2..self.idx)
            .map(|v| i16::from_be_bytes(v.try_into().unwrap()))
    }

    pub(crate) fn parse_f2_dot_14(&mut self) -> Option<F2Dot14> {
        self.parse_u16().map(F2Dot14::from_u16)
    }

    pub(crate) fn parse_i64(&mut self) -> Option<i64> {
        self.idx += 8;
        self.bytes
            .get(self.idx - 8..self.idx)
            .map(|v| i64::from_be_bytes(v.try_into().unwrap()))
    }

    pub(crate) fn parse_u32(&mut self) -> Option<u32> {
        self.idx += 4;
        self.bytes
            .get(self.idx - 4..self.idx)
            .map(|v| u32::from_be_bytes(v.try_into().unwrap()))
    }

    pub(crate) fn parse_i32(&mut self) -> Option<i32> {
        self.idx += 4;
        self.bytes
            .get(self.idx - 4..self.idx)
            .map(|v| i32::from_be_bytes(v.try_into().unwrap()))
    }

    pub(crate) fn parse_slice<T>(&mut self, length: usize) -> Option<&'a [T]> {
        let ptr = unsafe { self.bytes.as_ptr().add(self.idx) } as *const T;
        let out = unsafe { core::slice::from_raw_parts(ptr, length) };
        let size = size_of::<T>() * length;
        if self.idx + size <= self.bytes.len() {
            self.idx += size;
            Some(out)
        } else {
            None
        }
    }

    pub(crate) fn parse_utf8(&mut self, length: usize) -> Option<&'a str> {
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

    pub(crate) fn parse_slice_rest<T>(&mut self) -> &'a [T] {
        let ptr = &raw const self.bytes[self.idx] as *const T;
        let len = (self.bytes.len() - self.idx) / size_of::<T>();
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}
