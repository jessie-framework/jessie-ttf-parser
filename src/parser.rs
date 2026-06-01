use core::fmt::{Debug, Display};

use crate::{cmap::CmapHeader, endian::U32BE, head::HeadTable, hhea::HheaTable, maxp::MaxpTable};

pub struct Parser<'a> {
    pub(crate) stream: crate::stream::Stream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: crate::stream::Stream::new(bytes),
        }
    }

    pub fn parse(&mut self) -> Option<TableDirectory<'a>> {
        let sfnt_version = self.stream.parse_u32()?;
        let num_tables = self.stream.parse_u16()?;
        let search_range = self.stream.parse_u16()?;
        let entry_selector = self.stream.parse_u16()?;
        let range_shift = self.stream.parse_u16()?;
        let table_records = self.stream.parse_slice(num_tables as usize)?;
        Some(TableDirectory {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records,
        })
    }

    pub fn parse_table(&mut self, table: TableRecord) -> Option<Table<'a>> {
        if table.table_tag.is_cmap() {
            return Some(Table::Cmap(self.parse_cmap(table)?));
        }
        if table.table_tag.is_head() {
            return Some(Table::Head(self.parse_head(table)?));
        }
        if table.table_tag.is_hhea() {
            return Some(Table::Hhea(self.parse_hhea(table)?));
        }
        if table.table_tag.is_maxp() {
            return Some(Table::Maxp(self.parse_maxp(table)?));
        }
        None
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TableDirectory<'a> {
    pub sfnt_version: u32,
    /// Number of tables.
    pub num_tables: u16,
    /// Maximum power of 2 less than or equal to numTables, times 16 ((2**floor(log2(numTables))) * 16, where “**” is an exponentiation operator).
    pub search_range: u16,
    /// Log2 of the maximum power of 2 less than or equal to numTables (log2(searchRange/16), which is equal to floor(log2(numTables))).
    pub entry_selector: u16,
    /// numTables times 16, minus searchRange ((numTables * 16) - searchRange).
    pub range_shift: u16,
    /// Table records array—one for each top-level table in the font.
    pub table_records: &'a [TableRecord],
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TableRecord {
    /// Table identifier.
    pub table_tag: Tag,
    /// Checksum for this table.
    pub checksum: U32BE,
    /// Offset from beginning of font file.
    pub offset: U32BE,
    /// Length of this table.
    pub length: U32BE,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Tag(pub(crate) [u8; 4]);

impl Tag {
    pub fn as_bytes(self) -> [u8; 4] {
        self.0
    }

    /// Character to glyph mapping
    pub fn is_cmap(self) -> bool {
        &self.as_bytes() == b"cmap"
    }

    /// Font header
    pub fn is_head(self) -> bool {
        &self.as_bytes() == b"head"
    }

    /// Horizontal header
    pub fn is_hhea(self) -> bool {
        &self.as_bytes() == b"hhea"
    }

    /// Horizontal metrics
    pub fn is_hmtx(self) -> bool {
        &self.as_bytes() == b"hmtx"
    }

    /// Maximum profile
    pub fn is_maxp(self) -> bool {
        &self.as_bytes() == b"maxp"
    }

    /// Naming table
    pub fn is_name(self) -> bool {
        &self.as_bytes() == b"name"
    }

    /// OS/2 and Windows specific metrics
    pub fn is_os2(self) -> bool {
        &self.as_bytes() == b"OS/2"
    }

    /// PostScript information
    pub fn is_post(self) -> bool {
        &self.as_bytes() == b"post"
    }

    /// Control Value Table (optional table)
    pub fn is_cvt(self) -> bool {
        &self.as_bytes() == b"cvt "
    }

    /// Font program (optional table)
    pub fn is_fpgm(self) -> bool {
        &self.as_bytes() == b"fpgm"
    }

    /// Glyph data
    pub fn is_glyf(self) -> bool {
        &self.as_bytes() == b"glyf"
    }

    /// Index to location
    pub fn is_loca(self) -> bool {
        &self.as_bytes() == b"loca"
    }

    /// Control Value Program (optional table)
    pub fn is_prep(self) -> bool {
        &self.as_bytes() == b"prep"
    }

    /// Grid-fitting/Scan-conversion (optional table)
    pub fn is_gasp(self) -> bool {
        &self.as_bytes() == b"gasp"
    }

    /// Compact Font Format 1.0
    pub fn is_cff(self) -> bool {
        &self.as_bytes() == b"CFF "
    }

    /// Compact Font Format 2.0
    pub fn is_cff2(self) -> bool {
        &self.as_bytes() == b"CFF2"
    }

    /// Vertical Origin (optional table)
    pub fn is_vorg(self) -> bool {
        &self.as_bytes() == b"VORG"
    }

    /// The SVG (Scalable Vector Graphics) table
    pub fn is_svg(self) -> bool {
        &self.as_bytes() == b"SVG "
    }

    /// Embedded bitmap data
    pub fn is_ebdt(self) -> bool {
        &self.as_bytes() == b"EBDT"
    }

    /// Embedded bitmap location data
    pub fn is_eblc(self) -> bool {
        &self.as_bytes() == b"EBLC"
    }

    /// Embedded bitmap scaling data
    pub fn is_ebsc(self) -> bool {
        &self.as_bytes() == b"EBSC"
    }

    /// Color bitmap data
    pub fn is_cbdt(self) -> bool {
        &self.as_bytes() == b"CBDT"
    }

    /// Color bitmap location data
    pub fn is_cblc(self) -> bool {
        &self.as_bytes() == b"CBLC"
    }

    /// Standard bitmap graphics
    pub fn is_sbix(self) -> bool {
        &self.as_bytes() == b"sbix"
    }

    /// Baseline data
    pub fn is_base(self) -> bool {
        &self.as_bytes() == b"BASE"
    }

    /// Glyph definition data
    pub fn is_gdef(self) -> bool {
        &self.as_bytes() == b"GDEF"
    }

    /// Glyph positioning data
    pub fn is_gpos(self) -> bool {
        &self.as_bytes() == b"GPOS"
    }

    /// Glyph substitution data
    pub fn is_gsub(self) -> bool {
        &self.as_bytes() == b"GSUB"
    }

    /// Justification data
    pub fn is_jstf(self) -> bool {
        &self.as_bytes() == b"JSTF"
    }

    /// Math layout data
    pub fn is_math(self) -> bool {
        &self.as_bytes() == b"MATH"
    }

    /// Axis variations
    pub fn is_avar(self) -> bool {
        &self.as_bytes() == b"avar"
    }

    /// CVT variations (TrueType outlines only)
    pub fn is_cvar(self) -> bool {
        &self.as_bytes() == b"cvar"
    }

    /// Font variations
    pub fn is_fvar(self) -> bool {
        &self.as_bytes() == b"fvar"
    }

    /// Glyph variations (TrueType outlines only)
    pub fn is_gvar(self) -> bool {
        &self.as_bytes() == b"gvar"
    }

    /// Horizontal metrics variations
    pub fn is_hvar(self) -> bool {
        &self.as_bytes() == b"HVAR"
    }

    /// Metrics variations
    pub fn is_mvar(self) -> bool {
        &self.as_bytes() == b"MVAR"
    }

    /// Style attributes (required for variable fonts, optional for non-variable fonts)
    pub fn is_stat(self) -> bool {
        &self.as_bytes() == b"STAT"
    }

    /// Vertical metrics variations
    pub fn is_vvar(self) -> bool {
        &self.as_bytes() == b"VVAR"
    }

    /// Color table
    pub fn is_colr(self) -> bool {
        &self.as_bytes() == b"COLR"
    }

    /// Color palette table
    pub fn is_cpal(self) -> bool {
        &self.as_bytes() == b"CPAL"
    }

    /// Color bitmap data
    pub fn is_dsig(self) -> bool {
        &self.as_bytes() == b"DSIG"
    }

    /// Color bitmap location data
    pub fn is_hdmx(self) -> bool {
        &self.as_bytes() == b"hdmx"
    }

    /// Kerning
    pub fn is_kern(self) -> bool {
        &self.as_bytes() == b"hdmx"
    }

    /// Linear threshold data
    pub fn is_ltsh(self) -> bool {
        &self.as_bytes() == b"LTSH"
    }

    /// Merge
    pub fn is_merg(self) -> bool {
        &self.as_bytes() == b"MERG"
    }

    /// Metadata
    pub fn is_meta(self) -> bool {
        &self.as_bytes() == b"meta"
    }

    /// PCL 5 data
    pub fn is_pclt(self) -> bool {
        &self.as_bytes() == b"PCLT"
    }

    /// Vertical device metrics
    pub fn is_vdmx(self) -> bool {
        &self.as_bytes() == b"VDMX"
    }

    /// Vertical Metrics header
    pub fn is_vhea(self) -> bool {
        &self.as_bytes() == b"vhea"
    }

    /// Vertical Metrics
    pub fn is_vmtx(self) -> bool {
        &self.as_bytes() == b"vmtx"
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Ok(v) = core::str::from_utf8(&self.as_bytes()) {
            write!(f, "{v}")
        } else {
            write!(f, "")
        }
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

pub enum Table<'a> {
    Cmap(CmapHeader<'a>),
    Head(HeadTable),
    Hhea(HheaTable),
    Maxp(MaxpTable),
}
