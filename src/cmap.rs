use crate::{
    endian::{I16BE, U16BE, U24BE, U32BE},
    parser::{Parser, TableRecord},
};

pub(crate) struct CmapParser<'a> {
    stream: crate::stream::Stream<'a>,
}

impl<'a> CmapParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: crate::stream::Stream::new(bytes),
        }
    }

    fn parse(&mut self) -> Option<CmapHeader<'a>> {
        let version = self.stream.parse_u16()?;
        let num_tables = self.stream.parse_u16()?;
        let encoding_records = self.stream.parse_slice(num_tables as usize)?;
        Some(CmapHeader {
            version,
            num_tables,
            encoding_records,
            bytes: self.stream.bytes,
        })
    }

    fn parse_encoding_record(&mut self, input: EncodingRecord) -> Option<CmapSubtable<'a>> {
        let bytes = &self.stream.bytes[input.subtable_offset.into_u32() as usize..];
        let mut parser = Self::new(bytes);
        let format = parser.stream.parse_u16()?;
        match format {
            0 => Some(CmapSubtable::ByteEncodingTable(
                parser.parse_byte_encoding_table(format)?,
            )),
            2 => Some(CmapSubtable::HighByteMappingThroughTable(
                parser.parse_high_byte_mapping_through_table(format)?,
            )),
            4 => Some(CmapSubtable::SegmentMappingToDeltaValues(
                parser.parse_segment_mapping_to_delta_values(format)?,
            )),
            6 => Some(CmapSubtable::TrimmedTableMapping(
                parser.parse_trimmed_table_mapping(format)?,
            )),
            8 => Some(CmapSubtable::Mixed16BitAnd32BitCoverage(
                parser.parse_mixed_16_bit_and_32_bit_coverage(format)?,
            )),
            10 => Some(CmapSubtable::TrimmedArray(
                parser.parse_trimmed_array(format)?,
            )),
            12 => Some(CmapSubtable::SegmentedCoverage(
                parser.parse_segmented_coverage(format)?,
            )),
            13 => Some(CmapSubtable::ManyToOneRangeMappings(
                parser.parse_many_to_one_range_mappings(format)?,
            )),
            14 => Some(CmapSubtable::UnicodeVariationSequences(
                parser.parse_unicode_variation_sequences(format)?,
            )),
            _ => None,
        }
    }

    fn parse_unicode_variation_sequences(
        &mut self,
        format: u16,
    ) -> Option<CmapUnicodeVariationSequences<'a>> {
        let length = self.stream.parse_u32()?;
        let num_var_selector_records = self.stream.parse_u32()?;
        let var_selector = self.stream.parse_slice(num_var_selector_records as usize)?;
        Some(CmapUnicodeVariationSequences {
            format,
            length,
            num_var_selector_records,
            var_selector,
            bytes: self.stream.bytes,
        })
    }

    fn parse_many_to_one_range_mappings(
        &mut self,
        format: u16,
    ) -> Option<CmapManyToOneRangeMappings<'a>> {
        let reserved = self.stream.parse_u16()?;
        let length = self.stream.parse_u32()?;
        let language = self.stream.parse_u32()?;
        let num_groups = self.stream.parse_u32()?;
        let groups = self.stream.parse_slice(num_groups as usize)?;
        Some(CmapManyToOneRangeMappings {
            format,
            reserved,
            length,
            language,
            num_groups,
            groups,
            bytes: self.stream.bytes,
        })
    }

    fn parse_segmented_coverage(&mut self, format: u16) -> Option<CmapSegmentedCoverage<'a>> {
        let reserved = self.stream.parse_u16()?;
        let length = self.stream.parse_u32()?;
        let language = self.stream.parse_u32()?;
        let num_groups = self.stream.parse_u32()?;
        let groups = self.stream.parse_slice(num_groups as usize)?;
        Some(CmapSegmentedCoverage {
            format,
            reserved,
            length,
            language,
            num_groups,
            groups,
        })
    }

    fn parse_trimmed_array(&mut self, format: u16) -> Option<CmapTrimmedArray<'a>> {
        let reserved = self.stream.parse_u16()?;
        let length = self.stream.parse_u32()?;
        self.stream.bytes = &self.stream.bytes[..length as usize];
        let language = self.stream.parse_u32()?;
        let start_char_code = self.stream.parse_u32()?;
        let num_chars = self.stream.parse_u32()?;
        let glyph_id_array = self.stream.parse_slice_rest();
        Some(CmapTrimmedArray {
            format,
            reserved,
            length,
            language,
            start_char_code,
            num_chars,
            glyph_id_array,
        })
    }

    fn parse_mixed_16_bit_and_32_bit_coverage(
        &mut self,
        format: u16,
    ) -> Option<CmapMixed16BitAnd32BitCoverage<'a>> {
        let reserved = self.stream.parse_u16()?;
        let length = self.stream.parse_u32()?;
        let language = self.stream.parse_u32()?;
        let is_32 = self.stream.parse_slice(8192)?;
        let num_groups = self.stream.parse_u32()?;
        let groups = self.stream.parse_slice(num_groups as usize)?;
        Some(CmapMixed16BitAnd32BitCoverage {
            format,
            reserved,
            length,
            language,
            is_32,
            num_groups,
            groups,
            bytes: self.stream.bytes,
        })
    }

    fn parse_trimmed_table_mapping(&mut self, format: u16) -> Option<CmapTrimmedTableMapping<'a>> {
        let length = self.stream.parse_u16()?;
        let language = self.stream.parse_u16()?;
        let first_code = self.stream.parse_u16()?;
        let entry_count = self.stream.parse_u16()?;
        let glyph_id_array = self.stream.parse_slice(entry_count as usize)?;
        Some(CmapTrimmedTableMapping {
            format,
            length,
            language,
            first_code,
            entry_count,
            glyph_id_array,
            bytes: self.stream.bytes,
        })
    }

    fn parse_segment_mapping_to_delta_values(
        &mut self,
        format: u16,
    ) -> Option<CmapSegmentMappingToDeltaValues<'a>> {
        let length = self.stream.parse_u16()?;
        self.stream.bytes = &self.stream.bytes[..length as usize];
        let language = self.stream.parse_u16()?;
        let seg_count_x_2 = self.stream.parse_u16()?;
        let seg_count = (seg_count_x_2 / 2) as usize;
        let search_range = self.stream.parse_u16()?;
        let entry_selector = self.stream.parse_u16()?;
        let range_shift = self.stream.parse_u16()?;
        let end_code = self.stream.parse_slice(seg_count)?;
        let reserved_pad = self.stream.parse_u16()?;
        let start_code = self.stream.parse_slice(seg_count)?;
        let id_delta = self.stream.parse_slice(seg_count)?;
        let id_range_offset = self.stream.parse_slice(seg_count)?;
        let glyph_id_array = self.stream.parse_slice_rest();
        Some(CmapSegmentMappingToDeltaValues {
            format,
            length,
            language,
            seg_count_x_2,
            search_range,
            entry_selector,
            range_shift,
            end_code,
            reserved_pad,
            start_code,
            id_delta,
            id_range_offset,
            glyph_id_array,
            bytes: self.stream.bytes,
        })
    }

    fn parse_high_byte_mapping_through_table(
        &mut self,
        format: u16,
    ) -> Option<CmapHighByteMappingThroughTable<'a>> {
        let length = self.stream.parse_u16()?;
        self.stream.bytes = &self.stream.bytes[..length as usize];
        let language = self.stream.parse_u16()?;
        let sub_header_keys: &[U16BE] = self.stream.parse_slice(256)?;
        let sub_header_count = sub_header_keys.iter().map(|x| x.into_u16()).max().unwrap() / 8 + 1;
        let sub_headers = self.stream.parse_slice(sub_header_count as usize)?;
        let glyph_id_array = self.stream.parse_slice_rest();
        Some(CmapHighByteMappingThroughTable {
            format,
            length,
            language,
            sub_header_keys,
            sub_headers,
            glyph_id_array,
            bytes: self.stream.bytes,
        })
    }

    fn parse_byte_encoding_table(&mut self, format: u16) -> Option<CmapByteEncodingTable<'a>> {
        let length = core::cmp::min(256, self.stream.parse_u16()?);
        let language = self.stream.parse_u16()?;
        let glyph_id_array = self.stream.parse_slice(256)?;
        Some(CmapByteEncodingTable {
            format,
            length,
            language,
            glyph_id_array,
        })
    }

    fn parse_default_uvs_table(&mut self) -> Option<DefaultUVS<'a>> {
        let num_unicode_value_ranges = self.stream.parse_u32()?;
        let ranges = self.stream.parse_slice(num_unicode_value_ranges as usize)?;
        Some(DefaultUVS {
            num_unicode_value_ranges,
            ranges,
        })
    }

    fn parse_non_default_uvs_table(&mut self) -> Option<NonDefaultUVS<'a>> {
        let num_uvs_mappings = self.stream.parse_u32()?;
        let uvs_mappings = self.stream.parse_slice(num_uvs_mappings as usize)?;
        Some(NonDefaultUVS {
            num_uvs_mappings,
            uvs_mappings,
        })
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum CmapSubtable<'a> {
    ByteEncodingTable(CmapByteEncodingTable<'a>),
    HighByteMappingThroughTable(CmapHighByteMappingThroughTable<'a>),
    SegmentMappingToDeltaValues(CmapSegmentMappingToDeltaValues<'a>),
    TrimmedTableMapping(CmapTrimmedTableMapping<'a>),
    Mixed16BitAnd32BitCoverage(CmapMixed16BitAnd32BitCoverage<'a>),
    TrimmedArray(CmapTrimmedArray<'a>),
    SegmentedCoverage(CmapSegmentedCoverage<'a>),
    ManyToOneRangeMappings(CmapManyToOneRangeMappings<'a>),
    UnicodeVariationSequences(CmapUnicodeVariationSequences<'a>),
}

/// Format 0 was the standard mapping subtable used on older Macintosh platforms but is not required on newer Apple platforms.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapByteEncodingTable<'a> {
    /// Format number is set to 0.
    pub format: u16,
    /// This is the length in bytes of the subtable.
    pub length: u16,
    ///  For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u16,
    /// An array that maps character codes to glyph index values.
    pub glyph_id_array: &'a [u8],
}

/// This subtable format was created for “double-byte” encodings following national character code standards used for Japanese, Chinese, and Korean characters. These code standards use a mixed 8-/16-bit encoding. This format is not commonly used today.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapHighByteMappingThroughTable<'a> {
    /// Format number is set to 2.
    pub format: u16,
    /// This is the length in bytes of the subtable.
    pub length: u16,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u16,
    /// Array that maps high bytes into the subHeaders array: value is subHeaders index × 8.
    pub sub_header_keys: &'a [U16BE],
    /// Variable-length array of SubHeader records.
    pub sub_headers: &'a [SubHeader],
    ///  Variable-length array containing sub-arrays used for mapping the low byte of 2-byte characters.
    pub glyph_id_array: &'a [U16BE],
    bytes: &'a [u8],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SubHeader {
    /// First valid low byte for this SubHeader.
    pub first_code: U16BE,
    /// First valid low byte for this SubHeader.
    pub entry_count: U16BE,
    pub id_delta: I16BE,
    pub id_range_offset: U16BE,
}

/// This is the standard character-to-glyph-index mapping subtable for fonts that support only Unicode Basic Multilingual Plane characters (U+0000 to U+FFFF).
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapSegmentMappingToDeltaValues<'a> {
    /// Format number is set to 4.
    pub format: u16,
    /// This is the length in bytes of the subtable.
    pub length: u16,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u16,
    /// 2 × segCount.
    pub seg_count_x_2: u16,
    /// Maximum power of 2 less than or equal to segCount, times 2 ((2**floor(log2(segCount))) * 2, where “**” is an exponentiation operator)
    pub search_range: u16,
    /// Log2 of the maximum power of 2 less than or equal to segCount (log2(searchRange/2), which is equal to floor(log2(segCount)))
    pub entry_selector: u16,
    /// segCount times 2, minus searchRange ((segCount * 2) - searchRange)
    pub range_shift: u16,
    /// End characterCode for each segment, last=0xFFFF.
    pub end_code: &'a [U16BE],
    /// Set to 0.
    pub reserved_pad: u16,
    /// Start character code for each segment.
    pub start_code: &'a [U16BE],
    /// Delta for all character codes in segment.
    pub id_delta: &'a [I16BE],
    /// Offsets into glyphIdArray or 0
    pub id_range_offset: &'a [I16BE],
    /// Glyph index array (arbitrary length)
    pub glyph_id_array: &'a [U16BE],

    bytes: &'a [u8],
}

/// Format 6 was designed to map 16-bit characters to glyph indexes when the character codes for a font fall into a single contiguous range.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapTrimmedTableMapping<'a> {
    /// Format number is set to 6.
    pub format: u16,
    /// This is the length in bytes of the subtable.
    pub length: u16,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u16,
    /// First character code of subrange.
    pub first_code: u16,
    /// Number of character codes in subrange.
    pub entry_count: u16,
    /// Array of glyph index values for character codes in the range.
    pub glyph_id_array: &'a [U16BE],

    bytes: &'a [u8],
}

/// Subtable format 8 was designed to support Unicode supplementary-plane characters in UTF-16 encoding, though it is not commonly used. Format 8 is similar to format 2 in that it provides for mixed-length character codes. Instead of allowing for 8- and 16-bit character codes, however, it allows for 16- and 32-bit character codes.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapMixed16BitAnd32BitCoverage<'a> {
    /// Subtable format; set to 8.
    pub format: u16,
    /// Reserved; set to 0
    pub reserved: u16,

    /// Byte length of this subtable (including the header)
    pub length: u32,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u32,
    /// Tightly packed array of bits (8K bytes total) indicating whether the particular 16-bit (index) value is the start of a 32-bit character code
    pub is_32: &'a [u8],
    /// Number of groupings which follow
    pub num_groups: u32,
    /// Array of SequentialMapGroup records.
    pub groups: &'a [SequentialMapGroup],

    bytes: &'a [u8],
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct SequentialMapGroup {
    /// First character code in this group; note that if this group is for one or more 16-bit character codes (which is determined from the is32 array), this 32-bit value will have the high 16-bits set to zero
    pub start_char_code: U32BE,
    /// Last character code in this group; same condition as listed above for the startCharCode
    pub end_char_code: U32BE,
    /// Glyph index corresponding to the starting character code
    pub start_glyph_id: U32BE,
}

/// Subtable format 10 was designed to support Unicode supplementary-plane characters, though it is not commonly used. Format 10 is similar to format 6 in that it defines a trimmed array for a tight range of character codes. It differs, however, in that it uses 32-bit character codes.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapTrimmedArray<'a> {
    /// Subtable format; set to 10.
    pub format: u16,
    /// Reserved; set to 0
    pub reserved: u16,
    /// Byte length of this subtable (including the header)
    pub length: u32,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u32,
    /// First character code covered
    pub start_char_code: u32,
    /// Number of character codes covered
    pub num_chars: u32,
    /// Array of glyph indices for the character codes covered
    pub glyph_id_array: &'a [U16BE],
}

/// This is the standard character-to-glyph-index mapping subtable for fonts supporting Unicode character repertoires that include supplementary-plane characters (U+10000 to U+10FFFF).
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapSegmentedCoverage<'a> {
    /// Subtable format; set to 12.
    pub format: u16,
    /// Reserved; set to 0
    pub reserved: u16,
    /// Byte length of this subtable (including the header)
    pub length: u32,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u32,
    /// Number of groupings which follow
    pub num_groups: u32,
    /// Array of SequentialMapGroup records.
    pub groups: &'a [SequentialMapGroup],
}

/// This subtable provides for situations in which the same glyph is used for hundreds or even thousands of consecutive characters spanning across multiple ranges of the code space. This subtable format may be useful for “last resort” fonts, although these fonts may use other suitable subtable formats as well. (For “last resort” fonts, see also the 'head' table flags, bit 14.)
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapManyToOneRangeMappings<'a> {
    /// Subtable format; set to 13.
    pub format: u16,
    /// Reserved; set to 0
    pub reserved: u16,
    /// Byte length of this subtable (including the header)
    pub length: u32,
    /// For requirements on use of the language field, see “Use of the language field in 'cmap' subtables” in this document.
    pub language: u32,
    /// Number of groupings which follow
    pub num_groups: u32,
    /// Array of ConstantMapGroup records.
    pub groups: &'a [ConstantMapGroup],

    bytes: &'a [u8],
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct ConstantMapGroup {
    /// First character code in this group
    pub start_char_code: U32BE,
    /// Last character code in this group
    pub end_char_code: U32BE,
    /// Glyph index to be used for all the characters in the group’s range.
    pub glyph_id: U32BE,
}

/// Subtable format 14 specifies the Unicode variation sequences (UVSes) supported by the font. A variation sequence, according to the Unicode Standard, comprises a base character followed by a variation selector. For example, <U+82A6, U+E0101>.
#[repr(C)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CmapUnicodeVariationSequences<'a> {
    /// Subtable format; set to 14.
    pub format: u16,
    /// Byte length of this subtable (including this header)
    pub length: u32,
    /// Number of variation Selector Records
    pub num_var_selector_records: u32,
    /// Array of VariationSelector records.
    pub var_selector: &'a [VariationSelector],

    bytes: &'a [u8],
}

impl<'a> CmapUnicodeVariationSequences<'a> {
    pub fn parse_default_uvs_table(&self, input: VariationSelector) -> Option<DefaultUVS<'a>> {
        let bytes = &self.bytes[input.default_uvs_offset.into_u32() as usize..];
        let mut parser = CmapParser::new(bytes);
        parser.parse_default_uvs_table()
    }

    pub fn parse_non_default_uvs_table(
        &self,
        input: VariationSelector,
    ) -> Option<NonDefaultUVS<'a>> {
        let bytes = &self.bytes[input.non_default_uvs_offset.into_u32() as usize..];
        let mut parser = CmapParser::new(bytes);
        parser.parse_non_default_uvs_table()
    }
}

