use crate::parser::Parser;
use crate::parser::TableRecord;
use crate::stream::Stream;

/// This table establishes the memory requirements for this font. Fonts with CFF or CFF2 outlines must use Version 0.5 of this table, specifying only the numGlyphs field. Fonts with TrueType outlines must use Version 1.0 of this table, where all data is required.
#[derive(Clone, Debug)]
pub enum MaxpTable {
    Version0_5(MaxpVersion0_5),
    Version1_0(MaxpVersion1_0),
}

impl MaxpTable {
    pub fn num_glyphs(&self) -> u16 {
        match self {
            Self::Version0_5(t) => t.num_glyphs,
            Self::Version1_0(t) => t.num_glyphs,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MaxpVersion0_5 {
    /// 0x00005000 for version 0.5
    pub version: u32,
    /// The number of glyphs in the font.
    pub num_glyphs: u16,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MaxpVersion1_0 {
    /// 0x00010000 for version 1.0.
    pub version: u32,
    /// The number of glyphs in the font.
    pub num_glyphs: u16,
    /// Maximum points in a non-composite glyph.
    pub max_points: u16,
    /// Maximum contours in a non-composite glyph.
    pub max_contours: u16,
    /// Maximum points in a composite glyph.
    pub max_composite_points: u16,
    /// Maximum contours in a composite glyph.
    pub max_composite_contours: u16,
    /// 1 if instructions do not use the twilight zone (Z0), or 2 if instructions do use Z0; should be set to 2 in most cases.
    pub max_zones: u16,
    /// Maximum points used in Z0.
    pub max_twilight_points: u16,
    /// Number of Storage Area locations.
    pub max_storage: u16,
    /// Number of FDEFs, equal to the highest function number + 1.
    pub max_function_defs: u16,
    /// Number of IDEFs.
    pub max_instruction_defs: u16,
    /// Maximum stack depth across Font Program ('fpgm' table), CVT Program ('prep' table) and all glyph instructions (in the 'glyf' table).
    pub max_stack_elements: u16,
    /// Maximum byte count for glyph instructions.
    pub max_size_of_instructions: u16,
    /// Maximum number of components referenced at “top level” for any composite glyph.
    pub max_component_elements: u16,
    /// Maximum levels of recursion; 1 for simple components.
    pub max_component_depth: u16,
}

pub(crate) struct MaxpParser<'a> {
    stream: Stream<'a>,
}

impl<'a> MaxpParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<MaxpTable> {
        let version = self.stream.parse_u32()?;
        let num_glyphs = self.stream.parse_u16()?;
        match version {
            0x00005000 => Some(MaxpTable::Version0_5(MaxpVersion0_5 {
                version,
                num_glyphs,
            })),
            0x00010000 => Some(MaxpTable::Version1_0(
                self.parse_version1_0(version, num_glyphs)?,
            )),
            _ => None,
        }
    }

    pub(crate) fn parse_version1_0(
        &mut self,
        version: u32,
        num_glyphs: u16,
    ) -> Option<MaxpVersion1_0> {
        let max_points = self.stream.parse_u16()?;
        let max_contours = self.stream.parse_u16()?;
        let max_composite_points = self.stream.parse_u16()?;
        let max_composite_contours = self.stream.parse_u16()?;
        let max_zones = self.stream.parse_u16()?;
        let max_twilight_points = self.stream.parse_u16()?;
        let max_storage = self.stream.parse_u16()?;
        let max_function_defs = self.stream.parse_u16()?;
        let max_instruction_defs = self.stream.parse_u16()?;
        let max_stack_elements = self.stream.parse_u16()?;
        let max_size_of_instructions = self.stream.parse_u16()?;
        let max_component_elements = self.stream.parse_u16()?;
        let max_component_depth = self.stream.parse_u16()?;
        Some(MaxpVersion1_0 {
            version,
            num_glyphs,
            max_points,
            max_contours,
            max_composite_points,
            max_composite_contours,
            max_zones,
            max_twilight_points,
            max_storage,
            max_function_defs,
            max_instruction_defs,
            max_stack_elements,
            max_size_of_instructions,
            max_component_elements,
            max_component_depth,
        })
    }
}
impl<'a> Parser<'a> {
    pub fn parse_maxp(&self, input: TableRecord) -> Option<MaxpTable> {
        if input.table_tag.is_maxp() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = MaxpParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
