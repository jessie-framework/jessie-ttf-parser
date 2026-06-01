use core::marker::PhantomData;

use crate::{
    endian::{F2Dot14BE, U16BE, U32BE},
    parser::Tag,
    stream::Stream,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ItemVariationStore<'a, D = i8> {
    /// Set to 1
    pub format: u16,
    /// Offset in bytes from the start of the item variation store to the variation region list.
    pub variation_region_list_offset: u32,
    /// The number of item variation data subtables.
    pub item_variation_data_count: u16,
    /// Offsets in bytes from the start of the item variation store to each item variation data subtable.
    pub item_variation_data_offsets: &'a [ItemVariationDataOffset],
    pub variation_region_list: Option<VariationRegionList<'a>>,
    data: &'a [u8],
    _pd: PhantomData<D>,
}

impl<'a, D> ItemVariationStore<'a, D> {
    pub fn parse_item_variation_data(
        &self,
        input: ItemVariationDataOffset,
    ) -> Option<ItemVariationData<'a, D>> {
        if input.0.into_u32() == 0 {
            return None;
        }
        let mut stream = Stream::new(&self.data[input.0.into_u32() as usize..]);
        let data = stream.bytes;
        let item_count = stream.parse_u16()?;
        let word_delta_count = WordDeltaCount(stream.parse_u16()?);
        let region_index_count = stream.parse_u16()?;
        let region_indexes = stream.parse_slice(region_index_count as usize)?;
        let _pd = PhantomData;
        Some(ItemVariationData {
            item_count,
            word_delta_count,
            region_index_count,
            region_indexes,
            data,
            _pd,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ItemVariationDataOffset(U32BE);

pub struct ItemVariationData<'a, D = i8> {
    /// The number of delta sets for distinct items.
    pub item_count: u16,
    /// A packed field: the high bit is a flag—see details below.
    pub word_delta_count: WordDeltaCount,
    /// The number of variation regions referenced.
    pub region_index_count: u16,
    /// Array of indices into the variation region list for the regions referenced by this item variation data table.
    pub region_indexes: &'a [U16BE],
    data: &'a [u8],
    _pd: PhantomData<D>,
}

#[derive(Debug, Clone, Copy)]
pub struct WordDeltaCount(u16);

impl WordDeltaCount {
    /// Flag indicating that “word” deltas are long (int32)
    pub fn long_words(self) -> bool {
        self.0 & 0x8000 != 0
    }

    /// Count of “word” deltas
    pub fn word_delta_count_mask(self) -> u16 {
        self.0 ^ 0x7FFF
    }
}

impl<'a, D> ItemVariationData<'a, D> {
    pub fn iter_delta_sets(&self) -> DeltaSetIter<'a, D> {
        DeltaSetIter::new(self.data, self.word_delta_count.long_words())
    }
}

pub struct DeltaSetIter<'a, D> {
    stream: Stream<'a>,
    _pd: PhantomData<D>,
    is_long: bool,
}

impl<'a, D> DeltaSetIter<'a, D> {
    pub(crate) fn new(bytes: &'a [u8], is_long: bool) -> Self {
        Self {
            stream: Stream::new(bytes),
            _pd: PhantomData,
            is_long,
        }
    }
}

impl<'a> Iterator for DeltaSetIter<'a, i8> {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_long {
            return self.stream.parse_i16();
        }
        self.stream.parse_i8().map(i16::from)
    }
}

impl<'a> Iterator for DeltaSetIter<'a, i16> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_long {
            return self.stream.parse_i32();
        }
        self.stream.parse_i16().map(i32::from)
    }
}

pub(crate) struct ItemVariationStoreParser<'a, D = i8> {
    stream: Stream<'a>,
    _pd: PhantomData<D>,
}

