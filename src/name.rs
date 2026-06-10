use crate::{
    endian::{U16BE, UTF16BE},
    parser::{Parser, TableRecord},
    stream::Stream,
    util::slice_range,
};

/// The naming table allows multilingual strings to be associated with the OpenType™ font. These strings can represent copyright notices, font names, family names, style names, and so on. To keep this table short, the font manufacturer may wish to make a limited set of entries in some small set of languages; later, the font can be “localized” and the strings translated or added. Other parts of the OpenType font that require these strings can refer to them using a language-independent name ID. In addition to language variants, the table also allows for platform-specific character-encoding variants. Applications that need a particular string can look it up by its platform ID, encoding ID, language ID and name ID. Note that different platforms may have different requirements for the encoding of strings.
#[derive(Debug, Clone)]
pub enum NameTable<'a> {
    Version0(NameTableVersion0<'a>),
    Version1(NameTableVersion1<'a>),
}

impl<'a> NameTable<'a> {
    pub const fn parse_name_record(&self, input: NameRecord) -> UTF16BE<'a> {
        match self {
            Self::Version0(t) => t.parse_name_record(input),
            Self::Version1(t) => t.parse_name_record(input),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct NameTableVersion0<'a> {
    /// Table version number (=0).
    pub version: u16,
    /// Number of name records.
    pub count: u16,
    /// Offset to start of string storage (from start of table).
    pub storage_offset: u16,
    /// The name records where count is the number of records.
    pub name_record: &'a [NameRecord],
    storage: &'a [u8],
}

impl<'a> NameTableVersion0<'a> {
    pub const fn parse_name_record(&self, input: NameRecord) -> UTF16BE<'a> {
        let offset = input.string_offset.into_u16() as usize;
        let length = input.length.into_u16() as usize;
        let bytes = slice_range(self.storage, offset..offset + length);
        UTF16BE::new(bytes)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct NameTableVersion1<'a> {
    /// Table version number (=1).
    pub version: u16,
    /// Number of name records.
    pub count: u16,
    /// Offset to start of string storage (from start of table).
    pub storage_offset: u16,
    /// The name records where count is the number of records.
    pub name_record: &'a [NameRecord],
    /// Number of language-tag records.
    pub lang_tag_count: u16,
    /// The language-tag records where langTagCount is the number of records.
    pub lang_tag_record: &'a [LangTagRecord],
    storage: &'a [u8],
}

impl<'a> NameTableVersion1<'a> {
    pub const fn parse_name_record(&self, input: NameRecord) -> UTF16BE<'a> {
        let offset = input.string_offset.into_u16() as usize;
        let length = input.length.into_u16() as usize;
        let bytes = slice_range(self.storage, offset..offset + length);
        UTF16BE::new(bytes)
    }

    pub const fn parse_lang_tag_record(&self, input: LangTagRecord) -> UTF16BE<'a> {
        let offset = input.lang_tag_offset.into_u16() as usize;
        let length = input.length.into_u16() as usize;
        let bytes = slice_range(self.storage, offset..offset + length);
        UTF16BE::new(bytes)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct LangTagRecord {
    pub length: U16BE,
    pub lang_tag_offset: U16BE,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct NameRecord {
    pub platform_id: U16BE,
    pub encoding_id: U16BE,
    pub language_id: U16BE,
    pub name_id: U16BE,
    pub length: U16BE,
    pub string_offset: U16BE,
}

impl NameRecord {
    /// Copyright notice.
    #[inline]
    pub const fn is_copyright_notice(&self) -> bool {
        self.name_id.into_u16() == 0
    }

    /// Font Family name. The Font Family name is used in combination with Font Subfamily name (name ID 2), and should be shared among at most four fonts that differ only in weight or style (italic/oblique), as described below.
    ///
    /// This four-way distinction should also be reflected in the OS/2.fsSelection field, using bits 0 and 5.
    ///
    /// While some platforms or applications do not have this constraint, many existing applications that use this pair of names assume that a Font Family name is shared by at most four fonts that form a font style-linking group: regular, italic (or oblique), bold, and bold italic (or bold oblique). To be compatible with the broadest range of platforms and applications, it is strongly recommended that fonts limit use of Font Family name in this manner.
    ///
    /// For extended typographic families that includes fonts other than the four basic styles (regular, italic, bold, bold italic), it is strongly recommended that name IDs 16 and 17 be used in fonts to create an extended, typographic grouping. (See examples provided below.)
    ///
    /// It is also strongly recommended that applications support extended typographic-family groupings using name IDs 16 and 17. Note that variable fonts can include a large number of named instances, each of which will use a shared typographic family name (name ID 16) and will have a typographic subfamily name (equivalent to name ID 17). Applications that assume a four-style family grouping based on name IDs 1 and 2 are likely to provide a poor user experience with variable fonts.
    ///
    /// For fonts within an extended typographic family that fall outside the basic four-way distinction, the distinguishing attributes should be reflected in the Font Family name so that those fonts appear as a separate font family in applications that support only four-member families. For example, the Font Family name for the Arial Narrow font is “Arial Narrow”; the Font Family name for the Arial Black font is “Arial Black”. Note that, in such cases, name ID 16 should also be included with a shared name (e.g., "Arial") that reflects the full, typographic family.
    #[inline]
    pub const fn is_font_family_name(&self) -> bool {
        self.name_id.into_u16() == 1
    }

    ///Font Subfamily name. The Font Subfamily is used in combination with Font Family name (name ID 1), and distinguishes the fonts in a group with the same Font Family name. This should be used for weight and style (italic/oblique) variants only, as described below.
    ///
    /// This four-way distinction should also be reflected in the OS/2.fsSelection field, using bits 0 and 5.
    ///
    /// While some platforms or applications do not have this constraint, many existing applications that use name IDs 1 and 2 assume that a Font Family name is shared by at most four fonts that form a font style-linking group, and that Font Subfamily names would reflect one of the four basic styles, regular, italic (or oblique), bold, and bold italic (or bold oblique). To be compatible with the broadest range of platforms and applications, it is strongly recommended that fonts should limit use of Font Subfamily in this manner.
    ///
    /// For extended typographic families that includes fonts other than the four basic styles (regular, italic, bold, bold italic), it is strongly recommended that name IDs 16 and 17 be used in fonts to create an extended, typographic grouping.
    ///
    /// Within an extended typographic family that includes fonts beyond regular, bold, italic, or bold italic, distinctions for these other fonts are made in the Font Family name so that fonts appear to be in separate families. In some cases, this can lead to specifying a Subfamily name of “Regular” for a font that might not otherwise be considered a regular font. For example, the Arial Black font has a Font Family name of “Arial Black” and a Subfamily name of “Regular”. Note that, in such cases, name IDs 16 and 17 should also be included, using a shared value for name ID 16 (e.g., "Arial") that reflects the full typographic family, and values for name ID 17 that appropriately reflect the actual design variant of each font.
    ///
    /// Fonts that are not part of an extended typographic family and with no distinctive weight or style (e.g., medium weight, not italic) should use "Regular" as the Font Subfamily name (for English).
    #[inline]
    pub const fn is_font_subfamily_name(&self) -> bool {
        self.name_id.into_u16() == 2
    }

    /// Unique font identifier.
    #[inline]
    pub const fn is_unique_font_identifier(&self) -> bool {
        self.name_id.into_u16() == 3
    }

    /// Full font name that reflects all family and relevant subfamily descriptors. The full font name is generally a combination of name IDs 1 and 2, or of name IDs 16 and 17, or a similar human-readable variant.
    ///
    /// For fonts in extended typographic families (that is, families that include more than regular, italic, bold, and bold italic variants), values for name IDs 1 and 2 are normally chosen to provide compatibility with certain applications that assume a family has at most four style-linked fonts. In that case, some fonts may end up with a Subfamily name (name ID 2) of “Regular” even though the font would not be considered, typographically, a regular font. For such non-regular fonts in which name ID 2 is specified as “Regular”, the “Regular” descriptor would generally be omitted from name ID 4. For example, the Arial Black font has a Font Family name (name ID 1) of “Arial Black” and a Subfamily name (name ID 2) of “Regular”, but has a full font name (name ID 4) of “Arial Black”. Note that name IDs 16 and 17 should also be included in these fonts, and that name ID 4 would typically be a combination of name IDs 16 and 17, without needing any additional qualifications regarding “Regular”.
    #[inline]
    pub const fn is_full_font_name(&self) -> bool {
        self.name_id.into_u16() == 4
    }

    /// Version string. Should begin with the pattern “Version number.number” (upper case, lower case, or mixed, with a space between “Version” and the number).
    ///
    /// The string must contain a version number of the following form: one or more digits (0-9) of value less than 65,535, followed by a period, followed by one or more digits of value less than 65,535. Any character other than a digit will terminate the minor number. A character such as “;” is helpful to separate different pieces of version information.
    ///
    /// The first such match in the string can be used by installation software to compare font versions. Some installers could require the string to start with “Version ”, followed by a version number as above.
    #[inline]
    pub const fn is_version_string(&self) -> bool {
        self.name_id.into_u16() == 5
    }

    /// PostScript name for the font. Name ID 6 specifies a string which is used to invoke a PostScript language font that corresponds to this OpenType font. When translated to ASCII, the name string must be no longer than 63 characters and restricted to the printable ASCII subset, codes 33 to 126, except for the 10 characters '[', ']', '(', ')', '{', '}', '<', '>', '/', '%'.
    ///
    /// In a CFF OpenType font, there is no requirement that this name be the same as the font name in the CFF’s Name INDEX. Thus, the same CFF may be shared among multiple font components in a Font Collection. See the 'name' table section of “Recommendations for OpenType fonts” for additional information.
    #[inline]
    pub const fn is_postscript_name(&self) -> bool {
        self.name_id.into_u16() == 6
    }

    ///  Trademark. This is used to save any trademark notice/information for this font. Such information should be based on legal advice. This is distinctly separate from the copyright.
    #[inline]
    pub const fn is_trademark(&self) -> bool {
        self.name_id.into_u16() == 7
    }

    /// Manufacturer Name.
    #[inline]
    pub const fn is_manufacturer_name(&self) -> bool {
        self.name_id.into_u16() == 8
    }

    /// Designer. Name of the designer of the typeface.
    #[inline]
    pub const fn is_designer(&self) -> bool {
        self.name_id.into_u16() == 9
    }

    /// Description. Description of the typeface. Can contain revision information, usage recommendations, history, features, etc.
    #[inline]
    pub const fn is_description(&self) -> bool {
        self.name_id.into_u16() == 10
    }

    /// URL of Vendor. URL of font vendor (with protocol, e.g., http://, ftp://). If a unique serial number is embedded in the URL, it can be used to register the font.
    #[inline]
    pub const fn is_vendor_url(&self) -> bool {
        self.name_id.into_u16() == 11
    }

    /// URL of Designer. URL of typeface designer (with protocol, e.g., http://, ftp://).
    #[inline]
    pub const fn is_designer_url(&self) -> bool {
        self.name_id.into_u16() == 12
    }

    /// License Description. Description of the license or licenses under which the font is provided. This could be a reference to a named license agreement (e.g., a common open source licenses), identification of a software-use license under which a font is bundled, information about where to locate an external license (see also name ID 14), a summary of permitted uses, or the full legal text of a license agreement. It is prudent to seek legal advice on the content of this name ID to avoid possible conflict of interpretation between it and the license(s).
    #[inline]
    pub const fn is_license_description(&self) -> bool {
        self.name_id.into_u16() == 13
    }

    /// License Info URL. URL where additional licensing information can be found.
    #[inline]
    pub const fn is_license_info_url(&self) -> bool {
        self.name_id.into_u16() == 14
    }

    /// Typographic Family name. The typographic family grouping doesn’t impose any constraints on the number of faces within it, in contrast with the 4-style family grouping (ID 1), which is present both for historical reasons and to express style linking groups. If name ID 16 is absent, then name ID 1 is considered to be the typographic family name. (In earlier versions of the specification, name ID 16 was known as “Preferred Family”.)
    #[inline]
    pub const fn is_typographic_family_name(&self) -> bool {
        self.name_id.into_u16() == 16
    }

    /// Typographic Subfamily name. This allows font designers to specify a subfamily name within the typographic family grouping. This string must be unique within a particular typographic family. If it is absent, then name ID 2 is considered to be the typographic subfamily name. (In earlier versions of the specification, name ID 17 was known as “Preferred Subfamily”.)
    #[inline]
    pub const fn is_typographic_subfamily_name(&self) -> bool {
        self.name_id.into_u16() == 17
    }

    /// Compatible Full (Macintosh only). On the Macintosh, the menu name is constructed using the FOND resource. This usually matches the Full Name. If you want the name of the font to appear differently than the Full Name, you can insert the Compatible Full Name in ID 18.
    #[inline]
    pub const fn is_compatible_full(&self) -> bool {
        self.name_id.into_u16() == 18
    }

    /// Sample text. This can be the font name, or any other text that the designer thinks is the best sample to display the font in.
    #[inline]
    pub const fn is_sample_text(&self) -> bool {
        self.name_id.into_u16() == 19
    }

    ///  PostScript CID findfont name. Its presence in a font means that the nameID 6 holds a PostScript font name that is meant to be used with the “composefont” invocation in order to invoke the font in a PostScript interpreter. See the definition of name ID 6.
    ///
    /// The value held in the name ID 20 string is interpreted as a PostScript font name that is meant to be used with the “findfont” invocation, in order to invoke the font in a PostScript interpreter.
    ///
    /// When translated to ASCII, this name string must be restricted to the printable ASCII subset, codes 33 through 126, except for the 10 characters: '[', ']', '(', ')', '{', '}', '<', '>', '/', '%'.
    ///
    /// See “Recommendations for OTF fonts” for additional information
    #[inline]
    pub const fn is_postscript_cid_findfont_name(&self) -> bool {
        self.name_id.into_u16() == 20
    }

    /// WWS Family Name. Used to provide a WWS-conformant family name in case the entries for IDs 16 and 17 do not conform to the WWS model. (That is, in case the entry for ID 17 includes qualifiers for some attribute other than weight, width or slope.) If bit 8 of the OS/2 fsSelection field is set, a WWS Family Name entry should not be needed and should not be included. Conversely, if an entry for this ID is included, bit 8 should not be set. (See OS/2.fsSelection field for details.) Examples of name ID 21: “Minion Pro Caption” and “Minion Pro Display”. (Name ID 16 would be “Minion Pro” for these examples.)
    ///
    /// See additional remarks regarding IDs 21 and 22 below.
    #[inline]
    pub const fn is_wws_family_name(&self) -> bool {
        self.name_id.into_u16() == 21
    }

    /// WWS Subfamily Name. Used in conjunction with ID 21, this ID provides a WWS-conformant subfamily name (reflecting only weight, width and slope attributes) in case the entries for IDs 16 and 17 do not conform to the WWS model. As in the case of ID 21, use of this ID should correlate inversely with bit 8 of the OS/2 fsSelection field being set. Examples of name ID 22: “Semibold Italic”, “Bold Condensed”. (Name ID 17 could be “Semibold Italic Caption”, or “Bold Condensed Display”, for example.)
    ///
    /// See additional remarks regarding IDs 21 and 22 below.
    #[inline]
    pub const fn is_wws_subfamily_name(&self) -> bool {
        self.name_id.into_u16() == 22
    }

    /// Light Background Palette. This ID, if used in the CPAL table’s Palette Labels Array, specifies that the corresponding color palette in the CPAL table is appropriate to use with the font when displaying it on a light background such as white. Strings for this ID are for use as user interface strings associated with this palette.
    #[inline]
    pub const fn is_light_background_palette(&self) -> bool {
        self.name_id.into_u16() == 23
    }

    /// Dark Background Palette. This ID, if used in the CPAL table’s Palette Labels Array, specifies that the corresponding color palette in the CPAL table is appropriate to use with the font when displaying it on a dark background such as black. Strings for this ID are for use as user interface strings associated with this palette.
    #[inline]
    pub const fn is_dark_background_palette(&self) -> bool {
        self.name_id.into_u16() == 24
    }

    /// Variations PostScript Name Prefix. If present in a variable font, it may be used as the family prefix in the PostScript Name Generation for Variation Fonts algorithm. The character set is restricted to ASCII-range uppercase Latin letters, lowercase Latin letters, and digits. All name strings for name ID 25 within a font, when converted to ASCII, must be identical. See Adobe Technical Note #5902: “PostScript Name Generation for Variation Fonts” for reasons to include name ID 25 in a font, and for examples. For general information on OpenType Font Variations, see the chapter, OpenType Font Variations Overview.
    #[inline]
    pub const fn is_variations_postscript_name_prefix(&self) -> bool {
        self.name_id.into_u16() == 25
    }

    #[inline]
    pub const fn is_unicode(&self) -> bool {
        self.name_id.into_u16() == 0
    }

    #[inline]
    pub const fn is_macintosh(&self) -> bool {
        self.name_id.into_u16() == 1
    }

    #[inline]
    pub const fn is_windows(&self) -> bool {
        self.name_id.into_u16() == 3
    }

    /// Unicode 1.0 semantics—deprecated
    #[inline]
    pub const fn is_unicode_1_0_semantics(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 0
    }

    /// Unicode 1.1 semantics—deprecated
    #[inline]
    pub const fn is_unicode_1_1_semantics(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 1
    }

    /// ISO/IEC 10646 semantics—deprecated
    #[inline]
    pub const fn is_unicode_iso_iec_10646_semantics(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 2
    }

    /// Unicode 2.0 and onwards semantics, Unicode BMP only
    #[inline]
    pub const fn is_unicode_2_0_and_onwards_semantics_bmp_only(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 3
    }

    /// Unicode 2.0 and onwards semantics, Unicode full repertoire
    #[inline]
    pub const fn is_unicode_2_0_and_onwards_semantics_unicode_full_repertoire(&self) -> bool {
        self.is_unicode() && self.encoding_id.into_u16() == 4
    }

    /// Roman
    #[inline]
    pub const fn is_macintosh_roman(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 0
    }

    /// Japanese
    #[inline]
    pub const fn is_macintosh_japanese(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 1
    }

    /// Chinese (Traditional)
    #[inline]
    pub const fn is_macintosh_chinese_traditional(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 2
    }

    /// Korean
    #[inline]
    pub const fn is_macintosh_korean(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 3
    }

    /// Arabic
    #[inline]
    pub const fn is_macintosh_arabic(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 4
    }

    /// Hebrew
    #[inline]
    pub const fn is_macintosh_hebrew(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 5
    }

    /// Greek
    #[inline]
    pub const fn is_macintosh_greek(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 6
    }

    /// Russian
    #[inline]
    pub const fn is_macintosh_russian(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 7
    }

    /// RSymbol
    #[inline]
    pub const fn is_macintosh_rsymbol(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 8
    }

    /// Devanagari
    #[inline]
    pub const fn is_macintosh_devanagari(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 9
    }

    /// Gurmukhi
    #[inline]
    pub const fn is_macintosh_gurmukhi(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 10
    }

    /// Gujarati
    #[inline]
    pub const fn is_macintosh_gujarati(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 11
    }

    /// Odia
    #[inline]
    pub const fn is_macintosh_odia(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 12
    }

    /// Bangla
    #[inline]
    pub const fn is_macintosh_bangla(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 13
    }

    /// Tamil
    #[inline]
    pub const fn is_macintosh_tamil(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 14
    }

    /// Telugu
    #[inline]
    pub const fn is_macintosh_telugu(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 15
    }

    /// Kannada
    #[inline]
    pub const fn is_macintosh_kannada(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 16
    }

    /// Malayalam
    #[inline]
    pub const fn is_macintosh_malayalam(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 17
    }

    /// Sinhalese
    #[inline]
    pub const fn is_macintosh_sinhalese(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 18
    }

    /// Burmese
    pub const fn is_macintosh_burmese(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 19
    }

    /// Khmer
    #[inline]
    pub const fn is_macintosh_khmer(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 20
    }

    /// Thai
    #[inline]
    pub const fn is_macintosh_thai(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 21
    }

    /// Laotian
    #[inline]
    pub const fn is_macintosh_laotian(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 22
    }

    /// Georgian
    #[inline]
    pub const fn is_macintosh_georgian(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 23
    }

    /// Armenian
    #[inline]
    pub const fn is_macintosh_armenian(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 24
    }

    /// Chinese (Simplified)
    #[inline]
    pub const fn is_macintosh_chinese_simplified(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 25
    }

    /// Tibetan
    #[inline]
    pub const fn is_macintosh_tibetan(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 26
    }

    /// Mongolian
    #[inline]
    pub const fn is_macintosh_mongolian(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 27
    }

    /// Geez
    #[inline]
    pub const fn is_macintosh_geez(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 28
    }

    /// Slavic
    #[inline]
    pub const fn is_macintosh_slavic(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 29
    }

    /// Vietnamese
    #[inline]
    pub const fn is_macintosh_vietnamese(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 30
    }

    /// Sindhi
    #[inline]
    pub const fn is_macintosh_sindhi(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 31
    }

    /// Uninterpreted
    #[inline]
    pub const fn is_macintosh_uninterpreted(&self) -> bool {
        self.is_macintosh() && self.encoding_id.into_u16() == 31
    }

    /// Symbol
    #[inline]
    pub const fn is_windows_symbol(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 0
    }

    /// Unicode BMP
    #[inline]
    pub const fn is_windows_unicode_bmp(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 1
    }

    /// ShiftJIS
    #[inline]
    pub const fn is_windows_shiftjis(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 2
    }

    /// PRC
    #[inline]
    pub const fn is_windows_prc(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 3
    }

    /// Big5
    #[inline]
    pub const fn is_windows_big5(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 4
    }

    /// Wansung
    #[inline]
    pub const fn is_windows_wansung(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 5
    }

    /// Johab
    #[inline]
    pub const fn is_windows_johab(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 6
    }

    /// Unicode full repertoire
    #[inline]
    pub const fn is_windows_unicode_full_repertoire(&self) -> bool {
        self.is_windows() && self.encoding_id.into_u16() == 10
    }
}

pub(crate) struct NameParser<'a> {
    stream: Stream<'a>,
}

impl<'a> NameParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<NameTable<'a>> {
        let version = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        match version {
            0 => Some(NameTable::Version0(match self.parse_version_0(version) {
                Some(v) => v,
                _ => return None,
            })),
            1 => Some(NameTable::Version1(match self.parse_version_1(version) {
                Some(v) => v,
                _ => return None,
            })),
            _ => None,
        }
    }

    pub(crate) const fn parse_version_0(&mut self, version: u16) -> Option<NameTableVersion0<'a>> {
        let count = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let storage_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let name_record = match self.stream.parse_slice(count as usize) {
            Some(v) => v,
            _ => return None,
        };
        let storage = self.stream.parse_slice_rest();
        Some(NameTableVersion0 {
            version,
            count,
            storage_offset,
            name_record,
            storage,
        })
    }

    pub(crate) const fn parse_version_1(&mut self, version: u16) -> Option<NameTableVersion1<'a>> {
        let count = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let storage_offset = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let name_record = match self.stream.parse_slice(count as usize) {
            Some(v) => v,
            _ => return None,
        };
        let lang_tag_count = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let lang_tag_record = match self.stream.parse_slice(lang_tag_count as usize) {
            Some(v) => v,
            _ => return None,
        };
        let storage = self.stream.parse_slice_rest();
        Some(NameTableVersion1 {
            version,
            count,
            storage_offset,
            name_record,
            lang_tag_count,
            lang_tag_record,
            storage,
        })
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_name(&self, input: TableRecord) -> Option<NameTable<'a>> {
        if input.table_tag.is_name() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = NameParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
