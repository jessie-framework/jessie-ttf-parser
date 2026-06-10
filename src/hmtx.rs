use crate::{
    endian::{FWordBE, UFWordBE},
    fword::FWord,
    hhea::HheaTable,
    maxp::MaxpTable,
    parser::TableRecord,
    stream::Stream,
    util::slice_range,
};

#[repr(C)]
pub struct HmtxHeader<'a> {
    /// Paired advance width and left side bearing values for each glyph. Records are indexed by glyph ID.
    pub h_metrics: &'a [LongHorMetric],
    /// Left side bearings for glyph IDs greater than or equal to numberOfHMetrics.
    pub left_side_bearings: &'a [FWord],
}

#[repr(C)]
pub struct LongHorMetric {
    pub advance_width: UFWordBE,
    pub lsb: FWordBE,
}

pub(crate) struct HmtxParser<'a> {
    stream: Stream<'a>,
}

impl<'a> HmtxParser<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(
        &mut self,
        hhea: &HheaTable,
        maxp: &MaxpTable,
    ) -> Option<HmtxHeader<'a>> {
        let number_of_h_metrics = hhea.number_of_h_metrics;
        let num_glyphs = maxp.num_glyphs();
        let h_metrics = match self.stream.parse_slice(number_of_h_metrics as usize) {
            Some(v) => v,
            _ => return None,
        };
        let left_side_bearings = match self
            .stream
            .parse_slice(num_glyphs as usize - number_of_h_metrics as usize)
        {
            Some(v) => v,
            _ => return None,
        };
        Some(HmtxHeader {
            h_metrics,
            left_side_bearings,
        })
    }
}

impl<'a> crate::parser::Parser<'a> {
    pub const fn parse_hmtx(
        &self,
        input: TableRecord,
        hhea: &HheaTable,
        maxp: &MaxpTable,
    ) -> Option<HmtxHeader<'a>> {
        if input.table_tag.is_hmtx() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = HmtxParser::new(bytes);
            return parser.parse(hhea, maxp);
        }
        None
    }
}
