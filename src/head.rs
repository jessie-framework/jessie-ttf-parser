use crate::{
    fixed::Fixed,
    longdatetime::LongDateTime,
    parser::{Parser, TableRecord},
    stream::Stream,
    util::slice_range,
};

pub(crate) struct HeadParser<'a> {
    stream: Stream<'a>,
}

impl<'a> HeadParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    const fn parse(&mut self) -> Option<HeadTable> {
        let major_version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let minor_version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let font_revision = Fixed::from_u32(match self.stream.parse_u32() {
            Some(v) => v,
            _ => return None,
        });
        let checksum_adjustment = match self.stream.parse_u32() {
            Some(v) => v,
            _ => return None,
        };
        let magic_number = match self.stream.parse_u32() {
            Some(v) => v,
            _ => return None,
        };
        let flags = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let units_per_em = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let created = match self.stream.parse_i64() {
            Some(v) => v,
            _ => return None,
        };
        let modified = match self.stream.parse_i64() {
            Some(v) => v,
            _ => return None,
        };
        let x_min = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let y_min = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let x_max = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let y_max = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let mac_style = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lowest_rec_ppem = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let font_direction_hint = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let index_to_loc_format = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let glyph_data_format = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        Some(HeadTable {
            major_version,
            minor_version,
            font_revision,
            checksum_adjustment,
            magic_number,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        })
    }
}

/// This table gives global information about the font. The bounding box values should be computed using only glyphs that have contours. Glyphs with no contours should be ignored for the purposes of these calculations.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HeadTable {
    /// Major version number of the font header table — set to 1.
    pub major_version: u16,
    /// Minor version number of the font header table — set to 0.
    pub minor_version: u16,
    /// Set by font manufacturer.
    pub font_revision: Fixed,
    /// To compute: set it to 0, sum the entire font as uint32, then store 0xB1B0AFBA - sum. If the font is used as a component in a font collection file, the value of this field will be invalidated by changes to the file structure and font table directory, and must be ignored.
    pub checksum_adjustment: u32,
    /// Set to 0x5F0F3CF5.
    pub magic_number: u32,
    /// Bit 0: Baseline for font at y=0.
    /// Bit 1: Left sidebearing point at x=0 (relevant only for TrueType rasterizers) — see additional information below regarding variable fonts.

    /// Bit 2: Instructions may depend on point size.

    /// Bit 3: Force ppem to integer values for all internal scaler math; may use fractional ppem sizes if this bit is clear. It is strongly recommended that this be set in hinted fonts.

    /// Bit 4: Instructions may alter advance width (the advance widths might not scale linearly).

    /// Bit 5: This bit is not used in OpenType, and should not be set in order to ensure compatible behavior on all platforms. If set, it may result in different behavior for vertical layout in some platforms. (See Apple’s specification for details regarding behavior in Apple platforms.)

    /// Bits 6 – 10: These bits are not used in OpenType and should always be cleared. (See Apple’s specification for details regarding legacy use in Apple platforms.)

    /// Bit 11: Font data is “lossless” as a result of having been subjected to optimizing transformation and/or compression (such as compression mechanisms defined by ISO/IEC 14496-18, MicroType® Express, WOFF 2.0, or similar) where the original font functionality and features are retained but the binary compatibility between input and output font files is not guaranteed. As a result of the applied transform, the DSIG table may also be invalidated.

    /// Bit 12: Font converted (produce compatible metrics).

    /// Bit 13: Font optimized for ClearType®. Note, fonts that rely on embedded bitmaps (EBDT) for rendering should not be considered optimized for ClearType, and therefore should keep this bit cleared.

    /// Bit 14: Last Resort font. If set, indicates that the glyphs encoded in the 'cmap' subtables are simply generic symbolic representations of code point ranges and do not truly represent support for those code points. If unset, indicates that the glyphs encoded in the 'cmap' subtables represent proper support for those code points.

    /// Bit 15: Reserved, set to 0.
    pub flags: u16,
    /// Set to a value from 16 to 16384. Any value in this range is valid. In fonts that have TrueType outlines, a power of 2 is recommended as this allows performance optimization in some rasterizers.
    pub units_per_em: u16,
    /// Number of seconds since 12:00 midnight that started January 1st, 1904, in GMT/UTC time zone.
    pub created: LongDateTime,
    /// Number of seconds since 12:00 midnight that started January 1st, 1904, in GMT/UTC time zone.
    pub modified: LongDateTime,
    /// Minimum x coordinate across all glyph bounding boxes.
    pub x_min: i16,
    /// Minimum y coordinate across all glyph bounding boxes.
    pub y_min: i16,
    /// Maximum x coordinate across all glyph bounding boxes.
    pub x_max: i16,
    /// Maximum y coordinate across all glyph bounding boxes.
    pub y_max: i16,
    /// Bit 0: Bold (if set to 1);
    /// Bit 1: Italic (if set to 1)
    /// Bit 2: Underline (if set to 1)
    /// Bit 3: Outline (if set to 1)
    /// Bit 4: Shadow (if set to 1)
    /// Bit 5: Condensed (if set to 1)
    /// Bit 6: Extended (if set to 1)
    /// Bits 7 – 15: Reserved (set to 0).
    pub mac_style: u16,
    /// Smallest readable size in pixels.
    pub lowest_rec_ppem: u16,
    /// Deprecated (Set to 2).
    /// 0: Fully mixed directional glyphs;
    /// 1: Only strongly left to right;
    /// 2: Like 1 but also contains neutrals;
    /// -1: Only strongly right to left;
    /// -2: Like -1 but also contains neutrals.
    pub font_direction_hint: i16,
    /// 0 for short offsets (Offset16), 1 for long (Offset32).
    pub index_to_loc_format: i16,
    /// 0 for current format.
    pub glyph_data_format: i16,
}

impl<'a> Parser<'a> {
    pub const fn parse_head(&self, input: TableRecord) -> Option<HeadTable> {
        if input.table_tag.is_head() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = HeadParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
