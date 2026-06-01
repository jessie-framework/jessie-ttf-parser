use crate::{
    common::{ItemVariationStore, ItemVariationStoreParser},
    endian::U16BE,
    parser::{Parser, TableRecord, Tag},
    stream::Stream,
};

/// The BASE table begins with a header that starts with a version number. Two versions are defined. Version 1.0 contains offsets to horizontal and vertical Axis tables (HorizAxis and VertAxis). Version 1.1 also includes an offset to an ItemVariationStore table.
///
/// Each Axis table stores all baseline information and min/max extents for one layout direction. The HorizAxis table contains Y values for horizontal text layout; the VertAxis table contains X values for vertical text layout.
///
/// A font may supply information for both layout directions. If a font has values for only one text direction, the Axis table offset value for the other direction will be set to NULL.
///
/// The optional ItemVariationStore table is used in variable fonts to provide variation data for BASE metric values within the Axis tables.
pub enum BaseTable<'a> {
    Version1_0(BaseTableVersion1_0<'a>),
    Version1_1(BaseTableVersion1_1<'a>),
}

pub struct BaseTableVersion1_0<'a> {
    /// Major version of the BASE table, = 1.
    pub major_version: u16,
    /// Minor version of the BASE table, = 0<./td>
    pub minor_version: u16,
    /// Offset to horizontal Axis table, from beginning of BASE table (may be NULL).
    pub horiz_axis_offset: u16,
    /// Offset to vertical Axis table, from beginning of BASE table (may be NULL).
    pub vert_axis_offset: u16,
    pub horiz_axis_table: Option<AxisTable<'a>>,
    pub vert_axis_table: Option<AxisTable<'a>>,
}

pub struct BaseTableVersion1_1<'a> {
    /// Major version of the BASE table, = 1.
    pub major_version: u16,
    /// Minor version of the BASE table, = 1.
    pub minor_version: u16,
    /// Offset to horizontal Axis table, from beginning of BASE table (may be NULL).
    pub horiz_axis_offset: u16,
    /// Offset to vertical Axis table, from beginning of BASE table (may be NULL).
    pub vert_axis_offset: u16,
    /// Offset to ItemVariationStore table, from beginning of BASE table (may be null).
    pub item_var_store_offset: u32,
    pub horiz_axis_table: Option<AxisTable<'a>>,
    pub vert_axis_table: Option<AxisTable<'a>>,
    pub item_var_store: Option<ItemVariationStore<'a>>,
}

pub struct AxisTable<'a> {
    /// Offset to BaseTagList table, from beginning of Axis table (may be NULL).
    pub base_tag_list_offset: u16,
    /// Offset to BaseScriptList table, from beginning of Axis table.
    pub base_script_list_offset: u16,
    pub base_tag_list: BaseTagList<'a>,
    pub base_script_list: BaseScriptList<'a>,
}

pub struct BaseTagList<'a> {
    /// Number of baseline identification tags in this text direction — may be zero (0).
    pub base_tag_count: u16,
    /// Array of 4-byte baseline identification tags — must be in alphabetical order.
    pub baseline_tags: &'a [Tag],
}

/// The BaseScriptList table identifies all scripts in the font that are rendered in the same layout direction. If a script is not listed here, then the text-processing client will render the script using the layout information specified for the entire font.
///
/// For each script listed in the BaseScriptList table, a BaseScriptRecord must be defined that identifies the script and references its layout data. BaseScriptRecords are stored in the baseScriptRecords array, ordered alphabetically by the baseScriptTag in each record. The baseScriptCount specifies the total number of BaseScriptRecords in the array.
pub struct BaseScriptList<'a> {
    /// Number of BaseScriptRecords defined.
    pub base_script_count: u16,
    /// Array of BaseScriptRecords, in alphabetical order by baseScriptTag.
    pub base_script_records: &'a [BaseScriptRecord],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct BaseScriptRecord {
    pub base_script_tag: Tag,
    pub base_script_offset: U16BE,
}

pub struct BaseScript<'a> {
    /// Offset to BaseValues table, from beginning of BaseScript table (may be NULL).
    pub base_values_offset: u16,
    /// Offset to MinMax table, from beginning of BaseScript table (may be NULL).
    pub default_min_max_offset: u16,
    /// Number of BaseLangSys records defined — may be zero (0).
    pub base_lang_sys_count: u16,
    /// Array of BaseLangSys records, in alphabetical order by BaseLangSysTag.
    pub base_lang_sys_records: &'a [BaseLangSys],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct BaseLangSys {
    /// 4-byte language system identification tag.
    pub base_lang_sys_tag: Tag,
    /// Offset to MinMax table, from beginning of BaseScript table.
    pub min_max_offset: U16BE,
}

