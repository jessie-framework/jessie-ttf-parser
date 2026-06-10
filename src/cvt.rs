use crate::{
    endian::FWordBE,
    parser::{Parser, TableRecord},
    stream::Stream,
    util::slice_range,
};

#[derive(Debug, Clone)]
pub struct CvtTable<'a>(
    /// This version makes it possible to create a font that is not burdened with a large set of glyph names. A version 3.0 'post' table can be used by OpenType fonts with TrueType or CFF (version 1 or 2) data.
    ///
    /// This version specifies that no PostScript name information is provided for the glyphs in this font file. The printing behavior of this version on PostScript printers is unspecified, except that it should not result in a fatal or unrecoverable error. Some drivers may print nothing; other drivers may attempt to print using a default naming scheme.
    pub &'a [FWordBE],
);

pub(crate) struct CvtParser<'a> {
    stream: Stream<'a>,
}

impl<'a> CvtParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<CvtTable<'a>> {
        let inner = self.stream.parse_slice_rest();
        Some(CvtTable(inner))
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_cvt(&self, input: TableRecord) -> Option<CvtTable<'a>> {
        if input.table_tag.is_cvt() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = CvtParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