/// A Default UVS table is simply a range-compressed list of Unicode scalar values, representing the base characters of the default UVSes which use the varSelector of the associated VariationSelector record.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct DefaultUVS<'a> {
    /// Number of Unicode character ranges.
    pub num_unicode_value_ranges: u32,
    /// Array of UnicodeRange records.
    pub ranges: &'a [UnicodeRange],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct UnicodeRange {
    /// First value in this range
    pub start_unicode_value: U24BE,
    /// Number of additional values in this range
    pub additional_count: u8,
}

/// A Non-Default UVS table is a list of pairs of Unicode scalar values and glyph IDs. The Unicode values represent the base characters of all non-default UVSes which use the varSelector of the associated VariationSelector record, and the glyph IDs specify the glyph IDs to use for the UVSes.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct NonDefaultUVS<'a> {
    /// Number of UVS Mappings that follow
    pub num_uvs_mappings: u32,
    /// Array of UVSMapping records.
    pub uvs_mappings: &'a [UVSMapping],
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct UVSMapping {
    /// Base Unicode value of the UVS
    pub unicode_value: U24BE,
    /// Glyph ID of the UVS
    pub glyph_id: U16BE,
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct VariationSelector {
    pub var_selector: U24BE,
    pub default_uvs_offset: U32BE,
    pub non_default_uvs_offset: U32BE,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CmapHeader<'a> {
    pub version: u16,
    pub num_tables: u16,
    pub encoding_records: &'a [EncodingRecord],
    bytes: &'a [u8],
}

impl<'a> CmapHeader<'a> {
    pub fn parse_encoding_record(&self, record: EncodingRecord) -> Option<CmapSubtable<'a>> {
        let mut parser = CmapParser::new(self.bytes);
        parser.parse_encoding_record(record)
    }
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct EncodingRecord {
    /// Platform ID.
    pub platform_id: U16BE,
    /// Platform-specific encoding ID.
    pub encoding_id: U16BE,
    /// Byte offset from beginning of table to the subtable for this encoding.
    pub subtable_offset: U32BE,
}

impl EncodingRecord {
    #[inline]
    pub fn is_unicode(&self) -> bool {
        self.platform_id.into_u16() == 0
    }
    #[inline]
    pub fn is_macintosh(&self) -> bool {
        self.platform_id.into_u16() == 1
    }
    #[inline]
    pub fn is_iso(&self) -> bool {
        self.platform_id.into_u16() == 2
    }
    #[inline]
    pub fn is_windows(&self) -> bool {
        self.platform_id.into_u16() == 3
    }
    #[inline]
    pub fn is_custom(&self) -> bool {
        self.platform_id.into_u16() == 4
    }
    /// Unicode 1.0 semantics—deprecated
    #[inline]
    pub fn unicode_is_unicode_v1_0(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 0
    }
    /// Unicode 1.1 semantics—deprecated
    #[inline]
    pub fn unicode_is_unicode_v_1_1(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 1
    }
    /// ISO/IEC 10646 semantics—deprecated
    #[inline]
    pub fn unicode_is_iso_10646(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 2
    }
    /// Unicode 2.0 and onwards semantics, Unicode BMP only
    #[inline]
    pub fn unicode_is_unicode_v2_0_bmp_only(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 3
    }
    /// Unicode 2.0 and onwards semantics, Unicode full repertoire
    #[inline]
    pub fn unicode_is_unicode_v2_0_full_repertoire(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 4
    }
    /// Unicode variation sequences—for use with subtable format 14
    #[inline]
    pub fn unicode_is_unicode_variation_sequences(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 5
    }
    /// Unicode full repertoire—for use with subtable format 13
    #[inline]
    pub fn unicode_is_unicode_full_repertoire(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 6
    }
    /// 7-bit ASCII
    #[inline]
    pub fn iso_is_7_bit_ascii(&self) -> bool {
        self.is_iso() && self.encoding_id.into_u16() == 0
    }
    /// ISO 10646
    #[inline]
    pub fn iso_is_iso_10646(&self) -> bool {
        self.is_iso() && self.encoding_id.into_u16() == 1
    }
    /// ISO 8859-1
    #[inline]
    pub fn iso_is_iso_8859_1(&self) -> bool {
        self.is_iso() && self.encoding_id.into_u16() == 2
    }
    /// Symbol
    #[inline]
    pub fn windows_is_symbol(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 0
    }
    /// Unicode BMP
    #[inline]
    pub fn windows_is_unicode_bmp(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 1
    }
    /// ShiftJIS
    #[inline]
    pub fn windows_is_shiftjis(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 2
    }
    /// PRC
    #[inline]
    pub fn windows_is_prc(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 3
    }
    /// Big5
    #[inline]
    pub fn windows_is_big5(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 4
    }
    /// Wansung
    #[inline]
    pub fn windows_is_wansung(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 5
    }
    /// Johab
    #[inline]
    pub fn windows_is_johab(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 6
    }
    /// Unicode full repertoire
    #[inline]
    pub fn windows_is_unicode_full_repertoire(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 10
    }
    /// OTF Windows NT compatibility mapping
    #[inline]
    pub fn custom_is_windows_nt_compatibility_mapping(&self) -> bool {
        self.is_custom() && self.encoding_id.into_u16() <= 255
    }
}

impl<'a> Parser<'a> {
    pub fn parse_cmap(&self, input: TableRecord) -> Option<CmapHeader<'a>> {
        if input.table_tag.is_cmap() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = CmapParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