pub struct BaseValues<'a> {
    /// Index number of default baseline for this script — equals index position of baseline tag in baselineTags array of the BaseTagList.
    pub default_baseline_index: u16,
    /// Number of BaseCoord tables defined — should equal baseTagCount in the BaseTagList.
    pub base_coord_count: u16,
    /// Array of offsets to BaseCoord tables, from beginning of BaseValues table — order matches baselineTags array in the BaseTagList.
    pub base_coord_offsets: &'a [U16BE],
}

pub struct MinMax<'a> {
    /// Offset to BaseCoord table that defines the minimum extent value, from the beginning of MinMax table (may be NULL).
    pub min_coord_offset: u16,
    /// Offset to BaseCoord table that defines maximum extent value, from the beginning of MinMax table (may be NULL).
    pub max_coord_offset: u16,
    /// Number of FeatMinMaxRecords — may be zero (0).
    pub feat_min_max_count: u16,
    /// Array of FeatMinMax records, in alphabetical order by featureTag.
    pub feat_min_max_records: &'a [FeatMinMax],
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct FeatMinMax {
    /// 4-byte feature identification tag — must match feature tag in FeatureList.
    pub feature_tag: Tag,
    /// Offset to BaseCoord table that defines the minimum extent value, from beginning of MinMax table (may be NULL).
    pub min_coord_offset: u16,
    /// Offset to BaseCoord table that defines the maximum extent value, from beginning of MinMax table (may be NULL).
    pub max_coord_offset: u16,
}

/// Within the BASE table, a BaseCoord table is used to specify a baseline, a min extent or a max extent. Each BaseCoord table defines one X or Y value:
///
/// If defined within the HorizAxis table, then the BaseCoord table contains a Y value.
/// If defined within the VertAxis table, then the BaseCoord table contains an X value.
///
/// All values are defined in design units, which typically are scaled and rounded to the nearest integer when scaling the glyphs. Values may be negative.
///
/// Three formats available for BaseCoord table data define single X or Y coordinate values in design units. Two of the formats also support fine adjustments to the X or Y values based on a contour point or a Device table. In a variable font, the third format uses a VariationIndex table (a variant of a Device table), as needed, to reference variation data for adjustment of the X or Y values for the current variation instance.
pub enum BaseCoord {
    BaseCoordFormat1(BaseCoordFormat1),
    BaseCoordFormat2(BaseCoordFormat2),
    BaseCoordFormat3(BaseCoordFormat3),
}

/// The first BaseCoord format (BaseCoordFormat1) consists of a format identifier, followed by a single design unit coordinate that specifies the BaseCoord value. This format has the benefits of small size and simplicity, but the BaseCoord value cannot be hinted for fine adjustments at different sizes or device resolutions.
pub struct BaseCoordFormat1 {
    /// Format identifier — format = 1.
    pub format: u16,
    /// X or Y value, in design units.
    pub coordinate: i16,
}

/// The second BaseCoord format (BaseCoordFormat2) specifies the BaseCoord value in design units, but also supplies a glyph index and a contour point for reference. During font hinting, the contour point on the glyph outline may move. The point’s final position after hinting provides the final value for rendering a given font size.
pub struct BaseCoordFormat2 {
    /// Format identifier — format = 2.
    pub format: u16,
    /// X or Y value, in design units.
    pub coordinate: i16,
    /// Glyph ID of control glyph.
    pub reference_glyph: u16,
    /// Index of contour point on the reference glyph.
    pub base_coord_point: u16,
}

/// The third BaseCoord format (BaseCoordFormat3) also specifies the BaseCoord value in design units, but, in a non-variable font, it uses a Device table rather than a contour point to adjust the value. This format offers the advantage of fine-tuning the BaseCoord value for any font size and device resolution. (For more information about Device tables, see the chapter, Common Table Formats.)
///
/// In a variable font, BaseCoordFormat3 must be used to reference variation data to adjust the X or Y value for different variation instances, if needed. In this case, BaseCoordFormat3 specifies an offset to a VariationIndex table, which is a variant of the Device table that is used for referencing variation data.
pub struct BaseCoordFormat3 {
    /// Format identifier — format = 3.
    pub format: u16,
    /// X or Y value, in design units.
    pub coordinate: i16,
    /// Offset to Device table (non-variable font) / Variation Index table (variable font) for X or Y value, from beginning of BaseCoord table (may be NULL).
    pub device_offset: u16,
}

pub(crate) struct BaseParser<'a> {
    stream: Stream<'a>,
}

