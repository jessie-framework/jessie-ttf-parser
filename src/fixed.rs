use std::fmt::{Debug, Display};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Fixed {
    pub num: u16,
    pub frac: u16,
}

impl Fixed {
    pub fn from_u32(input: u32) -> Self {
        let num = (input << 16) as u16;
        let frac = (input >> 16) as u16;
        Fixed { num, frac }
    }
}

impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.num, self.frac)
    }
}

impl Debug for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