impl<'a, D> ItemVariationStoreParser<'a, D> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            _pd: PhantomData,
        }
    }

    pub(crate) fn parse(&mut self) -> Option<ItemVariationStore<'a, D>> {
        let format = self.stream.parse_u16()?;
        let variation_region_list_offset = self.stream.parse_u32()?;
        let item_variation_data_count = self.stream.parse_u16()?;
        let item_variation_data_offsets = self
            .stream
            .parse_slice(item_variation_data_count as usize)?;
        let variation_region_list =
            self.parse_variation_region_list_from_offset(variation_region_list_offset);
        let data = self.stream.parse_slice_rest();
        let _pd = PhantomData;
        Some(ItemVariationStore {
            format,
            variation_region_list_offset,
            item_variation_data_count,
            item_variation_data_offsets,
            variation_region_list,
            data,
            _pd,
        })
    }

    pub(crate) fn parse_variation_region_list_from_offset(
        &mut self,
        offset: u32,
    ) -> Option<VariationRegionList<'a>> {
        if offset == 0 {
            return None;
        }
        let mut stream = Self::new(&self.stream.bytes[offset as usize..]);
        stream.parse_variation_region_list()
    }

    pub(crate) fn parse_variation_region_list(&mut self) -> Option<VariationRegionList<'a>> {
        let data = self.stream.bytes;
        let axis_count = self.stream.parse_u16()?;
        let region_count = self.stream.parse_u16()?;
        Some(VariationRegionList {
            axis_count,
            region_count,
            data,
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VariationRegionList<'a> {
    /// The number of variation axes for this font. This must be the same number as axisCount in the 'fvar' table.
    pub axis_count: u16,
    /// The number of variation region tables in the variation region list. Must be less than 32,768.
    pub region_count: u16,
    data: &'a [u8],
}

impl<'a> VariationRegionList<'a> {
    pub fn iter_variation_regions(&self) -> VariationRegionIter<'a> {
        VariationRegionIter::new(self.data, self.region_count, self.axis_count)
    }
}

pub struct VariationRegionIter<'a> {
    stream: Stream<'a>,
    region_count: u16,
    axis_count: u16,
}

impl<'a> VariationRegionIter<'a> {
    pub(crate) fn new(bytes: &'a [u8], region_count: u16, axis_count: u16) -> Self {
        Self {
            stream: Stream::new(bytes),
            region_count,
            axis_count,
        }
    }
}

impl<'a> Iterator for VariationRegionIter<'a> {
    type Item = VariationRegion<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.region_count == 0 {
            return None;
        }
        self.region_count -= 1;
        let region_axes = self.stream.parse_slice(self.axis_count as usize)?;
        Some(VariationRegion { region_axes })
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VariationRegion<'a> {
    /// Array of region axis coordinates records, in the order of axes given in the 'fvar' table.
    pub region_axes: &'a [RegionAxisCoordinates],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct RegionAxisCoordinates {
    /// The region start coordinate value for the current axis.
    pub start_coord: F2Dot14BE,
    /// The region peak coordinate value for the current axis.
    pub peak_coord: F2Dot14BE,
    /// The region end coordinate value for the current axis.
    pub end_coord: F2Dot14BE,
}

pub(crate) struct ClassDefinitionParser<'a, C: Copy> {
    stream: Stream<'a>,
    _pd: PhantomData<C>,
}

