use crate::{fword::FWord, parser::Parser, parser::TableRecord, stream::Stream, ufword::UFWord};

/// This table contains information for horizontal layout. The values in the minRightSidebearing, minLeftSideBearing and xMaxExtent should be computed using only glyphs that have contours. Glyphs with no contours should be ignored for the purposes of these calculations. All reserved areas must be set to 0.
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct HheaTable {
    /// Major version number of the horizontal header table — set to 1.
    pub major_version: u16,
    /// Minor version number of the horizontal header table — set to 0.
    pub minor_version: u16,
    /// Typographic ascent—see remarks below.
    pub ascender: u16,
    /// Typographic descent—see remarks below.
    pub descender: u16,
    ///  Typographic line gap.
    /// Negative lineGap values are treated as zero in some legacy platform implementations.
    pub line_gap: FWord,
    /// Maximum advance width value in 'hmtx' table.
    pub advance_width_max: UFWord,
    /// Minimum left sidebearing value in 'hmtx' table for glyphs with contours (empty glyphs should be ignored).
    pub min_left_side_bearing: FWord,
    /// Minimum right sidebearing value; calculated as min(aw - (lsb + xMax - xMin)) for glyphs with contours (empty glyphs should be ignored).
    pub min_right_side_bearing: FWord,
    /// Max(lsb + (xMax - xMin)).
    pub x_max_extent: FWord,
    /// Used to calculate the slope of the cursor (rise/run); 1 for vertical.
    pub caret_slope_rise: i16,
    /// 0 for vertical.
    pub caret_slope_run: i16,
    /// The amount by which a slanted highlight on a glyph needs to be shifted to produce the best appearance. Set to 0 for non-slanted fonts
    pub caret_offset: i16,
    /// set to 0
    pub reserved1: i16,
    /// set to 0
    pub reserved2: i16,
    /// set to 0
    pub reserved3: i16,
    /// set to 0
    pub reserved4: i16,
    /// 0 for current format.
    pub metric_data_format: i16,
    /// Number of hMetric entries in 'hmtx' table
    pub number_of_h_metrics: u16,
}

pub(crate) struct HheaParser<'a> {
    stream: Stream<'a>,
}

impl<'a> HheaParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<HheaTable> {
        let major_version = self.stream.parse_u16()?;
        let minor_version = self.stream.parse_u16()?;
        let ascender = self.stream.parse_u16()?;
        let descender = self.stream.parse_u16()?;
        let line_gap = self.stream.parse_i16()?;
        let advance_width_max = self.stream.parse_u16()?;
        let min_left_side_bearing = self.stream.parse_i16()?;
        let min_right_side_bearing = self.stream.parse_i16()?;
        let x_max_extent = self.stream.parse_i16()?;
        let caret_slope_rise = self.stream.parse_i16()?;
        let caret_slope_run = self.stream.parse_i16()?;
        let caret_offset = self.stream.parse_i16()?;
        let reserved1 = self.stream.parse_i16()?;
        let reserved2 = self.stream.parse_i16()?;
        let reserved3 = self.stream.parse_i16()?;
        let reserved4 = self.stream.parse_i16()?;
        let metric_data_format = self.stream.parse_i16()?;
        let number_of_h_metrics = self.stream.parse_u16()?;
        Some(HheaTable {
            major_version,
            minor_version,
            ascender,
            descender,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            reserved1,
            reserved2,
            reserved3,
            reserved4,
            metric_data_format,
            number_of_h_metrics,
        })
    }
}

impl<'a> Parser<'a> {
    pub fn parse_hhea(&self, input: TableRecord) -> Option<HheaTable> {
        if input.table_tag.is_hhea() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = HheaParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
