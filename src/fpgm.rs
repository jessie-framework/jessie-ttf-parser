use crate::{
    parser::{Parser, TableRecord},
    stream::Stream,
};

#[derive(Debug, Clone)]
pub struct FpgmTable<'a>(
    /// Instructions. n is the number of uint8 items that fit in the size of the table.
    pub &'a [u8],
);

pub(crate) struct FpgmParser<'a> {
    stream: Stream<'a>,
}

impl<'a> FpgmParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<FpgmTable<'a>> {
        let inner = self.stream.parse_slice_rest();
        Some(FpgmTable(inner))
    }
}

impl<'a> Parser<'a> {
    pub fn parse_fpgm(&self, input: TableRecord) -> Option<FpgmTable<'a>> {
        if input.table_tag.is_fpgm() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = FpgmParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
