use crate::parser::{Parser, TableRecord};

/// The Control Value (CV) Program consists of a set of TrueType instructions that can be used to make font-wide changes in the Control Value Table. Any instruction is valid in the CV Program but since no glyph is associated with it, instructions intended to move points within a particular glyph outline have no effect in the CV Program.
///
/// The CV Program can potentially be executed before any glyph is processed, as determined by the rasterizer implementation. The CV Program will always be executed before the first glyph is processed after a change in font, point size, global transformation matrix, or other values that can be obtained from the GET INFOrmation instruction. The CV Program is not re-executed for components in a composite glyph, including cases in which a composite glyph description applies a transform to a component.
pub struct PrepTable<'a> {
    /// Set of instructions executed whenever point size or font or transformation change. n is the number of uint8 items that fit in the size of the table.
    pub data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn parse_prep(&self, input: TableRecord) -> Option<PrepTable<'a>> {
        if input.table_tag.is_prep() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            return Some(PrepTable { data: bytes });
        }
        None
    }
}
