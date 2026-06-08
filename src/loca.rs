use crate::{
    endian::{U16BE, U32BE},
    head::HeadTable,
    maxp::MaxpTable,
    parser::{Parser, TableRecord},
    stream::Stream,
};

#[derive(Debug, Clone)]
pub enum LocaTable<'a> {
    ShortFormat(LocaShortFormat<'a>),
    LongFormat(LocaLongFormat<'a>),
}

impl<'a> LocaTable<'a> {
    pub fn get_glyphid(&self, input: GlyphId) -> Option<LocaOffset> {
        match self {
            LocaTable::ShortFormat(t) => {
                let start = t.offsets.get(input.0 as usize)?.into_u16() as u32 * 2;
                let end = t.offsets.get(input.0 as usize + 1)?.into_u16() as u32 * 2;
                Some(LocaOffset { start, end })
            }
            LocaTable::LongFormat(t) => {
                let start = t.offsets.get(input.0 as usize)?.into_u32();
                let end = t.offsets.get(input.0 as usize + 1)?.into_u32();
                Some(LocaOffset { start, end })
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct LocaOffset {
    pub(crate) start: u32,
    pub(crate) end: u32,
}

impl LocaOffset {
    pub fn has_no_outline_or_instructions(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone)]
pub struct LocaShortFormat<'a> {
    /// The local offset divided by 2 is stored.
    pub offsets: &'a [U16BE],
}

#[derive(Debug, Clone)]
pub struct LocaLongFormat<'a> {
    /// The actual local offset is stored.
    pub offsets: &'a [U32BE],
}

pub(crate) struct LocaParser<'a> {
    stream: Stream<'a>,
}

impl<'a> LocaParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self, maxp: &MaxpTable, head: &HeadTable) -> Option<LocaTable<'a>> {
        let format = head.index_to_loc_format;
        let num_glyphs = maxp.num_glyphs();
        match format {
            0 => {
                let offsets = self.stream.parse_slice(num_glyphs as usize + 1)?;
                Some(LocaTable::ShortFormat(LocaShortFormat { offsets }))
            }
            1 => {
                let offsets = self.stream.parse_slice(num_glyphs as usize + 1)?;
                Some(LocaTable::LongFormat(LocaLongFormat { offsets }))
            }
            _ => None,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_loca(
        &self,
        input: TableRecord,
        maxp: &MaxpTable,
        head: &HeadTable,
    ) -> Option<LocaTable<'a>> {
        if input.table_tag.is_loca() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = LocaParser::new(bytes);
            return parser.parse(maxp, head);
        }
        None
    }
}
