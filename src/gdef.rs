use crate::{
    common::{
        ClassDefinitionParser, ClassDefinitionTable, CoverageParser, CoverageTable,
        ItemVariationStore, ItemVariationStoreParser,
    },
    endian::U16BE,
    parser::{Parser, TableRecord},
    stream::Stream,
    util::{slice_range, slice_rest},
};

/// The GDEF table begins with a header that starts with a version number. Three versions are defined. Version 1.0 contains offsets to glyph class definition, attachment point list, ligature caret list and mark attachment class definition tables. Version 1.2 also includes an offset to a mark glyph sets table. Version 1.3 also includes an offset to an item variation store table.
#[derive(Debug, Clone)]
pub enum GdefTable<'a> {
    GdefVersion1_0(GdefVersion1_0<'a>),
    GdefVersion1_2(GdefVersion1_2<'a>),
    GdefVersion1_3(GdefVersion1_3<'a>),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GdefVersion1_0<'a> {
    /// Major version of the GDEF table, = 1.
    pub major_version: u16,
    /// Minor version of the GDEF table, = 0.
    pub minor_version: u16,
    /// Offset to class definition table for glyph type, from beginning of GDEF header (may be NULL).
    pub glyph_class_def_offset: u16,
    /// Offset to attachment point list table, from beginning of GDEF header (may be NULL).
    pub attach_list_offset: u16,
    /// Offset to ligature caret list table, from beginning of GDEF header (may be NULL).
    pub lig_caret_list_offset: u16,
    /// Offset to class definition table for mark attachment type, from beginning of GDEF header (may be NULL).
    pub mark_attach_class_def_offset: u16,
    pub glyph_class_def_table: Option<GlyphClassDefinitionTable<'a>>,
    pub attach_list: Option<AttachList<'a>>,
    pub lig_caret_list: Option<LigCaretList<'a>>,
    pub mark_attach_class_def_table: Option<MarkAttachmentClassDefinitionTable<'a>>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GdefVersion1_2<'a> {
    /// Major version of the GDEF table, = 1.
    pub major_version: u16,
    /// Minor version of the GDEF table, = 2.
    pub minor_version: u16,
    /// Offset to class definition table for glyph type, from beginning of GDEF header (may be NULL).
    pub glyph_class_def_offset: u16,
    /// Offset to attachment point list table, from beginning of GDEF header (may be NULL).
    pub attach_list_offset: u16,
    /// Offset to ligature caret list table, from beginning of GDEF header (may be NULL).
    pub lig_caret_list_offset: u16,
    /// Offset to class definition table for mark attachment type, from beginning of GDEF header (may be NULL).
    pub mark_attach_class_def_offset: u16,
    /// Offset to the table of mark glyph set definitions, from beginning of GDEF header (may be NULL).
    pub mark_glyph_sets_def_offset: u16,
    pub glyph_class_def_table: Option<GlyphClassDefinitionTable<'a>>,
    pub attach_list: Option<AttachList<'a>>,
    pub lig_caret_list: Option<LigCaretList<'a>>,
    pub mark_attach_class_def_table: Option<MarkAttachmentClassDefinitionTable<'a>>,
    pub mark_glyph_sets_def_table: Option<MarkGlyphSetsDef<'a>>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GdefVersion1_3<'a> {
    /// Major version of the GDEF table, = 1.
    pub major_version: u16,
    /// Minor version of the GDEF table, = 3.
    pub minor_version: u16,
    /// Offset to class definition table for glyph type, from beginning of GDEF header (may be NULL).
    pub glyph_class_def_offset: u16,
    /// Offset to attachment point list table, from beginning of GDEF header (may be NULL).
    pub attach_list_offset: u16,
    /// Offset to ligature caret list table, from beginning of GDEF header (may be NULL).
    pub lig_caret_list_offset: u16,
    /// Offset to class definition table for mark attachment type, from beginning of GDEF header (may be NULL).
    pub mark_attach_class_def_offset: u16,
    /// Offset to the table of mark glyph set definitions, from beginning of GDEF header (may be NULL).
    pub mark_glyph_sets_def_offset: u16,
    /// Offset to the item variation store table, from beginning of GDEF header (may be NULL).
    pub item_var_store_offset: u32,
    pub glyph_class_def_table: Option<GlyphClassDefinitionTable<'a>>,
    pub attach_list: Option<AttachList<'a>>,
    pub lig_caret_list: Option<LigCaretList<'a>>,
    pub mark_attach_class_def_table: Option<MarkAttachmentClassDefinitionTable<'a>>,
    pub mark_glyph_sets_def_table: Option<MarkGlyphSetsDef<'a>>,
    pub item_var_store_table: Option<ItemVariationStore<'a>>,
}

pub type GlyphClassDefinitionTable<'a> = ClassDefinitionTable<'a, GlyphClassDef>;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphClassDef(U16BE);

impl GlyphClassDef {
    /// Base glyph (single character, spacing glyph)
    #[inline]
    pub const fn is_base_glyph(self) -> bool {
        self.0.into_u16() == 1
    }

    /// Ligature glyph (multiple character, spacing glyph)
    #[inline]
    pub const fn is_ligature_glyph(self) -> bool {
        self.0.into_u16() == 2
    }

    /// Mark glyph (non-spacing combining glyph)
    #[inline]
    pub const fn is_mark_glyph(self) -> bool {
        self.0.into_u16() == 3
    }

    /// Component glyph (part of single character, spacing glyph)
    #[inline]
    pub const fn is_component_glyph(self) -> bool {
        self.0.into_u16() == 4
    }

    #[inline]
    pub const fn into_u16(self) -> u16 {
        self.0.into_u16()
    }
}

/// The attachment point list table (AttachList) may be used to cache attachment point coordinates along with glyph bitmaps.
///
/// The table consists of an offset to a Coverage table listing all glyphs that define attachment points in the GPOS table, a count of the glyphs with attachment points, and an array of offsets to AttachPoint tables. The array lists the AttachPoint tables, one for each glyph in the Coverage table, in the same order as the Coverage Index.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AttachList<'a> {
    /// Offset to Coverage table, from beginning of AttachList table.
    pub coverage_offset: u16,
    /// Number of glyphs with attachment points.
    pub glyph_count: u16,
    /// Array of offsets to AttachPoint tables, from beginning of AttachList table, in Coverage Index order.
    pub attach_point_offsets: &'a [AttachPointOffset],
    data: &'a [u8],
}

impl<'a> AttachList<'a> {
    pub const fn parse_attach_point_offset(
        &self,
        input: AttachPointOffset,
    ) -> Option<AttachPoint<'a>> {
        let mut stream = Stream::new(slice_rest(self.data, input.0.into_u16() as usize));
        let point_count = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let point_indices = match stream.parse_slice(point_count as usize) {
            Some(v) => v,
            _ => return None,
        };
        Some(AttachPoint {
            point_count,
            point_indices,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AttachPointOffset(U16BE);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AttachPoint<'a> {
    /// Number of attachment points on this glyph.
    pub point_count: u16,
    /// Array of contour point indices, in increasing numerical order.
    pub point_indices: &'a [U16BE],
}

/// The ligature caret list table (LigCaretList) defines caret positions for ligature glyphs in a font. The table consists of an offset to a Coverage table that lists all the ligature glyphs, a count of the defined ligatures, and an array of offsets to LigGlyph tables. The array lists the LigGlyph tables, one for each ligature in the Coverage table, in the same order as the Coverage Index.
///
/// Example 4 at the end of this chapter shows a LigCaretList table.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LigCaretList<'a> {
    pub coverage_offset: u16,
    pub lig_glyph_count: u16,
    pub lig_glyph_offsets: &'a [LigGlyphOffset],
    pub coverage: CoverageTable<'a>,
    data: &'a [u8],
}

impl<'a> LigCaretList<'a> {
    pub const fn parse_lig_glyph_offset(&self, input: LigGlyphOffset) -> Option<LigGlyph<'a>> {
        let data = slice_rest(self.data, input.0.into_u16() as usize);
        let mut stream = Stream::new(data);
        let caret_count = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let caret_value_offsets = match stream.parse_slice(caret_count as usize) {
            Some(v) => v,
            _ => return None,
        };
        Some(LigGlyph {
            caret_count,
            caret_value_offsets,
            data,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LigGlyphOffset(U16BE);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LigGlyph<'a> {
    pub caret_count: u16,
    pub caret_value_offsets: &'a [CaretValueOffset],
    data: &'a [u8],
}

impl<'a> LigGlyph<'a> {
    pub const fn parse_caret_value_offset(&self, input: CaretValueOffset) -> Option<CaretValue> {
        let mut stream = Stream::new(slice_rest(self.data, input.0.into_u16() as usize));
        let format = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        match format {
            1 => {
                let coordinate = match stream.parse_i16() {
                    Some(v) => v,
                    _ => return None,
                };
                Some(CaretValue::CaretValueFormat1(CaretValueFormat1 {
                    format,
                    coordinate,
                }))
            }
            2 => {
                let caret_value_point_index = match stream.parse_u16() {
                    Some(v) => v,
                    _ => return None,
                };
                Some(CaretValue::CaretValueFormat2(CaretValueFormat2 {
                    format,
                    caret_value_point_index,
                }))
            }
            3 => {
                let coordinate = match stream.parse_i16() {
                    Some(v) => v,
                    _ => return None,
                };
                let device_offset = match stream.parse_u16() {
                    Some(v) => v,
                    _ => return None,
                };
                Some(CaretValue::CaretValueFormat3(CaretValueFormat3 {
                    format,
                    coordinate,
                    device_offset,
                }))
            }
            _ => None,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct CaretValueOffset(U16BE);

#[derive(Debug, Clone)]
pub enum CaretValue {
    CaretValueFormat1(CaretValueFormat1),
    CaretValueFormat2(CaretValueFormat2),
    CaretValueFormat3(CaretValueFormat3),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CaretValueFormat1 {
    pub format: u16,
    pub coordinate: i16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CaretValueFormat2 {
    pub format: u16,
    pub caret_value_point_index: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CaretValueFormat3 {
    pub format: u16,
    pub coordinate: i16,
    pub device_offset: u16,
}

pub type MarkAttachmentClassDefinitionTable<'a> = ClassDefinitionTable<'a, U16BE>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MarkGlyphSetsDef<'a> {
    pub format: u16,
    pub mark_glyph_set_count: u16,
    pub coverage_offsets: &'a [MarkGlyphSetCoverageOffset],
    data: &'a [u8],
}

impl<'a> MarkGlyphSetsDef<'a> {
    pub const fn parse_mark_glyph_set_coverage_offset(
        &self,
        input: MarkGlyphSetCoverageOffset,
    ) -> Option<MarkGlyphSet<'a>> {
        let mut parser = CoverageParser::new(slice_rest(self.data, input.0.into_u16() as usize));
        parser.parse()
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct MarkGlyphSetCoverageOffset(U16BE);

pub type MarkGlyphSet<'a> = CoverageTable<'a>;

pub(crate) struct GdefParser<'a> {
    stream: Stream<'a>,
}

impl<'a> GdefParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<GdefTable<'a>> {
        let major_version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let minor_version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        match (major_version, minor_version) {
            (1, 0) => Some(GdefTable::GdefVersion1_0(
                match self.parse_gdef_version_1_0(major_version, minor_version) {
                    Some(v) => v,
                    _ => return None,
                },
            )),
            (1, 2) => Some(GdefTable::GdefVersion1_2(
                match self.parse_gdef_version_1_2(major_version, minor_version) {
                    Some(v) => v,
                    _ => return None,
                },
            )),
            (1, 3) => Some(GdefTable::GdefVersion1_3(
                match self.parse_gdef_version_1_3(major_version, minor_version) {
                    Some(v) => v,
                    _ => return None,
                },
            )),
            _ => None,
        }
    }

    pub(crate) const fn parse_gdef_version_1_0(
        &mut self,
        major_version: u16,
        minor_version: u16,
    ) -> Option<GdefVersion1_0<'a>> {
        let glyph_class_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let attach_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lig_caret_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let mark_attach_class_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let glyph_class_def_table = self.parse_glyph_class_def_table(glyph_class_def_offset);
        let attach_list = self.parse_attach_list(attach_list_offset);
        let lig_caret_list = self.parse_lig_caret_list(lig_caret_list_offset);
        let mark_attach_class_def_table =
            self.parse_mark_attach_class_def_table(mark_attach_class_def_offset);
        Some(GdefVersion1_0 {
            major_version,
            minor_version,
            glyph_class_def_offset,
            attach_list_offset,
            lig_caret_list_offset,
            mark_attach_class_def_offset,
            glyph_class_def_table,
            attach_list,
            lig_caret_list,
            mark_attach_class_def_table,
        })
    }
    pub(crate) const fn parse_gdef_version_1_2(
        &mut self,
        major_version: u16,
        minor_version: u16,
    ) -> Option<GdefVersion1_2<'a>> {
        let glyph_class_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let attach_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lig_caret_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let mark_attach_class_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let mark_glyph_sets_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let glyph_class_def_table = self.parse_glyph_class_def_table(glyph_class_def_offset);
        let attach_list = self.parse_attach_list(attach_list_offset);
        let lig_caret_list = self.parse_lig_caret_list(lig_caret_list_offset);
        let mark_attach_class_def_table =
            self.parse_mark_attach_class_def_table(mark_attach_class_def_offset);
        let mark_glyph_sets_def_table =
            self.parse_mark_glyph_sets_def_table(mark_glyph_sets_def_offset);
        Some(GdefVersion1_2 {
            major_version,
            minor_version,
            glyph_class_def_offset,
            attach_list_offset,
            lig_caret_list_offset,
            mark_attach_class_def_offset,
            mark_glyph_sets_def_offset,
            glyph_class_def_table,
            attach_list,
            lig_caret_list,
            mark_attach_class_def_table,
            mark_glyph_sets_def_table,
        })
    }
    pub(crate) const fn parse_gdef_version_1_3(
        &mut self,
        major_version: u16,
        minor_version: u16,
    ) -> Option<GdefVersion1_3<'a>> {
        let glyph_class_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let attach_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lig_caret_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let mark_attach_class_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let mark_glyph_sets_def_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let item_var_store_offset = match self.stream.parse_u32() {
            Some(v) => v,
            _ => return None,
        };
        let glyph_class_def_table = self.parse_glyph_class_def_table(glyph_class_def_offset);
        let attach_list = self.parse_attach_list(attach_list_offset);
        let lig_caret_list = self.parse_lig_caret_list(lig_caret_list_offset);
        let mark_attach_class_def_table =
            self.parse_mark_attach_class_def_table(mark_glyph_sets_def_offset);
        let mark_glyph_sets_def_table =
            self.parse_mark_glyph_sets_def_table(mark_glyph_sets_def_offset);
        let item_var_store_table = self.parse_item_var_store_table(item_var_store_offset);
        Some(GdefVersion1_3 {
            major_version,
            minor_version,
            glyph_class_def_offset,
            attach_list_offset,
            lig_caret_list_offset,
            mark_attach_class_def_offset,
            mark_glyph_sets_def_offset,
            item_var_store_offset,
            glyph_class_def_table,
            attach_list,
            lig_caret_list,
            mark_attach_class_def_table,
            mark_glyph_sets_def_table,
            item_var_store_table,
        })
    }

    pub(crate) const fn parse_glyph_class_def_table(
        &mut self,
        offset: u16,
    ) -> Option<GlyphClassDefinitionTable<'a>> {
        if offset == 0 {
            return None;
        }
        let mut parser = ClassDefinitionParser::new(slice_rest(self.stream.bytes, offset as usize));
        parser.parse()
    }

    pub(crate) const fn parse_attach_list(&mut self, offset: u16) -> Option<AttachList<'a>> {
        if offset == 0 {
            return None;
        }
        let data = slice_rest(self.stream.bytes, offset as usize);
        let mut stream = Stream::new(data);
        let coverage_offset = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let glyph_count = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let attach_point_offsets = match stream.parse_slice(glyph_count as usize) {
            Some(v) => v,
            _ => return None,
        };
        Some(AttachList {
            coverage_offset,
            glyph_count,
            attach_point_offsets,
            data,
        })
    }

    pub(crate) const fn parse_lig_caret_list(&mut self, offset: u16) -> Option<LigCaretList<'a>> {
        if offset == 0 {
            return None;
        }
        let data = slice_rest(self.stream.bytes, offset as usize);
        let mut stream = Stream::new(data);
        let coverage_offset = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lig_glyph_count = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lig_glyph_offsets = match stream.parse_slice(lig_glyph_count as usize) {
            Some(v) => v,
            _ => return None,
        };
        let mut coverage_parser = CoverageParser::new(slice_rest(data, coverage_offset as usize));
        let coverage = match coverage_parser.parse() {
            Some(v) => v,
            _ => return None,
        };
        Some(LigCaretList {
            coverage_offset,
            lig_glyph_count,
            lig_glyph_offsets,
            data,
            coverage,
        })
    }

    pub(crate) const fn parse_mark_attach_class_def_table(
        &mut self,
        offset: u16,
    ) -> Option<MarkAttachmentClassDefinitionTable<'a>> {
        if offset == 0 {
            return None;
        }
        let mut parser = ClassDefinitionParser::new(slice_rest(self.stream.bytes, offset as usize));
        parser.parse()
    }

    pub(crate) const fn parse_mark_glyph_sets_def_table(
        &mut self,
        offset: u16,
    ) -> Option<MarkGlyphSetsDef<'a>> {
        if offset == 0 {
            return None;
        }
        let data = slice_rest(self.stream.bytes, offset as usize);
        let mut stream = Stream::new(data);
        let format = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let mark_glyph_set_count = match stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let coverage_offsets = match stream.parse_slice(mark_glyph_set_count as usize) {
            Some(v) => v,
            _ => return None,
        };
        Some(MarkGlyphSetsDef {
            format,
            mark_glyph_set_count,
            coverage_offsets,
            data,
        })
    }

    pub(crate) const fn parse_item_var_store_table(
        &mut self,
        offset: u32,
    ) -> Option<ItemVariationStore<'a>> {
        if offset == 0 {
            return None;
        }
        let mut parser =
            ItemVariationStoreParser::new(slice_rest(self.stream.bytes, offset as usize));
        parser.parse()
    }
}

impl<'a> Parser<'a> {
    pub fn parse_gdef(&self, input: TableRecord) -> Option<GdefTable<'a>> {
        if input.table_tag.is_gdef() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = GdefParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
