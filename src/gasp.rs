use crate::{
    endian::U16BE,
    parser::{Parser, TableRecord},
    stream::Stream,
    util::slice_range,
};

/// This table contains information which describes the preferred rasterization techniques for the typeface when it is rendered on grayscale-capable devices. This table also has some use for monochrome devices, which may use the table to turn off hinting at very large or small sizes, to improve performance.
///
/// At very small sizes, the best appearance on grayscale devices can usually be achieved by rendering the glyphs in grayscale without using hints. At intermediate sizes, hinting and monochrome rendering will usually produce the best appearance. At large sizes, the combination of hinting and grayscale rendering will typically produce the best appearance.
///
/// If the 'gasp' table is not present in a typeface, the rasterizer may apply default rules to decide how to render the glyphs on grayscale devices.
///
/// Two versions of the 'gasp' table have been defined: both versions use the same formats, except that two new flags were defined in version 1, as documented below. All new fonts and applications should use version 1.
pub struct GaspTable<'a> {
    /// Version number (0 or 1—set to 1 in new fonts)
    pub version: u16,
    /// Number of records to follow
    pub num_ranges: u16,
    /// Sorted by ppem
    pub gasp_ranges: &'a [GaspRange],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GaspRange {
    /// Upper limit of range, in PPEM
    pub range_max_ppem: U16BE,
    /// Flags describing desired rasterizer behavior.
    pub range_gasp_behavior: RangeGaspBehavior,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RangeGaspBehavior(U16BE);

impl RangeGaspBehavior {
    /// Use gridfitting
    pub const fn gasp_gridfit(self) -> bool {
        self.0.into_u16() & 0x0001 != 0
    }

    /// Use grayscale rendering
    pub const fn gasp_dogray(self) -> bool {
        self.0.into_u16() & 0x0002 != 0
    }

    /// Use gridfitting with ClearType symmetric smoothing
    /// Only supported in version 1
    pub const fn gasp_symmetric_gridfit(self) -> bool {
        self.0.into_u16() & 0x0004 != 0
    }

    /// Use smoothing along multiple axes with ClearType®
    /// Only supported in version 1
    pub const fn gasp_symmetric_smoothing(self) -> bool {
        self.0.into_u16() & 0x0008 != 0
    }
}

pub(crate) struct GaspParser<'a> {
    stream: Stream<'a>,
}

impl<'a> GaspParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<GaspTable<'a>> {
        let version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let num_ranges = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let gasp_ranges = match self.stream.parse_slice(num_ranges as usize) {
            Some(v) => v,
            _ => return None,
        };
        Some(GaspTable {
            version,
            num_ranges,
            gasp_ranges,
        })
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_gasp(&self, input: TableRecord) -> Option<GaspTable<'a>> {
        if input.table_tag.is_gasp() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = GaspParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