impl<'a, C: Copy> ClassDefinitionParser<'a, C> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            _pd: PhantomData,
        }
    }

    pub(crate) fn parse(&mut self) -> Option<ClassDefinitionTable<'a, C>> {
        let format = self.stream.parse_u16()?;
        match format {
            1 => {
                let start_glyph_id = self.stream.parse_u16()?;
                let glyph_count = self.stream.parse_u16()?;
                let class_values = self.stream.parse_slice(glyph_count as usize)?;
                Some(ClassDefinitionTable::ClassDefinitionFormat1(
                    ClassDefFormat1 {
                        format,
                        start_glyph_id,
                        glyph_count,
                        class_values,
                    },
                ))
            }
            2 => {
                let class_range_count = self.stream.parse_u16()?;
                let class_range_records = self.stream.parse_slice(class_range_count as usize)?;
                Some(ClassDefinitionTable::ClassDefinitionFormat2(
                    ClassDefFormat2 {
                        format,
                        class_range_count,
                        class_range_records,
                    },
                ))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClassDefinitionTable<'a, C: Copy> {
    ClassDefinitionFormat1(ClassDefFormat1<'a, C>),
    ClassDefinitionFormat2(ClassDefFormat2<'a, C>),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClassDefFormat1<'a, C: Copy> {
    /// Format identifier — format = 1.
    pub format: u16,
    /// First glyph ID assigned to a class.
    pub start_glyph_id: u16,
    /// Number of elements in the classValues array.
    pub glyph_count: u16,
    /// Array of class values — one per glyph ID.
    pub class_values: &'a [C],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClassDefFormat2<'a, C: Copy> {
    /// Format identifier — format = 2.
    pub format: u16,
    /// Number of ClassRange records.
    pub class_range_count: u16,
    /// Array of ClassRangeRecords — ordered by startGlyphID.
    pub class_range_records: &'a [ClassRange<C>],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct ClassRange<C: Copy> {
    /// First glyph ID in the range.
    pub start_glyph_id: C,
    /// Last glyph ID in the range.
    pub end_glyph_id: C,
    /// Applied to all glyphs in the range.
    pub class: C,
}

pub(crate) struct CoverageParser<'a> {
    stream: Stream<'a>,
}

impl<'a> CoverageParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<CoverageTable<'a>> {
        let format = self.stream.parse_u16()?;
        match format {
            1 => {
                let glyph_count = self.stream.parse_u16()?;
                let glyph_array = self.stream.parse_slice(glyph_count as usize)?;
                Some(CoverageTable::CoverageFormat1(CoverageFormat1 {
                    format,
                    glyph_count,
                    glyph_array,
                }))
            }
            2 => {
                let range_count = self.stream.parse_u16()?;
                let range_records = self.stream.parse_slice(range_count as usize)?;
                Some(CoverageTable::CoverageFormat2(CoverageFormat2 {
                    format,
                    range_count,
                    range_records,
                }))
            }
            _ => None,
        }
    }
}

/// Each subtable in a Lookup table (except an Extension lookup type subtable) references a Coverage table, which specifies all the glyphs affected by a substitution or positioning operation described in the subtable. The GSUB, GPOS, and GDEF tables rely on this notion of coverage. If a glyph does not appear in a Coverage table, the client can skip that subtable and move immediately to the next subtable.
///
/// A Coverage table identifies glyphs by glyph IDs in either of two ways:
///
/// As a list of individual glyph IDs in the glyph set.
/// As ranges of consecutive glyph IDs. The range format gives one or more start and end glyph ID pairs to denote the consecutive glyphs covered by the table.
///
/// In a Coverage table, a format field specifies the format as an integer: 1 = lists, and 2 = ranges.
///
/// A Coverage table defines a unique index value, the Coverage Index, for each covered glyph. The Coverage Indexes are sequential, from 0 to the number of covered glyphs minus 1. This unique value specifies the position of the covered glyph in the Coverage table. The client uses the Coverage Index to look up values in the subtable for each glyph.
#[derive(Debug, Clone)]
pub enum CoverageTable<'a> {
    CoverageFormat1(CoverageFormat1<'a>),
    CoverageFormat2(CoverageFormat2<'a>),
}

/// Coverage format 1 consists of a format field and a count of covered glyphs, followed by an array of glyph indices (glyphArray). The glyph indices must be in numerical order for binary searching of the list. When a glyph is found in the Coverage table, its position in the glyphArray determines the Coverage Index that is returned — the first glyph has a Coverage Index = 0, and the last glyph has a Coverage Index = GlyphCount -1.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CoverageFormat1<'a> {
    /// Format identifier — format = 1.
    pub format: u16,
    /// Number of glyphs in the glyph array.
    pub glyph_count: u16,
    /// Array of glyph IDs — in numerical order.
    pub glyph_array: &'a [U16BE],
}

/// Format 2 consists of a format field and a count of glyph index ranges, followed by an array of records (rangeRecords). Each RangeRecord consists of a start glyph index, an end glyph index, and the Coverage Index associated with the range’s start glyph. Ranges must be in startGlyphID order, and they must be distinct, with no overlapping.
///
/// The Coverage Indexes for the first range begin with zero (0) and increase sequentially to (endGlyphId - startGlyphId). For each successive range, the starting Coverage Index is one greater than the ending Coverage Index of the preceding range. Thus, startCoverageIndex for each non-initial range must equal the length of the preceding range (endGlyphID - startGlyphID + 1) added to the startCoverageIndex of the preceding range. This allows for a quick calculation of the Coverage Index for any glyph in any range using the formula: Coverage Index (glyphID) = startCoverageIndex + glyphID - startGlyphID.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CoverageFormat2<'a> {
    /// Format identifier — format = 2.
    pub format: u16,
    /// Number of RangeRecords.
    pub range_count: u16,
    /// Array of glyph ranges — ordered by startGlyphID.
    pub range_records: &'a [RangeRecord],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct RangeRecord {
    /// First glyph ID in the range.
    pub start_glyph_id: U16BE,
    /// First glyph ID in the range.
    pub end_glyph_id: U16BE,
    /// Coverage Index of first glyph ID in range.
    pub glyph_coverage_index: U16BE,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScriptList<'a> {
    /// Number of ScriptRecords
    pub script_count: u16,
    /// Array of ScriptRecords, listed alphabetically by script tag
    pub script_records: &'a [ScriptRecord],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct ScriptRecord {
    /// 4-byte script tag identifier
    pub script_tag: Tag,
    /// Offset to Script table, from beginning of ScriptList
    pub script_offset: U16BE,
}

pub(crate) struct ScriptListParser<'a> {
    stream: Stream<'a>,
}

impl<'a> ScriptListParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<ScriptList<'a>> {
        let script_count = self.stream.parse_u16()?;
        let script_records = self.stream.parse_slice(script_count as usize)?;
        Some(ScriptList {
            script_count,
            script_records,
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FeatureList<'a> {
    /// Number of records in the featureRecords array.
    pub feature_count: u16,
    /// Array of FeatureRecords.
    pub feature_records: &'a [FeatureRecord],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct FeatureRecord {
    /// 4-byte feature identification tag.
    pub feature_tag: Tag,
    /// Offset to Feature table, from beginning of FeatureList.
    pub feature_offset: U16BE,
}

pub(crate) struct FeatureListParser<'a> {
    stream: Stream<'a>,
}

impl<'a> FeatureListParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<FeatureList<'a>> {
        let feature_count = self.stream.parse_u16()?;
        let feature_records = self.stream.parse_slice(feature_count as usize)?;
        Some(FeatureList {
            feature_count,
            feature_records,
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LookupList<'a> {
    /// Number of elements in the lookupOffsets array.
    pub lookup_count: u16,
    /// Array of offsets to Lookup tables, from beginning of LookupList.
    pub lookup_offsets: &'a [U16BE],
}

pub(crate) struct LookupListParser<'a> {
    stream: Stream<'a>,
}

impl<'a> LookupListParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<LookupList<'a>> {
        let lookup_count = self.stream.parse_u16()?;
        let lookup_offsets = self.stream.parse_slice(lookup_count as usize)?;
        Some(LookupList {
            lookup_count,
            lookup_offsets,
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FeatureVariations<'a> {
    /// Major version of the FeatureVariations table — set to 1.
    pub major_version: u16,
    /// Minor version of the FeatureVariations table — set to 0.
    pub minor_version: u16,
    /// Number of feature variation records.
    pub feature_variation_record_count: u32,
    /// Array of feature variation records.
    pub feature_variation_records: &'a [FeatureVariationRecord],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct FeatureVariationRecord {
    /// Offset to a condition set table, from beginning of FeatureVariations table.
    pub condition_set_offset: U32BE,
    /// Offset to a feature table substitution table, from beginning of the FeatureVariations table.
    pub feature_table_substitution_offset: U32BE,
}

pub(crate) struct FeatureVariationsParser<'a> {
    stream: Stream<'a>,
}

impl<'a> FeatureVariationsParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<FeatureVariations<'a>> {
        let major_version = self.stream.parse_u16()?;
        let minor_version = self.stream.parse_u16()?;
        let feature_variation_record_count = self.stream.parse_u32()?;
        let feature_variation_records = self
            .stream
            .parse_slice(feature_variation_record_count as usize)?;
        Some(FeatureVariations {
            major_version,
            minor_version,
            feature_variation_record_count,
            feature_variation_records,
        })
    }
}
