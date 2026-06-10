#[derive(Debug, Clone, Copy, Default)]
/// 16-bit signed fixed number with the low 14 bits of fraction (2.14).
pub struct F2Dot14(f32);

impl F2Dot14 {
    #[inline]
    pub const fn default() -> Self {
        Self(0.)
    }

    #[inline]
    pub const fn from_f32(input: f32) -> Self {
        Self(input)
    }

    #[inline]
    pub const fn from_u16(input: u16) -> Self {
        let inner = (input as f32) / (1 << 14) as f32;
        Self(inner)
    }

    #[inline]
    pub const fn into_f32(self) -> f32 {
        self.0
    }
}
