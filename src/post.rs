use crate::{
    endian::U16BE,
    fixed::Fixed,
    fword::FWord,
    parser::{Parser, TableRecord},
    stream::Stream,
};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PostTable<'a> {
    Version1_0(PostVersion1_0),
    Version2_0(PostVersion2_0<'a>),
    Version2_5(PostVersion2_5<'a>),
    Version3_0(PostVersion3_0),
}

#[repr(C)]
#[derive(Debug, Clone)]
#[non_exhaustive]
/// This version is used in order to supply PostScript glyph names when the font file contains exactly the 258 glyphs in the standard Macintosh TrueType font file (see 'post' Format 1 in Apple’s specification for a list of the 258 Macintosh glyph names), and the font does not otherwise supply glyph names. As a result, the glyph names are taken from the system with no storage required by the font.
pub struct PostVersion1_0 {
    /// 0x00010000 for version 1.0
    /// 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated)
    /// 0x00030000 for version 3.0
    pub version: u32,
    /// Italic angle in counter-clockwise degrees from the vertical. Zero for upright text, negative for text that leans to the right (forward).
    pub italic_angle: Fixed,
    /// Suggested y-coordinate of the top of the underline.
    pub underline_position: FWord,
    /// Suggested values for the underline thickness. In general, the underline thickness should match the thickness of the underscore character (U+005F LOW LINE), and should also match the strikeout thickness, which is specified in the OS/2 table.
    pub underline_thickness: FWord,
    /// Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally spaced (i.e. monospaced).
    pub is_fixed_pitch: u32,
    /// Minimum memory usage when an OpenType font is downloaded.
    pub min_mem_type_42: u32,
    /// Maximum memory usage when an OpenType font is downloaded.
    pub max_mem_type_42: u32,
    /// Minimum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub min_mem_type_1: u32,
    /// Maximum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub max_mem_type_1: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
#[non_exhaustive]
/// Version 2.0 is used for fonts that use glyph names that are not in the set of Macintosh glyph names. A given font may map some of its glyphs to the standard Macintosh glyph names, and some to its own custom names. A version 2.0 'post' table can be used in fonts with TrueType or CFF version 2 outlines.
pub struct PostVersion2_0<'a> {
    /// 0x00010000 for version 1.0
    /// 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated)
    /// 0x00030000 for version 3.0
    pub version: u32,
    /// Italic angle in counter-clockwise degrees from the vertical. Zero for upright text, negative for text that leans to the right (forward).
    pub italic_angle: Fixed,
    /// Suggested y-coordinate of the top of the underline.
    pub underline_position: FWord,
    /// Suggested values for the underline thickness. In general, the underline thickness should match the thickness of the underscore character (U+005F LOW LINE), and should also match the strikeout thickness, which is specified in the OS/2 table.
    pub underline_thickness: FWord,
    /// Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally spaced (i.e. monospaced).
    pub is_fixed_pitch: u32,
    /// Minimum memory usage when an OpenType font is downloaded.
    pub min_mem_type_42: u32,
    /// Maximum memory usage when an OpenType font is downloaded.
    pub max_mem_type_42: u32,
    /// Minimum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub min_mem_type_1: u32,
    /// Maximum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub max_mem_type_1: u32,
    /// Number of glyphs (this should be the same as numGlyphs in 'maxp' table).
    pub num_glyphs: u16,
    /// Array of indices into the string data. See below for details.
    pub glyph_name_index: &'a [U16BE],
    /// Storage for the string data.
    pub string_data: &'a [u8],
}
#[repr(C)]
#[derive(Debug, Clone)]
#[non_exhaustive]
/// Version 2.5 of the 'post' table is deprecated.
/// This version provides a space-saving table for fonts containing TrueType outlines which contain a pure subset of, or a simple reordering of, the standard Macintosh glyph set.
pub struct PostVersion2_5<'a> {
    /// 0x00010000 for version 1.0
    /// 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated)
    /// 0x00030000 for version 3.0
    pub version: u32,
    /// Italic angle in counter-clockwise degrees from the vertical. Zero for upright text, negative for text that leans to the right (forward).
    pub italic_angle: Fixed,
    /// Suggested y-coordinate of the top of the underline.
    pub underline_position: FWord,
    /// Suggested values for the underline thickness. In general, the underline thickness should match the thickness of the underscore character (U+005F LOW LINE), and should also match the strikeout thickness, which is specified in the OS/2 table.
    pub underline_thickness: FWord,
    /// Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally spaced (i.e. monospaced).
    pub is_fixed_pitch: u32,
    /// Minimum memory usage when an OpenType font is downloaded.
    pub min_mem_type_42: u32,
    /// Maximum memory usage when an OpenType font is downloaded.
    pub max_mem_type_42: u32,
    /// Minimum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub min_mem_type_1: u32,
    /// Maximum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub max_mem_type_1: u32,
    /// Number of glyphs.
    pub num_glyphs: u16,
    /// Difference between the glyph index and the standard order of the glyph.
    pub offset: &'a [i8],
}

