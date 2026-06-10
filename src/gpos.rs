use crate::{
    common::{
        FeatureList, FeatureListParser, FeatureVariations, FeatureVariationsParser, LookupList,
        LookupListParser, ScriptList, ScriptListParser,
    },
    parser::{Parser, TableRecord},
    stream::Stream,
    util::{slice_range, slice_rest},
};

/// The GPOS table begins with a header that contains a version number for the table. Two versions are defined. Version 1.0 contains offsets to three tables: ScriptList, FeatureList, and LookupList. Version 1.1 also includes an offset to a FeatureVariations table. For descriptions of these tables, see the chapter, OpenType Layout Common Table Formats . Example 1 at the end of this chapter shows a GPOS Header version 1.0 table definition.
#[derive(Debug, Clone)]
pub enum GposTable<'a> {
    GposVersion1_0(GposVersion1_0<'a>),
    GposVersion1_1(GposVersion1_1<'a>),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GposVersion1_0<'a> {
    /// Major version of the GPOS table, = 1.
    pub major_version: u16,
    /// Minor version of the GPOS table, = 0.
    pub minor_version: u16,
    /// Offset to ScriptList table, from beginning of GPOS table.
    pub script_list_offset: u16,
    /// Offset to FeatureList table, from beginning of GPOS table.
    pub feature_list_offset: u16,
    /// Offset to LookupList table, from beginning of GPOS table.
    pub lookup_list_offset: u16,
    pub script_list_table: ScriptList<'a>,
    pub feature_list_table: FeatureList<'a>,
    pub lookup_list_table: LookupList<'a>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GposVersion1_1<'a> {
    /// Major version of the GPOS table, = 1.
    pub major_version: u16,
    /// Minor version of the GPOS table, = 1.
    pub minor_version: u16,
    /// Offset to ScriptList table, from beginning of GPOS table.
    pub script_list_offset: u16,
    /// Offset to FeatureList table, from beginning of GPOS table.
    pub feature_list_offset: u16,
    /// Offset to LookupList table, from beginning of GPOS table.
    pub lookup_list_offset: u16,
    /// Offset to FeatureVariations table, from beginning of GPOS table (may be NULL).
    pub feature_variations_offset: u32,
    pub script_list_table: ScriptList<'a>,
    pub feature_list_table: FeatureList<'a>,
    pub lookup_list_table: LookupList<'a>,
    pub feature_variations_table: Option<FeatureVariations<'a>>,
}

pub(crate) struct GposParser<'a> {
    stream: Stream<'a>,
}

impl<'a> GposParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<GposTable<'a>> {
        let major_version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let minor_version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let script_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let feature_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lookup_list_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let script_list_table =
            match ScriptListParser::new(slice_rest(self.stream.bytes, script_list_offset as usize))
                .parse()
            {
                Some(v) => v,
                _ => return None,
            };
        let feature_list_table = match FeatureListParser::new(slice_rest(
            self.stream.bytes,
            feature_list_offset as usize,
        ))
        .parse()
        {
            Some(v) => v,
            _ => return None,
        };
        let lookup_list_table =
            match LookupListParser::new(slice_rest(self.stream.bytes, lookup_list_offset as usize))
                .parse()
            {
                Some(v) => v,
                _ => return None,
            };
        match (major_version, minor_version) {
            (1, 0) => Some(GposTable::GposVersion1_0(GposVersion1_0 {
                major_version,
                minor_version,
                script_list_offset,
                feature_list_offset,
                lookup_list_offset,
                script_list_table,
                feature_list_table,
                lookup_list_table,
            })),
            (1, 1) => {
                let feature_variations_offset = match self.stream.parse_u32() {
                    Some(v) => v,
                    _ => return None,
                };
                let feature_variations_table = if feature_variations_offset == 0 {
                    None
                } else {
                    FeatureVariationsParser::new(slice_rest(
                        self.stream.bytes,
                        feature_variations_offset as usize,
                    ))
                    .parse()
                };
                Some(GposTable::GposVersion1_1(GposVersion1_1 {
                    major_version,
                    minor_version,
                    script_list_offset,
                    feature_list_offset,
                    lookup_list_offset,
                    feature_variations_offset,
                    script_list_table,
                    feature_list_table,
                    lookup_list_table,
                    feature_variations_table,
                }))
            }
            _ => None,
        }
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_gpos(&self, input: TableRecord) -> Option<GposTable<'a>> {
        if input.table_tag.is_gpos() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = GposParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