impl<'a> BaseParser<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<BaseTable<'a>> {
        let major_version = self.stream.parse_u16()?;
        let minor_version = self.stream.parse_u16()?;
        let horiz_axis_offset = self.stream.parse_u16()?;
        let vert_axis_offset = self.stream.parse_u16()?;
        match (major_version, minor_version) {
            (1, 0) => Some(BaseTable::Version1_0(self.parse_base_version_1_0(
                major_version,
                minor_version,
                horiz_axis_offset,
                vert_axis_offset,
            )?)),
            (1, 1) => Some(BaseTable::Version1_1(self.parse_base_version_1_1(
                major_version,
                minor_version,
                horiz_axis_offset,
                vert_axis_offset,
            )?)),
            _ => None,
        }
    }

    pub(crate) fn parse_base_version_1_0(
        &mut self,
        major_version: u16,
        minor_version: u16,
        horiz_axis_offset: u16,
        vert_axis_offset: u16,
    ) -> Option<BaseTableVersion1_0<'a>> {
        let horiz_axis_table = self.parse_axis_table_from_offset(horiz_axis_offset);
        let vert_axis_table = self.parse_axis_table_from_offset(vert_axis_offset);
        Some(BaseTableVersion1_0 {
            major_version,
            minor_version,
            horiz_axis_offset,
            vert_axis_offset,
            horiz_axis_table,
            vert_axis_table,
        })
    }

    pub(crate) fn parse_base_version_1_1(
        &mut self,
        major_version: u16,
        minor_version: u16,
        horiz_axis_offset: u16,
        vert_axis_offset: u16,
    ) -> Option<BaseTableVersion1_1<'a>> {
        let item_var_store_offset = self.stream.parse_u32()?;
        let horiz_axis_table = self.parse_axis_table_from_offset(horiz_axis_offset);
        let vert_axis_table = self.parse_axis_table_from_offset(vert_axis_offset);
        let item_var_store = self.parse_item_var_store_from_offset(item_var_store_offset);
        Some(BaseTableVersion1_1 {
            major_version,
            minor_version,
            horiz_axis_offset,
            vert_axis_offset,
            item_var_store_offset,
            horiz_axis_table,
            vert_axis_table,
            item_var_store,
        })
    }

    pub(crate) fn parse_item_var_store_from_offset(
        &mut self,
        offset: u32,
    ) -> Option<ItemVariationStore<'a>> {
        if offset == 0 {
            return None;
        }
        let mut parser = ItemVariationStoreParser::new(&self.stream.bytes[offset as usize..]);
        parser.parse()
    }

    pub(crate) fn parse_axis_table_from_offset(&mut self, offset: u16) -> Option<AxisTable<'a>> {
        if offset == 0 {
            return None;
        }
        let mut parser = Self::new(&self.stream.bytes[offset as usize..]);
        parser.parse_axis_table()
    }

    pub(crate) fn parse_axis_table(&mut self) -> Option<AxisTable<'a>> {
        let base_tag_list_offset = self.stream.parse_u16()?;
        let base_script_list_offset = self.stream.parse_u16()?;
        let base_tag_list = self.parse_base_tag_list_from_offset(base_script_list_offset)?;
        let base_script_list = self.parse_base_script_list_from_offset(base_script_list_offset)?;
        Some(AxisTable {
            base_tag_list_offset,
            base_script_list_offset,
            base_tag_list,
            base_script_list,
        })
    }

    pub(crate) fn parse_base_script_list_from_offset(
        &mut self,
        offset: u16,
    ) -> Option<BaseScriptList<'a>> {
        let mut parser = Self::new(&self.stream.bytes[offset as usize..]);
        parser.parse_base_script_list()
    }

    pub(crate) fn parse_base_script_list(&mut self) -> Option<BaseScriptList<'a>> {
        let base_script_count = self.stream.parse_u16()?;
        let base_script_records = self.stream.parse_slice(base_script_count as usize)?;
        Some(BaseScriptList {
            base_script_count,
            base_script_records,
        })
    }

    pub(crate) fn parse_base_tag_list_from_offset(
        &mut self,
        offset: u16,
    ) -> Option<BaseTagList<'a>> {
        let mut parser = Self::new(&self.stream.bytes[offset as usize..]);
        parser.parse_base_tag_list()
    }

    pub(crate) fn parse_base_tag_list(&mut self) -> Option<BaseTagList<'a>> {
        let base_tag_count = self.stream.parse_u16()?;
        let baseline_tags = self.stream.parse_slice(base_tag_count as usize)?;
        Some(BaseTagList {
            base_tag_count,
            baseline_tags,
        })
    }
}

impl<'a> Parser<'a> {
    pub fn parse_base(&self, input: TableRecord) -> Option<BaseTable<'a>> {
        if input.table_tag.is_base() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            let mut parser = BaseParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
