use crate::{
    att,
    endian::{I16BE, U16BE},
    parser::{Parser, TableRecord},
    stream::Stream,
    util::slice_range,
};

#[repr(C)]
#[derive(Debug, Clone)]
/// This optional table specifies the y coordinate of the vertical origin of every glyph in the font.
///
/// This table may only be used in CFF or CFF2 OpenType fonts. If present in OpenType fonts containing TrueType outline data, it must be ignored: the Vertical Metrics ('vmtx') and Glyph Data ('glyf') tables provide all the information necessary to accurately calculate the y coordinate of a glyph’s vertical origin. See the “Vertical Origin and Advance Height” section in the 'vmtx' table specification for more details.
pub struct VorgTable<'a> {
    /// Major version—set to 1.
    pub major_version: u16,
    /// Minor version—set to 0.
    pub minor_version: u16,
    /// The y coordinate of a glyph’s vertical origin, in the font’s design coordinate system, to be used if no entry is present for the glyph in the vertOriginYMetrics array.
    pub default_vert_origin_y: i16,
    /// Number of elements in the vertOriginYMetrics array.
    pub num_vert_origin_y_metrics: u16,
    /// Array of VertOriginYMetrics records, sorted by glyph ID.
    pub vert_origin_y_metrics: &'a [VertOriginYMetrics],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct VertOriginYMetrics {
    /// Glyph index.
    pub glyph_index: U16BE,
    /// Y coordinate, in the font’s design coordinate system, of the glyph’s vertical origin.
    pub vert_origin_y: I16BE,
}

pub(crate) struct VorgParser<'a> {
    stream: Stream<'a>,
}

impl<'a> VorgParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<VorgTable<'a>> {
        let major_version = att!(self.stream.parse_u16());
        let minor_version = att!(self.stream.parse_u16());
        let default_vert_origin_y = att!(self.stream.parse_i16());
        let num_vert_origin_y_metrics = att!(self.stream.parse_u16());
        let vert_origin_y_metrics =
            att!(self.stream.parse_slice(num_vert_origin_y_metrics as usize));
        Some(VorgTable {
            major_version,
            minor_version,
            default_vert_origin_y,
            num_vert_origin_y_metrics,
            vert_origin_y_metrics,
        })
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_vorg(&self, input: TableRecord) -> Option<VorgTable<'a>> {
        if input.table_tag.is_vorg() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = VorgParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
