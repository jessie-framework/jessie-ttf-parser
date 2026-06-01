use crate::{
    fword::FWord,
    parser::{Parser, TableRecord, Tag},
    stream::Stream,
    ufword::UFWord,
};

#[derive(Debug, Clone)]
pub enum OS2Table<'a> {
    Version0(OS2Version0<'a>),
    Version1(OS2Version1<'a>),
    Version2(OS2Version2<'a>),
    Version3(OS2Version3<'a>),
    Version4(OS2Version4<'a>),
    Version5(OS2Version5<'a>),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OS2Version0<'a> {
    pub version: u16,
    pub x_avg_char_width: FWord,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: FWord,
    pub y_subscript_y_size: FWord,
    pub y_subscript_x_offset: FWord,
    pub y_subscript_y_offset: FWord,
    pub y_superscript_x_size: FWord,
    pub y_superscript_y_size: FWord,
    pub y_superscript_x_offset: FWord,
    pub y_superscript_y_offset: FWord,
    pub y_strikeout_size: FWord,
    pub y_strikeout_position: FWord,
    pub s_family_class: i16,
    pub panose: &'a [u8],
    pub ul_unicode_range_1: u32,
    pub ul_unicode_range_2: u32,
    pub ul_unicode_range_3: u32,
    pub ul_unicode_range_4: u32,
    pub ach_vend_id: Tag,
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: FWord,
    pub s_typo_descender: FWord,
    pub s_typo_line_gap: FWord,
    pub us_win_ascent: UFWord,
    pub us_win_descent: UFWord,
}

impl<'a> OS2Version0<'a> {
    pub fn is_thin(&self) -> bool {
        self.us_weight_class < 150
    }

    pub fn is_extra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_ultra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_light(&self) -> bool {
        250 <= self.us_weight_class && self.us_weight_class < 350
    }

    pub fn is_normal(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_regular(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_medium(&self) -> bool {
        450 <= self.us_weight_class && self.us_weight_class < 550
    }

    pub fn is_semi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_demi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_bold(&self) -> bool {
        650 <= self.us_weight_class && self.us_weight_class < 750
    }

    pub fn is_extra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_ultra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_black(&self) -> bool {
        self.us_weight_class >= 850
    }

    /// Installable embedding: the font may be embedded, and may be permanently installed for use on a remote systems, or for use by other users. The user of the remote system acquires the identical rights, obligations and licenses for that font as the original purchaser of the font, and is subject to the same end-user license agreement, copyright, design patent, and/or trademark as was the original purchaser.
    pub fn is_installable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 0
    }

    /// Restricted License embedding: the font must not be modified, embedded or exchanged in any manner without first obtaining explicit permission of the legal owner.
    pub fn is_restricted_license_embedding(&self) -> bool {
        self.fs_type & 0x000F == 2
    }

    /// Preview & Print embedding: the font may be embedded, and may be temporarily loaded on other systems for purposes of viewing or printing the document. Documents containing Preview & Print fonts must be opened “read-only”; no edits may be applied to the document.
    pub fn is_preview_and_print_embedding(&self) -> bool {
        self.fs_type & 0x000F == 4
    }

    /// Editable embedding: the font may be embedded, and may be temporarily loaded on other systems. As with Preview & Print embedding, documents containing Editable fonts may be opened for reading. In addition, editing is permitted, including ability to format new text using the embedded font, and changes may be saved.
    pub fn is_editable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 8
    }

    /// No subsetting: When this bit is set, the font must not be subsetted prior to embedding. Other embedding restrictions specified in bits 0 – 3 and bit 9 also apply.
    pub fn is_no_subsetting(&self) -> bool {
        (self.fs_type >> 8) & 1 == 1
    }

    /// Bitmap embedding only: When this bit is set, only bitmaps contained in the font may be embedded. No outline data may be embedded. If there are no bitmaps available in the font, then the font is considered unembeddable and the embedding services will fail. Other embedding restrictions specified in bits 0-3 and 8 also apply.
    pub fn is_bitmap_embedding_only(&self) -> bool {
        (self.fs_type >> 9) & 1 == 1
    }

    pub fn is_basic_latin(&self) -> bool {
        (self.ul_unicode_range_1) & 1 == 1
    }

    pub fn is_latin_1_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 1) & 1 == 1
    }

    pub fn is_latin_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 2) & 1 == 1
    }

    pub fn is_latin_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 3) & 1 == 1
    }

    pub fn is_ipa_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    pub fn is_spacing_modifier_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_modifier_tone_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    pub fn is_combining_diacritical_marks(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_combining_diacritical_marks_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    pub fn is_greek_and_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 7) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 8) & 1 == 1
    }

    pub fn is_cyrillic(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cyrillic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    pub fn is_armenian(&self) -> bool {
        (self.ul_unicode_range_1 >> 10) & 1 == 1
    }

    pub fn is_hebrew(&self) -> bool {
        (self.ul_unicode_range_1 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_vai(&self) -> bool {
        (self.ul_unicode_range_1 >> 12) & 1 == 1
    }

    pub fn is_arabic(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_arabic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_nko(&self) -> bool {
        (self.ul_unicode_range_1 >> 14) & 1 == 1
    }

    pub fn is_devanagari(&self) -> bool {
        (self.ul_unicode_range_1 >> 15) & 1 == 1
    }

    pub fn is_bangla(&self) -> bool {
        (self.ul_unicode_range_1 >> 16) & 1 == 1
    }

    pub fn is_gurmukhi(&self) -> bool {
        (self.ul_unicode_range_1 >> 17) & 1 == 1
    }

    pub fn is_gujarati(&self) -> bool {
        (self.ul_unicode_range_1 >> 18) & 1 == 1
    }

    pub fn is_odia(&self) -> bool {
        (self.ul_unicode_range_1 >> 19) & 1 == 1
    }

    pub fn is_tamil(&self) -> bool {
        (self.ul_unicode_range_1 >> 20) & 1 == 1
    }

    pub fn is_tellugu(&self) -> bool {
        (self.ul_unicode_range_1 >> 21) & 1 == 1
    }

    pub fn is_kannada(&self) -> bool {
        (self.ul_unicode_range_1 >> 22) & 1 == 1
    }

    pub fn is_malayalam(&self) -> bool {
        (self.ul_unicode_range_1 >> 23) & 1 == 1
    }

    pub fn is_thai(&self) -> bool {
        (self.ul_unicode_range_1 >> 24) & 1 == 1
    }

    pub fn is_lao(&self) -> bool {
        (self.ul_unicode_range_1 >> 25) & 1 == 1
    }

    pub fn is_georgian(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_georgian_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_balinese(&self) -> bool {
        (self.ul_unicode_range_1 >> 27) & 1 == 1
    }

    pub fn is_hangul_jamo(&self) -> bool {
        (self.ul_unicode_range_1 >> 28) & 1 == 1
    }

    pub fn is_latin_extended_additional(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_c(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_d(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    pub fn is_greek_extended(&self) -> bool {
        (self.ul_unicode_range_1 >> 30) & 1 == 1
    }

    pub fn is_general_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_supplemental_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    pub fn is_superscripts_and_subscripts(&self) -> bool {
        (self.ul_unicode_range_2) & 1 == 1
    }

    pub fn is_currency_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 1) & 1 == 1
    }

    pub fn is_combining_diacritical_marks_for_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 2) & 1 == 1
    }

    pub fn is_letterlike_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 3) & 1 == 1
    }

    pub fn is_number_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 4) & 1 == 1
    }

    pub fn is_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_miscellaneous_symbols_and_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    pub fn is_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    pub fn is_miscellaneous_technical(&self) -> bool {
        (self.ul_unicode_range_2 >> 7) & 1 == 1
    }

    pub fn is_control_pictures(&self) -> bool {
        (self.ul_unicode_range_2 >> 8) & 1 == 1
    }

    pub fn is_optical_character_recognition(&self) -> bool {
        (self.ul_unicode_range_2 >> 9) & 1 == 1
    }

    pub fn is_enclosed_alphanumerics(&self) -> bool {
        (self.ul_unicode_range_2 >> 10) & 1 == 1
    }

    pub fn is_box_drawing(&self) -> bool {
        (self.ul_unicode_range_2 >> 11) & 1 == 1
    }

    pub fn is_block_elements(&self) -> bool {
        (self.ul_unicode_range_2 >> 12) & 1 == 1
    }

    pub fn is_geometric_shapes(&self) -> bool {
        (self.ul_unicode_range_2 >> 13) & 1 == 1
    }

    pub fn is_miscellaneous_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 14) & 1 == 1
    }

    pub fn is_dingbats(&self) -> bool {
        (self.ul_unicode_range_2 >> 15) & 1 == 1
    }

    pub fn is_cjk_symbols_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_2 >> 16) & 1 == 1
    }

    pub fn is_hiragana(&self) -> bool {
        (self.ul_unicode_range_2 >> 17) & 1 == 1
    }

    pub fn is_katakana(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_katakana_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    pub fn is_bopomofo(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_bopomofo_extended(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    pub fn is_hangul_compatibility_jamo(&self) -> bool {
        (self.ul_unicode_range_2 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_phags_pa(&self) -> bool {
        (self.ul_unicode_range_2 >> 21) & 1 == 1
    }

    pub fn is_enclosed_cjk_letters_and_months(&self) -> bool {
        (self.ul_unicode_range_2 >> 22) & 1 == 1
    }

    pub fn is_cjk_compatibility(&self) -> bool {
        (self.ul_unicode_range_2 >> 23) & 1 == 1
    }

    pub fn is_hangul_syllables(&self) -> bool {
        (self.ul_unicode_range_2 >> 24) & 1 == 1
    }

    /// Setting this bit implies there is at least one character beyond the Basic Multilingual Plane supported by this font. First assigned in OpenType 1.3 for OS/2 version 2.
    pub fn is_non_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phoenician(&self) -> bool {
        (self.ul_unicode_range_2 >> 26) & 1 == 1
    }

    pub fn is_cjk_unified_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_radicals_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_kangxi_radicals(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_ideographic_description_characters(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_unified_ideographs_extension_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_unified_ideographs_extension_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_kanbun(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    pub fn is_private_use_area_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 28) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cjk_strokes(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_cjk_compatibility_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_compatibility_ideographs_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_alphabetic_presentation_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 30) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 31) & 1 == 1
    }

    pub fn is_combining_half_marks(&self) -> bool {
        (self.ul_unicode_range_3) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_vertical_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_cjk_compatibility_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_small_form_variants(&self) -> bool {
        (self.ul_unicode_range_3 >> 2) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_b(&self) -> bool {
        (self.ul_unicode_range_3 >> 3) & 1 == 1
    }

    pub fn is_halfwidth_and_fullwidth_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 4) & 1 == 1
    }

    pub fn is_specials(&self) -> bool {
        (self.ul_unicode_range_3 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_tibetan(&self) -> bool {
        (self.ul_unicode_range_3 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_syriac(&self) -> bool {
        (self.ul_unicode_range_3 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_thaana(&self) -> bool {
        (self.ul_unicode_range_3 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_sinhala(&self) -> bool {
        (self.ul_unicode_range_3 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_myanmar(&self) -> bool {
        (self.ul_unicode_range_3 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ethiopic(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_extended(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_cherokee(&self) -> bool {
        (self.ul_unicode_range_3 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_unified_canadian_aboriginal_syllabics(&self) -> bool {
        (self.ul_unicode_range_3 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ogham(&self) -> bool {
        (self.ul_unicode_range_3 >> 14) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_runic(&self) -> bool {
        (self.ul_unicode_range_3 >> 15) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_khmer(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_khmer_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_mongolian(&self) -> bool {
        (self.ul_unicode_range_3 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_braille_patterns(&self) -> bool {
        (self.ul_unicode_range_3 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_syllables(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_radicals(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagalog(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_hanunoo(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_buhid(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagbanwa(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_old_italic(&self) -> bool {
        (self.ul_unicode_range_3 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_gothic(&self) -> bool {
        (self.ul_unicode_range_3 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_deseret(&self) -> bool {
        (self.ul_unicode_range_3 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_byzantine_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_musical_notation(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_mathematical_alphanumeric_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_15(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_16(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tags(&self) -> bool {
        (self.ul_unicode_range_3 >> 28) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_limbu(&self) -> bool {
        (self.ul_unicode_range_3 >> 29) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 30) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_new_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 31) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_buginese(&self) -> bool {
        (self.ul_unicode_range_4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_glagolitic(&self) -> bool {
        (self.ul_unicode_range_4 >> 1) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tifinagh(&self) -> bool {
        (self.ul_unicode_range_4 >> 2) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_yijing_hexagram_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 3) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_syloti_nagri(&self) -> bool {
        (self.ul_unicode_range_4 >> 4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_ideograms(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_aegean_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ugaritic(&self) -> bool {
        (self.ul_unicode_range_4 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_old_persian(&self) -> bool {
        (self.ul_unicode_range_4 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_shavian(&self) -> bool {
        (self.ul_unicode_range_4 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_osmanya(&self) -> bool {
        (self.ul_unicode_range_4 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cypriot_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kharoshthi(&self) -> bool {
        (self.ul_unicode_range_4 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_xuan_jing_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform_numbers_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_counting_rod_numerals(&self) -> bool {
        (self.ul_unicode_range_4 >> 15) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_sundanese(&self) -> bool {
        (self.ul_unicode_range_4 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_lepcha(&self) -> bool {
        (self.ul_unicode_range_4 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ol_chiki(&self) -> bool {
        (self.ul_unicode_range_4 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_saurashtra(&self) -> bool {
        (self.ul_unicode_range_4 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kayah_li(&self) -> bool {
        (self.ul_unicode_range_4 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_rejang(&self) -> bool {
        (self.ul_unicode_range_4 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cham(&self) -> bool {
        (self.ul_unicode_range_4 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phaistos_disc(&self) -> bool {
        (self.ul_unicode_range_4 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_carian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lycian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lydian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_domino_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_mahjong_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// Font contains italic or oblique glyphs, otherwise they are upright.
    pub fn is_selection_italic(&self) -> bool {
        (self.fs_selection) & 1 == 1
    }

    /// Glyphs are underscored.
    pub fn is_selection_underscore(&self) -> bool {
        (self.fs_selection >> 1) & 1 == 1
    }

    /// Glyphs have their foreground and background reversed.
    pub fn is_selection_negative(&self) -> bool {
        (self.fs_selection >> 2) & 1 == 1
    }

    /// Outline (hollow) glyphs, otherwise they are solid.
    pub fn is_selection_outlined(&self) -> bool {
        (self.fs_selection >> 3) & 1 == 1
    }

    /// Glyphs are overstruck.
    pub fn is_selection_strikeout(&self) -> bool {
        (self.fs_selection >> 4) & 1 == 1
    }

    /// Glyphs are emboldened.
    pub fn is_selection_bold(&self) -> bool {
        (self.fs_selection >> 5) & 1 == 1
    }

    /// Glyphs are in the standard weight/style for the font.
    pub fn is_selection_regular(&self) -> bool {
        (self.fs_selection >> 6) & 1 == 1
    }

    /// If set, it is strongly recommended that applications use OS/2.sTypoAscender - OS/2.sTypoDescender + OS/2.sTypoLineGap as the default line spacing for this font.
    pub fn is_selection_use_typo_metrics(&self) -> bool {
        (self.fs_selection >> 7) & 1 == 1
    }

    /// The font has 'name' table strings consistent with a weight/width/slope family without requiring use of name IDs 21 and 22. (See a more detailed description below.)
    pub fn is_selection_wws(&self) -> bool {
        (self.fs_selection >> 8) & 1 == 1
    }

    /// Font contains oblique glyphs.
    pub fn is_selection_oblique(&self) -> bool {
        (self.fs_selection >> 9) & 1 == 1
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OS2Version1<'a> {
    pub version: u16,
    pub x_avg_char_width: FWord,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: FWord,
    pub y_subscript_y_size: FWord,
    pub y_subscript_x_offset: FWord,
    pub y_subscript_y_offset: FWord,
    pub y_superscript_x_size: FWord,
    pub y_superscript_y_size: FWord,
    pub y_superscript_x_offset: FWord,
    pub y_superscript_y_offset: FWord,
    pub y_strikeout_size: FWord,
    pub y_strikeout_position: FWord,
    pub s_family_class: i16,
    pub panose: &'a [u8],
    pub ul_unicode_range_1: u32,
    pub ul_unicode_range_2: u32,
    pub ul_unicode_range_3: u32,
    pub ul_unicode_range_4: u32,
    pub ach_vend_id: Tag,
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: FWord,
    pub s_typo_descender: FWord,
    pub s_typo_line_gap: FWord,
    pub us_win_ascent: UFWord,
    pub us_win_descent: UFWord,
    pub ul_code_page_range_1: u32,
    pub ul_code_page_range_2: u32,
}

impl<'a> OS2Version1<'a> {
    pub fn is_thin(&self) -> bool {
        self.us_weight_class < 150
    }

    pub fn is_extra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_ultra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_light(&self) -> bool {
        250 <= self.us_weight_class && self.us_weight_class < 350
    }

    pub fn is_normal(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_regular(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_medium(&self) -> bool {
        450 <= self.us_weight_class && self.us_weight_class < 550
    }

    pub fn is_semi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_demi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_bold(&self) -> bool {
        650 <= self.us_weight_class && self.us_weight_class < 750
    }

    pub fn is_extra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_ultra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_black(&self) -> bool {
        self.us_weight_class >= 850
    }
    /// Installable embedding: the font may be embedded, and may be permanently installed for use on a remote systems, or for use by other users. The user of the remote system acquires the identical rights, obligations and licenses for that font as the original purchaser of the font, and is subject to the same end-user license agreement, copyright, design patent, and/or trademark as was the original purchaser.
    pub fn is_installable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 0
    }

    /// Restricted License embedding: the font must not be modified, embedded or exchanged in any manner without first obtaining explicit permission of the legal owner.
    pub fn is_restricted_license_embedding(&self) -> bool {
        self.fs_type & 0x000F == 2
    }

    /// Preview & Print embedding: the font may be embedded, and may be temporarily loaded on other systems for purposes of viewing or printing the document. Documents containing Preview & Print fonts must be opened “read-only”; no edits may be applied to the document.
    pub fn is_preview_and_print_embedding(&self) -> bool {
        self.fs_type & 0x000F == 4
    }

    /// Editable embedding: the font may be embedded, and may be temporarily loaded on other systems. As with Preview & Print embedding, documents containing Editable fonts may be opened for reading. In addition, editing is permitted, including ability to format new text using the embedded font, and changes may be saved.
    pub fn is_editable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 8
    }

    /// No subsetting: When this bit is set, the font must not be subsetted prior to embedding. Other embedding restrictions specified in bits 0 – 3 and bit 9 also apply.
    pub fn is_no_subsetting(&self) -> bool {
        (self.fs_type >> 8) & 1 == 1
    }

    /// Bitmap embedding only: When this bit is set, only bitmaps contained in the font may be embedded. No outline data may be embedded. If there are no bitmaps available in the font, then the font is considered unembeddable and the embedding services will fail. Other embedding restrictions specified in bits 0-3 and 8 also apply.
    pub fn is_bitmap_embedding_only(&self) -> bool {
        (self.fs_type >> 9) & 1 == 1
    }

    pub fn is_basic_latin(&self) -> bool {
        (self.ul_unicode_range_1) & 1 == 1
    }

    pub fn is_latin_1_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 1) & 1 == 1
    }

    pub fn is_latin_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 2) & 1 == 1
    }

    pub fn is_latin_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 3) & 1 == 1
    }

    pub fn is_ipa_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    pub fn is_spacing_modifier_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_modifier_tone_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    pub fn is_combining_diacritical_marks(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_combining_diacritical_marks_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    pub fn is_greek_and_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 7) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 8) & 1 == 1
    }

    pub fn is_cyrillic(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cyrillic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    pub fn is_armenian(&self) -> bool {
        (self.ul_unicode_range_1 >> 10) & 1 == 1
    }

    pub fn is_hebrew(&self) -> bool {
        (self.ul_unicode_range_1 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_vai(&self) -> bool {
        (self.ul_unicode_range_1 >> 12) & 1 == 1
    }

    pub fn is_arabic(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_arabic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_nko(&self) -> bool {
        (self.ul_unicode_range_1 >> 14) & 1 == 1
    }

    pub fn is_devanagari(&self) -> bool {
        (self.ul_unicode_range_1 >> 15) & 1 == 1
    }

    pub fn is_bangla(&self) -> bool {
        (self.ul_unicode_range_1 >> 16) & 1 == 1
    }

    pub fn is_gurmukhi(&self) -> bool {
        (self.ul_unicode_range_1 >> 17) & 1 == 1
    }

    pub fn is_gujarati(&self) -> bool {
        (self.ul_unicode_range_1 >> 18) & 1 == 1
    }

    pub fn is_odia(&self) -> bool {
        (self.ul_unicode_range_1 >> 19) & 1 == 1
    }

    pub fn is_tamil(&self) -> bool {
        (self.ul_unicode_range_1 >> 20) & 1 == 1
    }

    pub fn is_tellugu(&self) -> bool {
        (self.ul_unicode_range_1 >> 21) & 1 == 1
    }

    pub fn is_kannada(&self) -> bool {
        (self.ul_unicode_range_1 >> 22) & 1 == 1
    }

    pub fn is_malayalam(&self) -> bool {
        (self.ul_unicode_range_1 >> 23) & 1 == 1
    }

    pub fn is_thai(&self) -> bool {
        (self.ul_unicode_range_1 >> 24) & 1 == 1
    }

    pub fn is_lao(&self) -> bool {
        (self.ul_unicode_range_1 >> 25) & 1 == 1
    }

    pub fn is_georgian(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_georgian_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_balinese(&self) -> bool {
        (self.ul_unicode_range_1 >> 27) & 1 == 1
    }

    pub fn is_hangul_jamo(&self) -> bool {
        (self.ul_unicode_range_1 >> 28) & 1 == 1
    }

    pub fn is_latin_extended_additional(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_c(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_d(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    pub fn is_greek_extended(&self) -> bool {
        (self.ul_unicode_range_1 >> 30) & 1 == 1
    }

    pub fn is_general_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_supplemental_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    pub fn is_superscripts_and_subscripts(&self) -> bool {
        (self.ul_unicode_range_2) & 1 == 1
    }

    pub fn is_currency_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 1) & 1 == 1
    }

    pub fn is_combining_diacritical_marks_for_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 2) & 1 == 1
    }

    pub fn is_letterlike_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 3) & 1 == 1
    }

    pub fn is_number_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 4) & 1 == 1
    }

    pub fn is_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_miscellaneous_symbols_and_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    pub fn is_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    pub fn is_miscellaneous_technical(&self) -> bool {
        (self.ul_unicode_range_2 >> 7) & 1 == 1
    }

    pub fn is_control_pictures(&self) -> bool {
        (self.ul_unicode_range_2 >> 8) & 1 == 1
    }

    pub fn is_optical_character_recognition(&self) -> bool {
        (self.ul_unicode_range_2 >> 9) & 1 == 1
    }

    pub fn is_enclosed_alphanumerics(&self) -> bool {
        (self.ul_unicode_range_2 >> 10) & 1 == 1
    }

    pub fn is_box_drawing(&self) -> bool {
        (self.ul_unicode_range_2 >> 11) & 1 == 1
    }

    pub fn is_block_elements(&self) -> bool {
        (self.ul_unicode_range_2 >> 12) & 1 == 1
    }

    pub fn is_geometric_shapes(&self) -> bool {
        (self.ul_unicode_range_2 >> 13) & 1 == 1
    }

    pub fn is_miscellaneous_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 14) & 1 == 1
    }

    pub fn is_dingbats(&self) -> bool {
        (self.ul_unicode_range_2 >> 15) & 1 == 1
    }

    pub fn is_cjk_symbols_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_2 >> 16) & 1 == 1
    }

    pub fn is_hiragana(&self) -> bool {
        (self.ul_unicode_range_2 >> 17) & 1 == 1
    }

    pub fn is_katakana(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_katakana_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    pub fn is_bopomofo(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_bopomofo_extended(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    pub fn is_hangul_compatibility_jamo(&self) -> bool {
        (self.ul_unicode_range_2 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_phags_pa(&self) -> bool {
        (self.ul_unicode_range_2 >> 21) & 1 == 1
    }

    pub fn is_enclosed_cjk_letters_and_months(&self) -> bool {
        (self.ul_unicode_range_2 >> 22) & 1 == 1
    }

    pub fn is_cjk_compatibility(&self) -> bool {
        (self.ul_unicode_range_2 >> 23) & 1 == 1
    }

    pub fn is_hangul_syllables(&self) -> bool {
        (self.ul_unicode_range_2 >> 24) & 1 == 1
    }

    /// Setting this bit implies there is at least one character beyond the Basic Multilingual Plane supported by this font. First assigned in OpenType 1.3 for OS/2 version 2.
    pub fn is_non_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phoenician(&self) -> bool {
        (self.ul_unicode_range_2 >> 26) & 1 == 1
    }

    pub fn is_cjk_unified_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_radicals_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_kangxi_radicals(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_ideographic_description_characters(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_unified_ideographs_extension_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_unified_ideographs_extension_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_kanbun(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    pub fn is_private_use_area_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 28) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cjk_strokes(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_cjk_compatibility_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_compatibility_ideographs_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_alphabetic_presentation_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 30) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 31) & 1 == 1
    }

    pub fn is_combining_half_marks(&self) -> bool {
        (self.ul_unicode_range_3) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_vertical_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_cjk_compatibility_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_small_form_variants(&self) -> bool {
        (self.ul_unicode_range_3 >> 2) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_b(&self) -> bool {
        (self.ul_unicode_range_3 >> 3) & 1 == 1
    }

    pub fn is_halfwidth_and_fullwidth_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 4) & 1 == 1
    }

    pub fn is_specials(&self) -> bool {
        (self.ul_unicode_range_3 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_tibetan(&self) -> bool {
        (self.ul_unicode_range_3 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_syriac(&self) -> bool {
        (self.ul_unicode_range_3 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_thaana(&self) -> bool {
        (self.ul_unicode_range_3 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_sinhala(&self) -> bool {
        (self.ul_unicode_range_3 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_myanmar(&self) -> bool {
        (self.ul_unicode_range_3 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ethiopic(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_extended(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_cherokee(&self) -> bool {
        (self.ul_unicode_range_3 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_unified_canadian_aboriginal_syllabics(&self) -> bool {
        (self.ul_unicode_range_3 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ogham(&self) -> bool {
        (self.ul_unicode_range_3 >> 14) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_runic(&self) -> bool {
        (self.ul_unicode_range_3 >> 15) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_khmer(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_khmer_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_mongolian(&self) -> bool {
        (self.ul_unicode_range_3 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_braille_patterns(&self) -> bool {
        (self.ul_unicode_range_3 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_syllables(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_radicals(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagalog(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_hanunoo(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_buhid(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagbanwa(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_old_italic(&self) -> bool {
        (self.ul_unicode_range_3 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_gothic(&self) -> bool {
        (self.ul_unicode_range_3 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_deseret(&self) -> bool {
        (self.ul_unicode_range_3 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_byzantine_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_musical_notation(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_mathematical_alphanumeric_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_15(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_16(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tags(&self) -> bool {
        (self.ul_unicode_range_3 >> 28) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_limbu(&self) -> bool {
        (self.ul_unicode_range_3 >> 29) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 30) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_new_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 31) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_buginese(&self) -> bool {
        (self.ul_unicode_range_4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_glagolitic(&self) -> bool {
        (self.ul_unicode_range_4 >> 1) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tifinagh(&self) -> bool {
        (self.ul_unicode_range_4 >> 2) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_yijing_hexagram_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 3) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_syloti_nagri(&self) -> bool {
        (self.ul_unicode_range_4 >> 4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_ideograms(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_aegean_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ugaritic(&self) -> bool {
        (self.ul_unicode_range_4 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_old_persian(&self) -> bool {
        (self.ul_unicode_range_4 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_shavian(&self) -> bool {
        (self.ul_unicode_range_4 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_osmanya(&self) -> bool {
        (self.ul_unicode_range_4 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cypriot_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kharoshthi(&self) -> bool {
        (self.ul_unicode_range_4 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_xuan_jing_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform_numbers_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_counting_rod_numerals(&self) -> bool {
        (self.ul_unicode_range_4 >> 15) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_sundanese(&self) -> bool {
        (self.ul_unicode_range_4 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_lepcha(&self) -> bool {
        (self.ul_unicode_range_4 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ol_chiki(&self) -> bool {
        (self.ul_unicode_range_4 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_saurashtra(&self) -> bool {
        (self.ul_unicode_range_4 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kayah_li(&self) -> bool {
        (self.ul_unicode_range_4 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_rejang(&self) -> bool {
        (self.ul_unicode_range_4 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cham(&self) -> bool {
        (self.ul_unicode_range_4 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phaistos_disc(&self) -> bool {
        (self.ul_unicode_range_4 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_carian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lycian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lydian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_domino_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_mahjong_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// Font contains italic or oblique glyphs, otherwise they are upright.
    pub fn is_selection_italic(&self) -> bool {
        (self.fs_selection) & 1 == 1
    }

    /// Glyphs are underscored.
    pub fn is_selection_underscore(&self) -> bool {
        (self.fs_selection >> 1) & 1 == 1
    }

    /// Glyphs have their foreground and background reversed.
    pub fn is_selection_negative(&self) -> bool {
        (self.fs_selection >> 2) & 1 == 1
    }

    /// Outline (hollow) glyphs, otherwise they are solid.
    pub fn is_selection_outlined(&self) -> bool {
        (self.fs_selection >> 3) & 1 == 1
    }

    /// Glyphs are overstruck.
    pub fn is_selection_strikeout(&self) -> bool {
        (self.fs_selection >> 4) & 1 == 1
    }

    /// Glyphs are emboldened.
    pub fn is_selection_bold(&self) -> bool {
        (self.fs_selection >> 5) & 1 == 1
    }

    /// Glyphs are in the standard weight/style for the font.
    pub fn is_selection_regular(&self) -> bool {
        (self.fs_selection >> 6) & 1 == 1
    }

    /// If set, it is strongly recommended that applications use OS/2.sTypoAscender - OS/2.sTypoDescender + OS/2.sTypoLineGap as the default line spacing for this font.
    pub fn is_selection_use_typo_metrics(&self) -> bool {
        (self.fs_selection >> 7) & 1 == 1
    }

    /// The font has 'name' table strings consistent with a weight/width/slope family without requiring use of name IDs 21 and 22. (See a more detailed description below.)
    pub fn is_selection_wws(&self) -> bool {
        (self.fs_selection >> 8) & 1 == 1
    }

    /// Font contains oblique glyphs.
    pub fn is_selection_oblique(&self) -> bool {
        (self.fs_selection >> 9) & 1 == 1
    }
}

pub type OS2Version2<'a> = OS2Version3<'a>;
pub type OS2Version3<'a> = OS2Version4<'a>;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OS2Version4<'a> {
    pub version: u16,
    pub x_avg_char_width: FWord,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: FWord,
    pub y_subscript_y_size: FWord,
    pub y_subscript_x_offset: FWord,
    pub y_subscript_y_offset: FWord,
    pub y_superscript_x_size: FWord,
    pub y_superscript_y_size: FWord,
    pub y_superscript_x_offset: FWord,
    pub y_superscript_y_offset: FWord,
    pub y_strikeout_size: FWord,
    pub y_strikeout_position: FWord,
    pub s_family_class: i16,
    pub panose: &'a [u8],
    pub ul_unicode_range_1: u32,
    pub ul_unicode_range_2: u32,
    pub ul_unicode_range_3: u32,
    pub ul_unicode_range_4: u32,
    pub ach_vend_id: Tag,
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: FWord,
    pub s_typo_descender: FWord,
    pub s_typo_line_gap: FWord,
    pub us_win_ascent: UFWord,
    pub us_win_descent: UFWord,
    pub ul_code_page_range_1: u32,
    pub ul_code_page_range_2: u32,
    pub sx_height: FWord,
    pub s_cap_height: FWord,
    pub us_default_char: u16,
    pub us_break_char: u16,
    pub us_max_context: u16,
}

impl<'a> OS2Version4<'a> {
    pub fn is_thin(&self) -> bool {
        self.us_weight_class < 150
    }

    pub fn is_extra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_ultra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_light(&self) -> bool {
        250 <= self.us_weight_class && self.us_weight_class < 350
    }

    pub fn is_normal(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_regular(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_medium(&self) -> bool {
        450 <= self.us_weight_class && self.us_weight_class < 550
    }

    pub fn is_semi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_demi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_bold(&self) -> bool {
        650 <= self.us_weight_class && self.us_weight_class < 750
    }

    pub fn is_extra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_ultra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_black(&self) -> bool {
        self.us_weight_class >= 850
    }
    /// Installable embedding: the font may be embedded, and may be permanently installed for use on a remote systems, or for use by other users. The user of the remote system acquires the identical rights, obligations and licenses for that font as the original purchaser of the font, and is subject to the same end-user license agreement, copyright, design patent, and/or trademark as was the original purchaser.
    pub fn is_installable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 0
    }

    /// Restricted License embedding: the font must not be modified, embedded or exchanged in any manner without first obtaining explicit permission of the legal owner.
    pub fn is_restricted_license_embedding(&self) -> bool {
        self.fs_type & 0x000F == 2
    }

    /// Preview & Print embedding: the font may be embedded, and may be temporarily loaded on other systems for purposes of viewing or printing the document. Documents containing Preview & Print fonts must be opened “read-only”; no edits may be applied to the document.
    pub fn is_preview_and_print_embedding(&self) -> bool {
        self.fs_type & 0x000F == 4
    }

    /// Editable embedding: the font may be embedded, and may be temporarily loaded on other systems. As with Preview & Print embedding, documents containing Editable fonts may be opened for reading. In addition, editing is permitted, including ability to format new text using the embedded font, and changes may be saved.
    pub fn is_editable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 8
    }

    /// No subsetting: When this bit is set, the font must not be subsetted prior to embedding. Other embedding restrictions specified in bits 0 – 3 and bit 9 also apply.
    pub fn is_no_subsetting(&self) -> bool {
        (self.fs_type >> 8) & 1 == 1
    }

    /// Bitmap embedding only: When this bit is set, only bitmaps contained in the font may be embedded. No outline data may be embedded. If there are no bitmaps available in the font, then the font is considered unembeddable and the embedding services will fail. Other embedding restrictions specified in bits 0-3 and 8 also apply.
    pub fn is_bitmap_embedding_only(&self) -> bool {
        (self.fs_type >> 9) & 1 == 1
    }

    pub fn is_basic_latin(&self) -> bool {
        (self.ul_unicode_range_1) & 1 == 1
    }

    pub fn is_latin_1_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 1) & 1 == 1
    }

    pub fn is_latin_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 2) & 1 == 1
    }

    pub fn is_latin_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 3) & 1 == 1
    }

    pub fn is_ipa_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    pub fn is_spacing_modifier_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_modifier_tone_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    pub fn is_combining_diacritical_marks(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_combining_diacritical_marks_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    pub fn is_greek_and_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 7) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 8) & 1 == 1
    }

    pub fn is_cyrillic(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cyrillic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    pub fn is_armenian(&self) -> bool {
        (self.ul_unicode_range_1 >> 10) & 1 == 1
    }

    pub fn is_hebrew(&self) -> bool {
        (self.ul_unicode_range_1 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_vai(&self) -> bool {
        (self.ul_unicode_range_1 >> 12) & 1 == 1
    }

    pub fn is_arabic(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_arabic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_nko(&self) -> bool {
        (self.ul_unicode_range_1 >> 14) & 1 == 1
    }

    pub fn is_devanagari(&self) -> bool {
        (self.ul_unicode_range_1 >> 15) & 1 == 1
    }

    pub fn is_bangla(&self) -> bool {
        (self.ul_unicode_range_1 >> 16) & 1 == 1
    }

    pub fn is_gurmukhi(&self) -> bool {
        (self.ul_unicode_range_1 >> 17) & 1 == 1
    }

    pub fn is_gujarati(&self) -> bool {
        (self.ul_unicode_range_1 >> 18) & 1 == 1
    }

    pub fn is_odia(&self) -> bool {
        (self.ul_unicode_range_1 >> 19) & 1 == 1
    }

    pub fn is_tamil(&self) -> bool {
        (self.ul_unicode_range_1 >> 20) & 1 == 1
    }

    pub fn is_tellugu(&self) -> bool {
        (self.ul_unicode_range_1 >> 21) & 1 == 1
    }

    pub fn is_kannada(&self) -> bool {
        (self.ul_unicode_range_1 >> 22) & 1 == 1
    }

    pub fn is_malayalam(&self) -> bool {
        (self.ul_unicode_range_1 >> 23) & 1 == 1
    }

    pub fn is_thai(&self) -> bool {
        (self.ul_unicode_range_1 >> 24) & 1 == 1
    }

    pub fn is_lao(&self) -> bool {
        (self.ul_unicode_range_1 >> 25) & 1 == 1
    }

    pub fn is_georgian(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_georgian_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_balinese(&self) -> bool {
        (self.ul_unicode_range_1 >> 27) & 1 == 1
    }

    pub fn is_hangul_jamo(&self) -> bool {
        (self.ul_unicode_range_1 >> 28) & 1 == 1
    }

    pub fn is_latin_extended_additional(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_c(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_d(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    pub fn is_greek_extended(&self) -> bool {
        (self.ul_unicode_range_1 >> 30) & 1 == 1
    }

    pub fn is_general_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_supplemental_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    pub fn is_superscripts_and_subscripts(&self) -> bool {
        (self.ul_unicode_range_2) & 1 == 1
    }

    pub fn is_currency_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 1) & 1 == 1
    }

    pub fn is_combining_diacritical_marks_for_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 2) & 1 == 1
    }

    pub fn is_letterlike_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 3) & 1 == 1
    }

    pub fn is_number_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 4) & 1 == 1
    }

    pub fn is_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_miscellaneous_symbols_and_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    pub fn is_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    pub fn is_miscellaneous_technical(&self) -> bool {
        (self.ul_unicode_range_2 >> 7) & 1 == 1
    }

    pub fn is_control_pictures(&self) -> bool {
        (self.ul_unicode_range_2 >> 8) & 1 == 1
    }

    pub fn is_optical_character_recognition(&self) -> bool {
        (self.ul_unicode_range_2 >> 9) & 1 == 1
    }

    pub fn is_enclosed_alphanumerics(&self) -> bool {
        (self.ul_unicode_range_2 >> 10) & 1 == 1
    }

    pub fn is_box_drawing(&self) -> bool {
        (self.ul_unicode_range_2 >> 11) & 1 == 1
    }

    pub fn is_block_elements(&self) -> bool {
        (self.ul_unicode_range_2 >> 12) & 1 == 1
    }

    pub fn is_geometric_shapes(&self) -> bool {
        (self.ul_unicode_range_2 >> 13) & 1 == 1
    }

    pub fn is_miscellaneous_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 14) & 1 == 1
    }

    pub fn is_dingbats(&self) -> bool {
        (self.ul_unicode_range_2 >> 15) & 1 == 1
    }

    pub fn is_cjk_symbols_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_2 >> 16) & 1 == 1
    }

    pub fn is_hiragana(&self) -> bool {
        (self.ul_unicode_range_2 >> 17) & 1 == 1
    }

    pub fn is_katakana(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_katakana_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    pub fn is_bopomofo(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_bopomofo_extended(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    pub fn is_hangul_compatibility_jamo(&self) -> bool {
        (self.ul_unicode_range_2 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_phags_pa(&self) -> bool {
        (self.ul_unicode_range_2 >> 21) & 1 == 1
    }

    pub fn is_enclosed_cjk_letters_and_months(&self) -> bool {
        (self.ul_unicode_range_2 >> 22) & 1 == 1
    }

    pub fn is_cjk_compatibility(&self) -> bool {
        (self.ul_unicode_range_2 >> 23) & 1 == 1
    }

    pub fn is_hangul_syllables(&self) -> bool {
        (self.ul_unicode_range_2 >> 24) & 1 == 1
    }

    /// Setting this bit implies there is at least one character beyond the Basic Multilingual Plane supported by this font. First assigned in OpenType 1.3 for OS/2 version 2.
    pub fn is_non_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phoenician(&self) -> bool {
        (self.ul_unicode_range_2 >> 26) & 1 == 1
    }

    pub fn is_cjk_unified_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_radicals_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_kangxi_radicals(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_ideographic_description_characters(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_unified_ideographs_extension_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_unified_ideographs_extension_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_kanbun(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    pub fn is_private_use_area_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 28) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cjk_strokes(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_cjk_compatibility_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_compatibility_ideographs_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_alphabetic_presentation_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 30) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 31) & 1 == 1
    }

    pub fn is_combining_half_marks(&self) -> bool {
        (self.ul_unicode_range_3) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_vertical_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_cjk_compatibility_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_small_form_variants(&self) -> bool {
        (self.ul_unicode_range_3 >> 2) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_b(&self) -> bool {
        (self.ul_unicode_range_3 >> 3) & 1 == 1
    }

    pub fn is_halfwidth_and_fullwidth_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 4) & 1 == 1
    }

    pub fn is_specials(&self) -> bool {
        (self.ul_unicode_range_3 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_tibetan(&self) -> bool {
        (self.ul_unicode_range_3 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_syriac(&self) -> bool {
        (self.ul_unicode_range_3 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_thaana(&self) -> bool {
        (self.ul_unicode_range_3 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_sinhala(&self) -> bool {
        (self.ul_unicode_range_3 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_myanmar(&self) -> bool {
        (self.ul_unicode_range_3 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ethiopic(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_extended(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_cherokee(&self) -> bool {
        (self.ul_unicode_range_3 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_unified_canadian_aboriginal_syllabics(&self) -> bool {
        (self.ul_unicode_range_3 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ogham(&self) -> bool {
        (self.ul_unicode_range_3 >> 14) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_runic(&self) -> bool {
        (self.ul_unicode_range_3 >> 15) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_khmer(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_khmer_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_mongolian(&self) -> bool {
        (self.ul_unicode_range_3 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_braille_patterns(&self) -> bool {
        (self.ul_unicode_range_3 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_syllables(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_radicals(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagalog(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_hanunoo(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_buhid(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagbanwa(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_old_italic(&self) -> bool {
        (self.ul_unicode_range_3 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_gothic(&self) -> bool {
        (self.ul_unicode_range_3 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_deseret(&self) -> bool {
        (self.ul_unicode_range_3 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_byzantine_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_musical_notation(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_mathematical_alphanumeric_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_15(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_16(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tags(&self) -> bool {
        (self.ul_unicode_range_3 >> 28) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_limbu(&self) -> bool {
        (self.ul_unicode_range_3 >> 29) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 30) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_new_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 31) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_buginese(&self) -> bool {
        (self.ul_unicode_range_4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_glagolitic(&self) -> bool {
        (self.ul_unicode_range_4 >> 1) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tifinagh(&self) -> bool {
        (self.ul_unicode_range_4 >> 2) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_yijing_hexagram_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 3) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_syloti_nagri(&self) -> bool {
        (self.ul_unicode_range_4 >> 4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_ideograms(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_aegean_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ugaritic(&self) -> bool {
        (self.ul_unicode_range_4 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_old_persian(&self) -> bool {
        (self.ul_unicode_range_4 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_shavian(&self) -> bool {
        (self.ul_unicode_range_4 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_osmanya(&self) -> bool {
        (self.ul_unicode_range_4 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cypriot_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kharoshthi(&self) -> bool {
        (self.ul_unicode_range_4 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_xuan_jing_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform_numbers_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_counting_rod_numerals(&self) -> bool {
        (self.ul_unicode_range_4 >> 15) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_sundanese(&self) -> bool {
        (self.ul_unicode_range_4 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_lepcha(&self) -> bool {
        (self.ul_unicode_range_4 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ol_chiki(&self) -> bool {
        (self.ul_unicode_range_4 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_saurashtra(&self) -> bool {
        (self.ul_unicode_range_4 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kayah_li(&self) -> bool {
        (self.ul_unicode_range_4 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_rejang(&self) -> bool {
        (self.ul_unicode_range_4 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cham(&self) -> bool {
        (self.ul_unicode_range_4 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phaistos_disc(&self) -> bool {
        (self.ul_unicode_range_4 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_carian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lycian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lydian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_domino_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_mahjong_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// Font contains italic or oblique glyphs, otherwise they are upright.
    pub fn is_selection_italic(&self) -> bool {
        (self.fs_selection) & 1 == 1
    }

    /// Glyphs are underscored.
    pub fn is_selection_underscore(&self) -> bool {
        (self.fs_selection >> 1) & 1 == 1
    }

    /// Glyphs have their foreground and background reversed.
    pub fn is_selection_negative(&self) -> bool {
        (self.fs_selection >> 2) & 1 == 1
    }

    /// Outline (hollow) glyphs, otherwise they are solid.
    pub fn is_selection_outlined(&self) -> bool {
        (self.fs_selection >> 3) & 1 == 1
    }

    /// Glyphs are overstruck.
    pub fn is_selection_strikeout(&self) -> bool {
        (self.fs_selection >> 4) & 1 == 1
    }

    /// Glyphs are emboldened.
    pub fn is_selection_bold(&self) -> bool {
        (self.fs_selection >> 5) & 1 == 1
    }

    /// Glyphs are in the standard weight/style for the font.
    pub fn is_selection_regular(&self) -> bool {
        (self.fs_selection >> 6) & 1 == 1
    }

    /// If set, it is strongly recommended that applications use OS/2.sTypoAscender - OS/2.sTypoDescender + OS/2.sTypoLineGap as the default line spacing for this font.
    pub fn is_selection_use_typo_metrics(&self) -> bool {
        (self.fs_selection >> 7) & 1 == 1
    }

    /// The font has 'name' table strings consistent with a weight/width/slope family without requiring use of name IDs 21 and 22. (See a more detailed description below.)
    pub fn is_selection_wws(&self) -> bool {
        (self.fs_selection >> 8) & 1 == 1
    }

    /// Font contains oblique glyphs.
    pub fn is_selection_oblique(&self) -> bool {
        (self.fs_selection >> 9) & 1 == 1
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct OS2Version5<'a> {
    /// 0x0005
    pub version: u16,
    pub x_avg_char_width: FWord,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: FWord,
    pub y_subscript_y_size: FWord,
    pub y_subscript_x_offset: FWord,
    pub y_subscript_y_offset: FWord,
    pub y_superscript_x_size: FWord,
    pub y_superscript_y_size: FWord,
    pub y_superscript_x_offset: FWord,
    pub y_superscript_y_offset: FWord,
    pub y_strikeout_size: FWord,
    pub y_strikeout_position: FWord,
    pub s_family_class: i16,
    pub panose: &'a [u8],
    /// Bits 0 – 31
    pub ul_unicode_range_1: u32,
    /// Bits 32 – 63
    pub ul_unicode_range_2: u32,
    /// Bits 64 – 95
    pub ul_unicode_range_3: u32,
    /// Bits 96 – 127
    pub ul_unicode_range_4: u32,
    pub ach_vend_id: Tag,
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: FWord,
    pub s_typo_descender: FWord,
    pub s_typo_line_gap: FWord,
    pub us_win_ascent: UFWord,
    pub us_win_descent: UFWord,
    /// Bits 0 – 31
    pub ul_code_page_range_1: u32,
    /// Bits 32 – 63
    pub ul_code_page_range_2: u32,
    pub sx_height: FWord,
    pub s_cap_height: FWord,
    pub us_default_char: u16,
    pub us_break_char: u16,
    pub us_max_context: u16,
    pub us_lower_optical_point_size: u16,
    pub us_upper_optical_point_size: u16,
}

impl<'a> OS2Version5<'a> {
    pub fn is_thin(&self) -> bool {
        self.us_weight_class < 150
    }

    pub fn is_extra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_ultra_light(&self) -> bool {
        150 <= self.us_weight_class && self.us_weight_class < 250
    }

    pub fn is_light(&self) -> bool {
        250 <= self.us_weight_class && self.us_weight_class < 350
    }

    pub fn is_normal(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_regular(&self) -> bool {
        350 <= self.us_weight_class && self.us_weight_class < 450
    }

    pub fn is_medium(&self) -> bool {
        450 <= self.us_weight_class && self.us_weight_class < 550
    }

    pub fn is_semi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_demi_bold(&self) -> bool {
        550 <= self.us_weight_class && self.us_weight_class < 650
    }

    pub fn is_bold(&self) -> bool {
        650 <= self.us_weight_class && self.us_weight_class < 750
    }

    pub fn is_extra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_ultra_bold(&self) -> bool {
        750 <= self.us_weight_class && self.us_weight_class < 850
    }

    pub fn is_black(&self) -> bool {
        self.us_weight_class >= 850
    }
    /// Installable embedding: the font may be embedded, and may be permanently installed for use on a remote systems, or for use by other users. The user of the remote system acquires the identical rights, obligations and licenses for that font as the original purchaser of the font, and is subject to the same end-user license agreement, copyright, design patent, and/or trademark as was the original purchaser.
    pub fn is_installable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 0
    }

    /// Restricted License embedding: the font must not be modified, embedded or exchanged in any manner without first obtaining explicit permission of the legal owner.
    pub fn is_restricted_license_embedding(&self) -> bool {
        self.fs_type & 0x000F == 2
    }

    /// Preview & Print embedding: the font may be embedded, and may be temporarily loaded on other systems for purposes of viewing or printing the document. Documents containing Preview & Print fonts must be opened “read-only”; no edits may be applied to the document.
    pub fn is_preview_and_print_embedding(&self) -> bool {
        self.fs_type & 0x000F == 4
    }

    /// Editable embedding: the font may be embedded, and may be temporarily loaded on other systems. As with Preview & Print embedding, documents containing Editable fonts may be opened for reading. In addition, editing is permitted, including ability to format new text using the embedded font, and changes may be saved.
    pub fn is_editable_embedding(&self) -> bool {
        self.fs_type & 0x000F == 8
    }

    /// No subsetting: When this bit is set, the font must not be subsetted prior to embedding. Other embedding restrictions specified in bits 0 – 3 and bit 9 also apply.
    pub fn is_no_subsetting(&self) -> bool {
        (self.fs_type >> 8) & 1 == 1
    }

    /// Bitmap embedding only: When this bit is set, only bitmaps contained in the font may be embedded. No outline data may be embedded. If there are no bitmaps available in the font, then the font is considered unembeddable and the embedding services will fail. Other embedding restrictions specified in bits 0-3 and 8 also apply.
    pub fn is_bitmap_embedding_only(&self) -> bool {
        (self.fs_type >> 9) & 1 == 1
    }

    pub fn is_basic_latin(&self) -> bool {
        (self.ul_unicode_range_1) & 1 == 1
    }

    pub fn is_latin_1_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 1) & 1 == 1
    }

    pub fn is_latin_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 2) & 1 == 1
    }

    pub fn is_latin_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 3) & 1 == 1
    }

    pub fn is_ipa_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_phonetic_extensions_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 4) & 1 == 1
    }

    pub fn is_spacing_modifier_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_modifier_tone_letters(&self) -> bool {
        (self.ul_unicode_range_1 >> 5) & 1 == 1
    }

    pub fn is_combining_diacritical_marks(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_combining_diacritical_marks_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 6) & 1 == 1
    }

    pub fn is_greek_and_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 7) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_coptic(&self) -> bool {
        (self.ul_unicode_range_1 >> 8) & 1 == 1
    }

    pub fn is_cyrillic(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cyrillic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_a(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cyrillic_extended_b(&self) -> bool {
        (self.ul_unicode_range_1 >> 9) & 1 == 1
    }

    pub fn is_armenian(&self) -> bool {
        (self.ul_unicode_range_1 >> 10) & 1 == 1
    }

    pub fn is_hebrew(&self) -> bool {
        (self.ul_unicode_range_1 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_vai(&self) -> bool {
        (self.ul_unicode_range_1 >> 12) & 1 == 1
    }

    pub fn is_arabic(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_arabic_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 13) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_nko(&self) -> bool {
        (self.ul_unicode_range_1 >> 14) & 1 == 1
    }

    pub fn is_devanagari(&self) -> bool {
        (self.ul_unicode_range_1 >> 15) & 1 == 1
    }

    pub fn is_bangla(&self) -> bool {
        (self.ul_unicode_range_1 >> 16) & 1 == 1
    }

    pub fn is_gurmukhi(&self) -> bool {
        (self.ul_unicode_range_1 >> 17) & 1 == 1
    }

    pub fn is_gujarati(&self) -> bool {
        (self.ul_unicode_range_1 >> 18) & 1 == 1
    }

    pub fn is_odia(&self) -> bool {
        (self.ul_unicode_range_1 >> 19) & 1 == 1
    }

    pub fn is_tamil(&self) -> bool {
        (self.ul_unicode_range_1 >> 20) & 1 == 1
    }

    pub fn is_tellugu(&self) -> bool {
        (self.ul_unicode_range_1 >> 21) & 1 == 1
    }

    pub fn is_kannada(&self) -> bool {
        (self.ul_unicode_range_1 >> 22) & 1 == 1
    }

    pub fn is_malayalam(&self) -> bool {
        (self.ul_unicode_range_1 >> 23) & 1 == 1
    }

    pub fn is_thai(&self) -> bool {
        (self.ul_unicode_range_1 >> 24) & 1 == 1
    }

    pub fn is_lao(&self) -> bool {
        (self.ul_unicode_range_1 >> 25) & 1 == 1
    }

    pub fn is_georgian(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_georgian_supplement(&self) -> bool {
        (self.ul_unicode_range_1 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_balinese(&self) -> bool {
        (self.ul_unicode_range_1 >> 27) & 1 == 1
    }

    pub fn is_hangul_jamo(&self) -> bool {
        (self.ul_unicode_range_1 >> 28) & 1 == 1
    }

    pub fn is_latin_extended_additional(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_c(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_latin_extended_d(&self) -> bool {
        (self.ul_unicode_range_1 >> 29) & 1 == 1
    }

    pub fn is_greek_extended(&self) -> bool {
        (self.ul_unicode_range_1 >> 30) & 1 == 1
    }

    pub fn is_general_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_supplemental_punctuation(&self) -> bool {
        (self.ul_unicode_range_1 >> 31) & 1 == 1
    }

    pub fn is_superscripts_and_subscripts(&self) -> bool {
        (self.ul_unicode_range_2) & 1 == 1
    }

    pub fn is_currency_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 1) & 1 == 1
    }

    pub fn is_combining_diacritical_marks_for_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 2) & 1 == 1
    }

    pub fn is_letterlike_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 3) & 1 == 1
    }

    pub fn is_number_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 4) & 1 == 1
    }

    pub fn is_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_arrows_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_miscellaneous_symbols_and_arrows(&self) -> bool {
        (self.ul_unicode_range_2 >> 5) & 1 == 1
    }

    pub fn is_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_supplemental_mathematical_operators(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_miscellaneous_mathematical_symbols_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 6) & 1 == 1
    }

    pub fn is_miscellaneous_technical(&self) -> bool {
        (self.ul_unicode_range_2 >> 7) & 1 == 1
    }

    pub fn is_control_pictures(&self) -> bool {
        (self.ul_unicode_range_2 >> 8) & 1 == 1
    }

    pub fn is_optical_character_recognition(&self) -> bool {
        (self.ul_unicode_range_2 >> 9) & 1 == 1
    }

    pub fn is_enclosed_alphanumerics(&self) -> bool {
        (self.ul_unicode_range_2 >> 10) & 1 == 1
    }

    pub fn is_box_drawing(&self) -> bool {
        (self.ul_unicode_range_2 >> 11) & 1 == 1
    }

    pub fn is_block_elements(&self) -> bool {
        (self.ul_unicode_range_2 >> 12) & 1 == 1
    }

    pub fn is_geometric_shapes(&self) -> bool {
        (self.ul_unicode_range_2 >> 13) & 1 == 1
    }

    pub fn is_miscellaneous_symbols(&self) -> bool {
        (self.ul_unicode_range_2 >> 14) & 1 == 1
    }

    pub fn is_dingbats(&self) -> bool {
        (self.ul_unicode_range_2 >> 15) & 1 == 1
    }

    pub fn is_cjk_symbols_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_2 >> 16) & 1 == 1
    }

    pub fn is_hiragana(&self) -> bool {
        (self.ul_unicode_range_2 >> 17) & 1 == 1
    }

    pub fn is_katakana(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_katakana_phonetic_extensions(&self) -> bool {
        (self.ul_unicode_range_2 >> 18) & 1 == 1
    }

    pub fn is_bopomofo(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_bopomofo_extended(&self) -> bool {
        (self.ul_unicode_range_2 >> 19) & 1 == 1
    }

    pub fn is_hangul_compatibility_jamo(&self) -> bool {
        (self.ul_unicode_range_2 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    pub fn is_phags_pa(&self) -> bool {
        (self.ul_unicode_range_2 >> 21) & 1 == 1
    }

    pub fn is_enclosed_cjk_letters_and_months(&self) -> bool {
        (self.ul_unicode_range_2 >> 22) & 1 == 1
    }

    pub fn is_cjk_compatibility(&self) -> bool {
        (self.ul_unicode_range_2 >> 23) & 1 == 1
    }

    pub fn is_hangul_syllables(&self) -> bool {
        (self.ul_unicode_range_2 >> 24) & 1 == 1
    }

    /// Setting this bit implies there is at least one character beyond the Basic Multilingual Plane supported by this font. First assigned in OpenType 1.3 for OS/2 version 2.
    pub fn is_non_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phoenician(&self) -> bool {
        (self.ul_unicode_range_2 >> 26) & 1 == 1
    }

    pub fn is_cjk_unified_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_radicals_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_kangxi_radicals(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_ideographic_description_characters(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.3 for OS/2 version 2.
    pub fn is_cjk_unified_ideographs_extension_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_unified_ideographs_extension_b(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_kanbun(&self) -> bool {
        (self.ul_unicode_range_2 >> 27) & 1 == 1
    }

    pub fn is_private_use_area_plane_0(&self) -> bool {
        (self.ul_unicode_range_2 >> 28) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cjk_strokes(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_cjk_compatibility_ideographs(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_cjk_compatibility_ideographs_supplement(&self) -> bool {
        (self.ul_unicode_range_2 >> 29) & 1 == 1
    }

    pub fn is_alphabetic_presentation_forms(&self) -> bool {
        (self.ul_unicode_range_2 >> 30) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_a(&self) -> bool {
        (self.ul_unicode_range_2 >> 31) & 1 == 1
    }

    pub fn is_combining_half_marks(&self) -> bool {
        (self.ul_unicode_range_3) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_vertical_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_cjk_compatibility_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 1) & 1 == 1
    }

    pub fn is_small_form_variants(&self) -> bool {
        (self.ul_unicode_range_3 >> 2) & 1 == 1
    }

    pub fn is_arabic_presentation_forms_b(&self) -> bool {
        (self.ul_unicode_range_3 >> 3) & 1 == 1
    }

    pub fn is_halfwidth_and_fullwidth_forms(&self) -> bool {
        (self.ul_unicode_range_3 >> 4) & 1 == 1
    }

    pub fn is_specials(&self) -> bool {
        (self.ul_unicode_range_3 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_tibetan(&self) -> bool {
        (self.ul_unicode_range_3 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_syriac(&self) -> bool {
        (self.ul_unicode_range_3 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_thaana(&self) -> bool {
        (self.ul_unicode_range_3 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_sinhala(&self) -> bool {
        (self.ul_unicode_range_3 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_myanmar(&self) -> bool {
        (self.ul_unicode_range_3 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ethiopic(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ethiopic_extended(&self) -> bool {
        (self.ul_unicode_range_3 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_cherokee(&self) -> bool {
        (self.ul_unicode_range_3 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_unified_canadian_aboriginal_syllabics(&self) -> bool {
        (self.ul_unicode_range_3 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_ogham(&self) -> bool {
        (self.ul_unicode_range_3 >> 14) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_runic(&self) -> bool {
        (self.ul_unicode_range_3 >> 15) & 1 == 1
    }
    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_khmer(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_khmer_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_mongolian(&self) -> bool {
        (self.ul_unicode_range_3 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_braille_patterns(&self) -> bool {
        (self.ul_unicode_range_3 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_syllables(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// Added in OpenType 1.3, extending OS/2 version 2.
    pub fn is_yi_radicals(&self) -> bool {
        (self.ul_unicode_range_3 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagalog(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_hanunoo(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_buhid(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_tagbanwa(&self) -> bool {
        (self.ul_unicode_range_3 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_old_italic(&self) -> bool {
        (self.ul_unicode_range_3 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_gothic(&self) -> bool {
        (self.ul_unicode_range_3 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_deseret(&self) -> bool {
        (self.ul_unicode_range_3 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_byzantine_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_musical_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_musical_notation(&self) -> bool {
        (self.ul_unicode_range_3 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_mathematical_alphanumeric_symbols(&self) -> bool {
        (self.ul_unicode_range_3 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_15(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_private_use_plane_16(&self) -> bool {
        (self.ul_unicode_range_3 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// Added in OpenType 1.4 for OS/2 version 3.
    pub fn is_variation_selectors_supplement(&self) -> bool {
        (self.ul_unicode_range_3 >> 27) & 1 == 1
    }

    /// First assigned in OpenType 1.4 for OS/2 version 3.
    pub fn is_tags(&self) -> bool {
        (self.ul_unicode_range_3 >> 28) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_limbu(&self) -> bool {
        (self.ul_unicode_range_3 >> 29) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 30) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_new_tai_le(&self) -> bool {
        (self.ul_unicode_range_3 >> 31) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_buginese(&self) -> bool {
        (self.ul_unicode_range_4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_glagolitic(&self) -> bool {
        (self.ul_unicode_range_4 >> 1) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tifinagh(&self) -> bool {
        (self.ul_unicode_range_4 >> 2) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_yijing_hexagram_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 3) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_syloti_nagri(&self) -> bool {
        (self.ul_unicode_range_4 >> 4) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_linear_b_ideograms(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_aegean_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 5) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_greek_numbers(&self) -> bool {
        (self.ul_unicode_range_4 >> 6) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ugaritic(&self) -> bool {
        (self.ul_unicode_range_4 >> 7) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_old_persian(&self) -> bool {
        (self.ul_unicode_range_4 >> 8) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_shavian(&self) -> bool {
        (self.ul_unicode_range_4 >> 9) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_osmanya(&self) -> bool {
        (self.ul_unicode_range_4 >> 10) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cypriot_syllabary(&self) -> bool {
        (self.ul_unicode_range_4 >> 11) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kharoshthi(&self) -> bool {
        (self.ul_unicode_range_4 >> 12) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_tai_xuan_jing_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 13) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_cuneiform_numbers_and_punctuation(&self) -> bool {
        (self.ul_unicode_range_4 >> 14) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_counting_rod_numerals(&self) -> bool {
        (self.ul_unicode_range_4 >> 15) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_sundanese(&self) -> bool {
        (self.ul_unicode_range_4 >> 16) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_lepcha(&self) -> bool {
        (self.ul_unicode_range_4 >> 17) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ol_chiki(&self) -> bool {
        (self.ul_unicode_range_4 >> 18) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_saurashtra(&self) -> bool {
        (self.ul_unicode_range_4 >> 19) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_kayah_li(&self) -> bool {
        (self.ul_unicode_range_4 >> 20) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_rejang(&self) -> bool {
        (self.ul_unicode_range_4 >> 21) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_cham(&self) -> bool {
        (self.ul_unicode_range_4 >> 22) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_ancient_symbols(&self) -> bool {
        (self.ul_unicode_range_4 >> 23) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_phaistos_disc(&self) -> bool {
        (self.ul_unicode_range_4 >> 24) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_carian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lycian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// Added in OpenType 1.5 for OS/2 version 4.
    pub fn is_lydian(&self) -> bool {
        (self.ul_unicode_range_4 >> 25) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_domino_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// First assigned in OpenType 1.5 for OS/2 version 4.
    pub fn is_mahjong_tiles(&self) -> bool {
        (self.ul_unicode_range_4 >> 26) & 1 == 1
    }

    /// Font contains italic or oblique glyphs, otherwise they are upright.
    pub fn is_selection_italic(&self) -> bool {
        (self.fs_selection) & 1 == 1
    }

    /// Glyphs are underscored.
    pub fn is_selection_underscore(&self) -> bool {
        (self.fs_selection >> 1) & 1 == 1
    }

    /// Glyphs have their foreground and background reversed.
    pub fn is_selection_negative(&self) -> bool {
        (self.fs_selection >> 2) & 1 == 1
    }

    /// Outline (hollow) glyphs, otherwise they are solid.
    pub fn is_selection_outlined(&self) -> bool {
        (self.fs_selection >> 3) & 1 == 1
    }

    /// Glyphs are overstruck.
    pub fn is_selection_strikeout(&self) -> bool {
        (self.fs_selection >> 4) & 1 == 1
    }

    /// Glyphs are emboldened.
    pub fn is_selection_bold(&self) -> bool {
        (self.fs_selection >> 5) & 1 == 1
    }

    /// Glyphs are in the standard weight/style for the font.
    pub fn is_selection_regular(&self) -> bool {
        (self.fs_selection >> 6) & 1 == 1
    }

    /// If set, it is strongly recommended that applications use OS/2.sTypoAscender - OS/2.sTypoDescender + OS/2.sTypoLineGap as the default line spacing for this font.
    pub fn is_selection_use_typo_metrics(&self) -> bool {
        (self.fs_selection >> 7) & 1 == 1
    }

    /// The font has 'name' table strings consistent with a weight/width/slope family without requiring use of name IDs 21 and 22. (See a more detailed description below.)
    pub fn is_selection_wws(&self) -> bool {
        (self.fs_selection >> 8) & 1 == 1
    }

    /// Font contains oblique glyphs.
    pub fn is_selection_oblique(&self) -> bool {
        (self.fs_selection >> 9) & 1 == 1
    }
}

pub(crate) struct OS2Parser<'a> {
    stream: Stream<'a>,
}

impl<'a> OS2Parser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<OS2Table<'a>> {
        let version = self.stream.parse_u16()?;
        match version {
            0x0000 => Some(OS2Table::Version0(self.parse_version_0(version)?)),
            0x0001 => Some(OS2Table::Version1(self.parse_version_1(version)?)),
            0x0002 => Some(OS2Table::Version2(self.parse_version_2(version)?)),
            0x0003 => Some(OS2Table::Version3(self.parse_version_3(version)?)),
            0x0004 => Some(OS2Table::Version4(self.parse_version_4(version)?)),
            0x0005 => Some(OS2Table::Version5(self.parse_version_5(version)?)),
            _ => None,
        }
    }

    pub(crate) fn parse_version_0(&mut self, version: u16) -> Option<OS2Version0<'a>> {
        let x_avg_char_width = self.stream.parse_i16()?;
        let us_weight_class = self.stream.parse_u16()?;
        let us_width_class = self.stream.parse_u16()?;
        let fs_type = self.stream.parse_u16()?;
        let y_subscript_x_size = self.stream.parse_i16()?;
        let y_subscript_y_size = self.stream.parse_i16()?;
        let y_subscript_x_offset = self.stream.parse_i16()?;
        let y_subscript_y_offset = self.stream.parse_i16()?;
        let y_superscript_x_size = self.stream.parse_i16()?;
        let y_superscript_y_size = self.stream.parse_i16()?;
        let y_superscript_x_offset = self.stream.parse_i16()?;
        let y_superscript_y_offset = self.stream.parse_i16()?;
        let y_strikeout_size = self.stream.parse_i16()?;
        let y_strikeout_position = self.stream.parse_i16()?;
        let s_family_class = self.stream.parse_i16()?;
        let panose = self.stream.parse_slice(10)?;
        let ul_unicode_range_1 = self.stream.parse_u32()?;
        let ul_unicode_range_2 = self.stream.parse_u32()?;
        let ul_unicode_range_3 = self.stream.parse_u32()?;
        let ul_unicode_range_4 = self.stream.parse_u32()?;
        let ach_vend_id = self.stream.parse_tag()?;
        let fs_selection = self.stream.parse_u16()?;
        let us_first_char_index = self.stream.parse_u16()?;
        let us_last_char_index = self.stream.parse_u16()?;
        let s_typo_ascender = self.stream.parse_i16()?;
        let s_typo_descender = self.stream.parse_i16()?;
        let s_typo_line_gap = self.stream.parse_i16()?;
        let us_win_ascent = self.stream.parse_u16()?;
        let us_win_descent = self.stream.parse_u16()?;
        Some(OS2Version0 {
            version,
            x_avg_char_width,
            us_weight_class,
            us_width_class,
            fs_type,
            y_subscript_x_size,
            y_subscript_y_size,
            y_subscript_x_offset,
            y_subscript_y_offset,
            y_superscript_x_size,
            y_superscript_y_size,
            y_superscript_x_offset,
            y_superscript_y_offset,
            y_strikeout_size,
            y_strikeout_position,
            s_family_class,
            panose,
            ul_unicode_range_1,
            ul_unicode_range_2,
            ul_unicode_range_3,
            ul_unicode_range_4,
            ach_vend_id,
            fs_selection,
            us_first_char_index,
            us_last_char_index,
            s_typo_ascender,
            s_typo_descender,
            s_typo_line_gap,
            us_win_ascent,
            us_win_descent,
        })
    }
    pub(crate) fn parse_version_1(&mut self, version: u16) -> Option<OS2Version1<'a>> {
        let x_avg_char_width = self.stream.parse_i16()?;
        let us_weight_class = self.stream.parse_u16()?;
        let us_width_class = self.stream.parse_u16()?;
        let fs_type = self.stream.parse_u16()?;
        let y_subscript_x_size = self.stream.parse_i16()?;
        let y_subscript_y_size = self.stream.parse_i16()?;
        let y_subscript_x_offset = self.stream.parse_i16()?;
        let y_subscript_y_offset = self.stream.parse_i16()?;
        let y_superscript_x_size = self.stream.parse_i16()?;
        let y_superscript_y_size = self.stream.parse_i16()?;
        let y_superscript_x_offset = self.stream.parse_i16()?;
        let y_superscript_y_offset = self.stream.parse_i16()?;
        let y_strikeout_size = self.stream.parse_i16()?;
        let y_strikeout_position = self.stream.parse_i16()?;
        let s_family_class = self.stream.parse_i16()?;
        let panose = self.stream.parse_slice(10)?;
        let ul_unicode_range_1 = self.stream.parse_u32()?;
        let ul_unicode_range_2 = self.stream.parse_u32()?;
        let ul_unicode_range_3 = self.stream.parse_u32()?;
        let ul_unicode_range_4 = self.stream.parse_u32()?;
        let ach_vend_id = self.stream.parse_tag()?;
        let fs_selection = self.stream.parse_u16()?;
        let us_first_char_index = self.stream.parse_u16()?;
        let us_last_char_index = self.stream.parse_u16()?;
        let s_typo_ascender = self.stream.parse_i16()?;
        let s_typo_descender = self.stream.parse_i16()?;
        let s_typo_line_gap = self.stream.parse_i16()?;
        let us_win_ascent = self.stream.parse_u16()?;
        let us_win_descent = self.stream.parse_u16()?;
        let ul_code_page_range_1 = self.stream.parse_u32()?;
        let ul_code_page_range_2 = self.stream.parse_u32()?;
        Some(OS2Version1 {
            version,
            x_avg_char_width,
            us_weight_class,
            us_width_class,
            fs_type,
            y_subscript_x_size,
            y_subscript_y_size,
            y_subscript_x_offset,
            y_subscript_y_offset,
            y_superscript_x_size,
            y_superscript_y_size,
            y_superscript_x_offset,
            y_superscript_y_offset,
            y_strikeout_size,
            y_strikeout_position,
            s_family_class,
            panose,
            ul_unicode_range_1,
            ul_unicode_range_2,
            ul_unicode_range_3,
            ul_unicode_range_4,
            ach_vend_id,
            fs_selection,
            us_first_char_index,
            us_last_char_index,
            s_typo_ascender,
            s_typo_descender,
            s_typo_line_gap,
            us_win_ascent,
            us_win_descent,
            ul_code_page_range_1,
            ul_code_page_range_2,
        })
    }
    pub(crate) fn parse_version_2(&mut self, version: u16) -> Option<OS2Version2<'a>> {
        self.parse_version_3(version)
    }
    pub(crate) fn parse_version_3(&mut self, version: u16) -> Option<OS2Version3<'a>> {
        self.parse_version_4(version)
    }
    pub(crate) fn parse_version_4(&mut self, version: u16) -> Option<OS2Version4<'a>> {
        let x_avg_char_width = self.stream.parse_i16()?;
        let us_weight_class = self.stream.parse_u16()?;
        let us_width_class = self.stream.parse_u16()?;
        let fs_type = self.stream.parse_u16()?;
        let y_subscript_x_size = self.stream.parse_i16()?;
        let y_subscript_y_size = self.stream.parse_i16()?;
        let y_subscript_x_offset = self.stream.parse_i16()?;
        let y_subscript_y_offset = self.stream.parse_i16()?;
        let y_superscript_x_size = self.stream.parse_i16()?;
        let y_superscript_y_size = self.stream.parse_i16()?;
        let y_superscript_x_offset = self.stream.parse_i16()?;
        let y_superscript_y_offset = self.stream.parse_i16()?;
        let y_strikeout_size = self.stream.parse_i16()?;
        let y_strikeout_position = self.stream.parse_i16()?;
        let s_family_class = self.stream.parse_i16()?;
        let panose = self.stream.parse_slice(10)?;
        let ul_unicode_range_1 = self.stream.parse_u32()?;
        let ul_unicode_range_2 = self.stream.parse_u32()?;
        let ul_unicode_range_3 = self.stream.parse_u32()?;
        let ul_unicode_range_4 = self.stream.parse_u32()?;
        let ach_vend_id = self.stream.parse_tag()?;
        let fs_selection = self.stream.parse_u16()?;
        let us_first_char_index = self.stream.parse_u16()?;
        let us_last_char_index = self.stream.parse_u16()?;
        let s_typo_ascender = self.stream.parse_i16()?;
        let s_typo_descender = self.stream.parse_i16()?;
        let s_typo_line_gap = self.stream.parse_i16()?;
        let us_win_ascent = self.stream.parse_u16()?;
        let us_win_descent = self.stream.parse_u16()?;
        let ul_code_page_range_1 = self.stream.parse_u32()?;
        let ul_code_page_range_2 = self.stream.parse_u32()?;
        let sx_height = self.stream.parse_i16()?;
        let s_cap_height = self.stream.parse_i16()?;
        let us_default_char = self.stream.parse_u16()?;
        let us_break_char = self.stream.parse_u16()?;
        let us_max_context = self.stream.parse_u16()?;
        Some(OS2Version4 {
            version,
            x_avg_char_width,
            us_weight_class,
            us_width_class,
            fs_type,
            y_subscript_x_size,
            y_subscript_y_size,
            y_subscript_x_offset,
            y_subscript_y_offset,
            y_superscript_x_size,
            y_superscript_y_size,
            y_superscript_x_offset,
            y_superscript_y_offset,
            y_strikeout_size,
            y_strikeout_position,
            s_family_class,
            panose,
            ul_unicode_range_1,
            ul_unicode_range_2,
            ul_unicode_range_3,
            ul_unicode_range_4,
            ach_vend_id,
            fs_selection,
            us_first_char_index,
            us_last_char_index,
            s_typo_ascender,
            s_typo_descender,
            s_typo_line_gap,
            us_win_ascent,
            us_win_descent,
            ul_code_page_range_1,
            ul_code_page_range_2,
            sx_height,
            s_cap_height,
            us_default_char,
            us_break_char,
            us_max_context,
        })
    }
    pub(crate) fn parse_version_5(&mut self, version: u16) -> Option<OS2Version5<'a>> {
        let x_avg_char_width = self.stream.parse_i16()?;
        let us_weight_class = self.stream.parse_u16()?;
        let us_width_class = self.stream.parse_u16()?;
        let fs_type = self.stream.parse_u16()?;
        let y_subscript_x_size = self.stream.parse_i16()?;
        let y_subscript_y_size = self.stream.parse_i16()?;
        let y_subscript_x_offset = self.stream.parse_i16()?;
        let y_subscript_y_offset = self.stream.parse_i16()?;
        let y_superscript_x_size = self.stream.parse_i16()?;
        let y_superscript_y_size = self.stream.parse_i16()?;
        let y_superscript_x_offset = self.stream.parse_i16()?;
        let y_superscript_y_offset = self.stream.parse_i16()?;
        let y_strikeout_size = self.stream.parse_i16()?;
        let y_strikeout_position = self.stream.parse_i16()?;
        let s_family_class = self.stream.parse_i16()?;
        let panose = self.stream.parse_slice(10)?;
        let ul_unicode_range_1 = self.stream.parse_u32()?;
        let ul_unicode_range_2 = self.stream.parse_u32()?;
        let ul_unicode_range_3 = self.stream.parse_u32()?;
        let ul_unicode_range_4 = self.stream.parse_u32()?;
        let ach_vend_id = self.stream.parse_tag()?;
        let fs_selection = self.stream.parse_u16()?;
        let us_first_char_index = self.stream.parse_u16()?;
        let us_last_char_index = self.stream.parse_u16()?;
        let s_typo_ascender = self.stream.parse_i16()?;
        let s_typo_descender = self.stream.parse_i16()?;
        let s_typo_line_gap = self.stream.parse_i16()?;
        let us_win_ascent = self.stream.parse_u16()?;
        let us_win_descent = self.stream.parse_u16()?;
        let ul_code_page_range_1 = self.stream.parse_u32()?;
        let ul_code_page_range_2 = self.stream.parse_u32()?;
        let sx_height = self.stream.parse_i16()?;
        let s_cap_height = self.stream.parse_i16()?;
        let us_default_char = self.stream.parse_u16()?;
        let us_break_char = self.stream.parse_u16()?;
        let us_max_context = self.stream.parse_u16()?;
        let us_lower_optical_point_size = self.stream.parse_u16()?;
        let us_upper_optical_point_size = self.stream.parse_u16()?;
        Some(OS2Version5 {
            version,
            x_avg_char_width,
            us_weight_class,
            us_width_class,
            fs_type,
            y_subscript_x_size,
            y_subscript_y_size,
            y_subscript_x_offset,
            y_subscript_y_offset,
            y_superscript_x_size,
            y_superscript_y_size,
            y_superscript_x_offset,
            y_superscript_y_offset,
            y_strikeout_size,
            y_strikeout_position,
            s_family_class,
            panose,
            ul_unicode_range_1,
            ul_unicode_range_2,
            ul_unicode_range_3,
            ul_unicode_range_4,
            ach_vend_id,
            fs_selection,
            us_first_char_index,
            us_last_char_index,
            s_typo_ascender,
            s_typo_descender,
            s_typo_line_gap,
            us_win_ascent,
            us_win_descent,
            ul_code_page_range_1,
            ul_code_page_range_2,
            sx_height,
            s_cap_height,
            us_default_char,
            us_break_char,
            us_max_context,
            us_lower_optical_point_size,
            us_upper_optical_point_size,
        })
    }
}
impl<'a> Parser<'a> {
    pub fn parse_os2(&self, input: TableRecord) -> Option<OS2Table<'a>> {
        if input.table_tag.is_os2() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = OS2Parser::new(bytes);
            return parser.parse();
        }
        None
    }
}