#[repr(C)]
#[derive(Debug, Clone)]
#[non_exhaustive]
/// This version makes it possible to create a font that is not burdened with a large set of glyph names. A version 3.0 'post' table can be used by OpenType fonts with TrueType or CFF (version 1 or 2) data.
///
/// This version specifies that no PostScript name information is provided for the glyphs in this font file. The printing behavior of this version on PostScript printers is unspecified, except that it should not result in a fatal or unrecoverable error. Some drivers may print nothing; other drivers may attempt to print using a default naming scheme.
pub struct PostVersion3_0 {
    /// 0x00010000 for version 1.0
    /// 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated)
    /// 0x00030000 for version 3.0
    pub version: u32,
    /// Italic angle in counter-clockwise degrees from the vertical. Zero for upright text, negative for text that leans to the right (forward).
    pub italic_angle: Fixed,
    /// Suggested y-coordinate of the top of the underline.
    pub underline_position: FWord,
    /// Suggested values for the underline thickness. In general, the underline thickness should match the thickness of the underscore character (U+005F LOW LINE), and should also match the strikeout thickness, which is specified in the OS/2 table.
    pub underline_thickness: FWord,
    /// Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally spaced (i.e. monospaced).
    pub is_fixed_pitch: u32,
    /// Minimum memory usage when an OpenType font is downloaded.
    pub min_mem_type_42: u32,
    /// Maximum memory usage when an OpenType font is downloaded.
    pub max_mem_type_42: u32,
    /// Minimum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub min_mem_type_1: u32,
    /// Maximum memory usage when an OpenType font is downloaded as a Type 1 font.
    pub max_mem_type_1: u32,
}

pub(crate) struct PostParser<'a> {
    stream: Stream<'a>,
}

impl<'a> PostParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<PostTable<'a>> {
        let version = self.stream.parse_u32()?;
        let italic_angle = Fixed::from_u32(self.stream.parse_u32()?);
        let underline_position = self.stream.parse_i16()?;
        let underline_thickness = self.stream.parse_i16()?;
        let is_fixed_pitch = self.stream.parse_u32()?;
        let min_mem_type_42 = self.stream.parse_u32()?;
        let max_mem_type_42 = self.stream.parse_u32()?;
        let min_mem_type_1 = self.stream.parse_u32()?;
        let max_mem_type_1 = self.stream.parse_u32()?;
        match version {
            0x00010000 => Some(PostTable::Version1_0(PostVersion1_0 {
                version,
                italic_angle,
                underline_position,
                underline_thickness,
                is_fixed_pitch,
                min_mem_type_42,
                max_mem_type_42,
                min_mem_type_1,
                max_mem_type_1,
            })),
            0x00020000 => {
                let num_glyphs = self.stream.parse_u16()?;
                let glyph_name_index = self.stream.parse_slice(num_glyphs as usize)?;
                let string_data = self.stream.parse_slice_rest();

                Some(PostTable::Version2_0(PostVersion2_0 {
                    version,
                    italic_angle,
                    underline_position,
                    underline_thickness,
                    is_fixed_pitch,
                    min_mem_type_42,
                    max_mem_type_42,
                    min_mem_type_1,
                    max_mem_type_1,
                    num_glyphs,
                    glyph_name_index,
                    string_data,
                }))
            }
            0x00025000 => {
                let num_glyphs = self.stream.parse_u16()?;
                let offset = self.stream.parse_slice(num_glyphs as usize)?;

                Some(PostTable::Version2_5(PostVersion2_5 {
                    version,
                    italic_angle,
                    underline_position,
                    underline_thickness,
                    is_fixed_pitch,
                    min_mem_type_42,
                    max_mem_type_42,
                    min_mem_type_1,
                    max_mem_type_1,
                    num_glyphs,
                    offset,
                }))
            }
            0x00030000 => Some(PostTable::Version3_0(PostVersion3_0 {
                version,
                italic_angle,
                underline_position,
                underline_thickness,
                is_fixed_pitch,
                min_mem_type_42,
                max_mem_type_42,
                min_mem_type_1,
                max_mem_type_1,
            })),
            _ => None,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_post(&self, input: TableRecord) -> Option<PostTable<'a>> {
        if input.table_tag.is_post() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = PostParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
