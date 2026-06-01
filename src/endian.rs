#[cfg(target_endian = "little")]
use std::char;
use std::fmt::{Debug, Display};

use crate::f2dot14::F2Dot14;

pub type FWordBE = I16BE;
pub type UFWordBE = U16BE;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct U32BE(u32);

impl U32BE {
    #[cfg(target_endian = "big")]
    pub fn into_u32(self) -> u32 {
        self.0
    }

    #[cfg(target_endian = "little")]
    pub fn into_u32(self) -> u32 {
        self.0.to_be()
    }
}

impl Display for U32BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_u32())
    }
}

impl Debug for U32BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct U24BE(u16, u8);

impl U24BE {
    #[cfg(target_endian = "big")]
    pub fn into_u32(self) -> u32 {
        (self.0 as u32) & (self.1 as u32 >> 16)
    }

    #[cfg(target_endian = "little")]
    pub fn into_u32(self) -> u32 {
        (self.1.to_be() as u32) & (self.0.to_be() as u32 >> 8)
    }
}

impl Display for U24BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_u32())
    }
}

impl Debug for U24BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct U16BE(pub(crate) u16);

impl U16BE {
    #[cfg(target_endian = "big")]
    pub fn into_u16(self) -> u16 {
        self.0
    }

    #[cfg(target_endian = "little")]
    pub fn into_u16(self) -> u16 {
        self.0.to_be()
    }
}

impl Display for U16BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_u16())
    }
}
impl Debug for U16BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct I16BE(i16);

impl I16BE {
    #[cfg(target_endian = "big")]
    pub fn into_i16(self) -> i16 {
        self.0
    }

    #[cfg(target_endian = "little")]
    pub fn into_i16(self) -> i16 {
        self.0.to_be()
    }
}

impl Display for I16BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_i16())
    }
}
impl Debug for I16BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct UTF16BE<'a> {
    bytes: &'a [u8],
}

impl<'a> UTF16BE<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    #[cfg(target_endian = "little")]
    pub fn chars(&'a self) -> impl Iterator<Item = Result<char, char::DecodeUtf16Error>> {
        let (chunks, _) = self.bytes.as_chunks::<2>();
        let iter = chunks.iter().copied().map(u16::from_be_bytes);
        char::decode_utf16(iter)
    }

    #[cfg(target_endian = "big")]
    pub fn chars(&'a self) -> impl Iterator<Item = Result<char, char::DecodeUtf16Error>> {
        let (chunks, _) = self.bytes.as_chunks::<2>();
        let iter = chunks.iter().copied().map(u16::from_ne_bytes);
        char::decode_utf16(iter)
    }
}

impl<'a> Display for UTF16BE<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.chars();
        for i in chars.flatten() {
            write!(f, "{i}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct F2Dot14BE(U16BE);

impl F2Dot14BE {
    pub fn into_f32(self) -> f32 {
        F2Dot14::from_u16(self.0.into_u16()).into_f32()
    }
}

impl Display for F2Dot14BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_f32())
    }
}

impl Debug for F2Dot14BE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
