use core::fmt::{Debug, Display};
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Tag(pub(crate) [u8; 4]);

impl Tag {
    #[inline]
    pub const fn as_bytes(self) -> [u8; 4] {
        self.0
    }

    #[inline(always)]
    const fn matches_bytes(self, input: &[u8]) -> bool {
        let bytes = self.as_bytes();
        input[0] == bytes[0] && input[1] == bytes[1] && input[2] == bytes[2] && input[3] == bytes[3]
    }

    /// Character to glyph mapping
    #[inline]
    pub const fn is_cmap(self) -> bool {
        self.matches_bytes(b"cmap")
    }

    /// Font header
    #[inline]
    pub const fn is_head(self) -> bool {
        self.matches_bytes(b"head")
    }

    /// Horizontal header
    #[inline]
    pub const fn is_hhea(self) -> bool {
        self.matches_bytes(b"hhea")
    }

    /// Horizontal metrics
    #[inline]
    pub const fn is_hmtx(self) -> bool {
        self.matches_bytes(b"hmtx")
    }

    /// Maximum profile
    #[inline]
    pub const fn is_maxp(self) -> bool {
        self.matches_bytes(b"maxp")
    }

    /// Naming table
    #[inline]
    pub const fn is_name(self) -> bool {
        self.matches_bytes(b"name")
    }

    /// OS/2 and Windows specific metrics
    #[inline]
    pub const fn is_os2(self) -> bool {
        self.matches_bytes(b"OS/2")
    }

    /// PostScript information
    #[inline]
    pub const fn is_post(self) -> bool {
        self.matches_bytes(b"post")
    }

    /// Control Value Table (optional table)
    #[inline]
    pub const fn is_cvt(self) -> bool {
        self.matches_bytes(b"cvt ")
    }

    /// Font program (optional table)
    #[inline]
    pub const fn is_fpgm(self) -> bool {
        self.matches_bytes(b"fpgm")
    }

    /// Glyph data
    #[inline]
    pub const fn is_glyf(self) -> bool {
        self.matches_bytes(b"glyf")
    }

    /// Index to location
    #[inline]
    pub const fn is_loca(self) -> bool {
        self.matches_bytes(b"loca")
    }

    /// Control Value Program (optional table)
    #[inline]
    pub const fn is_prep(self) -> bool {
        self.matches_bytes(b"prep")
    }

    /// Grid-fitting/Scan-conversion (optional table)
    #[inline]
    pub const fn is_gasp(self) -> bool {
        self.matches_bytes(b"gasp")
    }

    /// Compact Font Format 1.0
    #[inline]
    pub const fn is_cff(self) -> bool {
        self.matches_bytes(b"CFF ")
    }

    /// Compact Font Format 2.0
    #[inline]
    pub const fn is_cff2(self) -> bool {
        self.matches_bytes(b"CFF2")
    }

    /// Vertical Origin (optional table)
    #[inline]
    pub const fn is_vorg(self) -> bool {
        self.matches_bytes(b"VORG")
    }

    /// The SVG (Scalable Vector Graphics) table
    #[inline]
    pub const fn is_svg(self) -> bool {
        self.matches_bytes(b"SVG ")
    }

    /// Embedded bitmap data
    #[inline]
    pub const fn is_ebdt(self) -> bool {
        self.matches_bytes(b"EBDT")
    }

    /// Embedded bitmap location data
    #[inline]
    pub const fn is_eblc(self) -> bool {
        self.matches_bytes(b"EBLC")
    }

    /// Embedded bitmap scaling data
    #[inline]
    pub const fn is_ebsc(self) -> bool {
        self.matches_bytes(b"EBSC")
    }

    /// Color bitmap data
    #[inline]
    pub const fn is_cbdt(self) -> bool {
        self.matches_bytes(b"CBDT")
    }

    /// Color bitmap location data
    #[inline]
    pub const fn is_cblc(self) -> bool {
        self.matches_bytes(b"CBLC")
    }

    /// Standard bitmap graphics
    #[inline]
    pub const fn is_sbix(self) -> bool {
        self.matches_bytes(b"sbix")
    }

    /// Baseline data
    #[inline]
    pub const fn is_base(self) -> bool {
        self.matches_bytes(b"BASE")
    }

    /// Glyph definition data
    #[inline]
    pub const fn is_gdef(self) -> bool {
        self.matches_bytes(b"GDEF")
    }

    /// Glyph positioning data
    #[inline]
    pub const fn is_gpos(self) -> bool {
        self.matches_bytes(b"GPOS")
    }

    /// Glyph substitution data
    #[inline]
    pub const fn is_gsub(self) -> bool {
        self.matches_bytes(b"GSUB")
    }

    /// Justification data
    #[inline]
    pub const fn is_jstf(self) -> bool {
        self.matches_bytes(b"JSTF")
    }

    /// Math layout data
    #[inline]
    pub fn is_math(self) -> bool {
        self.matches_bytes(b"MATH")
    }

    /// Axis variations
    #[inline]
    pub const fn is_avar(self) -> bool {
        self.matches_bytes(b"avar")
    }

    /// CVT variations (TrueType outlines only)
    #[inline]
    pub const fn is_cvar(self) -> bool {
        self.matches_bytes(b"cvar")
    }

    /// Font variations
    #[inline]
    pub const fn is_fvar(self) -> bool {
        self.matches_bytes(b"fvar")
    }

    /// Glyph variations (TrueType outlines only)
    #[inline]
    pub const fn is_gvar(self) -> bool {
        self.matches_bytes(b"gvar")
    }

    /// Horizontal metrics variations
    #[inline]
    pub const fn is_hvar(self) -> bool {
        self.matches_bytes(b"HVAR")
    }

    /// Metrics variations
    #[inline]
    pub const fn is_mvar(self) -> bool {
        self.matches_bytes(b"MVAR")
    }

    /// Style attributes (required for variable fonts, optional for non-variable fonts)
    #[inline]
    pub const fn is_stat(self) -> bool {
        self.matches_bytes(b"STAT")
    }

    /// Vertical metrics variations
    #[inline]
    pub const fn is_vvar(self) -> bool {
        self.matches_bytes(b"VVAR")
    }

    /// Color table
    #[inline]
    pub const fn is_colr(self) -> bool {
        self.matches_bytes(b"COLR")
    }

    /// Color palette table
    #[inline]
    pub const fn is_cpal(self) -> bool {
        self.matches_bytes(b"CPAL")
    }

    /// Color bitmap data
    #[inline]
    pub const fn is_dsig(self) -> bool {
        self.matches_bytes(b"DSIG")
    }

    /// Color bitmap location data
    #[inline]
    pub fn is_hdmx(self) -> bool {
        self.matches_bytes(b"hdmx")
    }

    /// Kerning
    #[inline]
    pub fn is_kern(self) -> bool {
        &self.as_bytes() == b"kern"
    }

    /// Linear threshold data
    pub const fn is_ltsh(self) -> bool {
        self.matches_bytes(b"LTSH")
    }

    /// Merge
    #[inline]
    pub const fn is_merg(self) -> bool {
        self.matches_bytes(b"MERG")
    }

    /// Metadata
    #[inline]
    pub fn is_meta(self) -> bool {
        self.matches_bytes(b"meta")
    }

    /// PCL 5 data
    #[inline]
    pub const fn is_pclt(self) -> bool {
        self.matches_bytes(b"PCLT")
    }

    /// Vertical device metrics
    #[inline]
    pub const fn is_vdmx(self) -> bool {
        self.matches_bytes(b"VDMX")
    }

    /// Vertical Metrics header
    #[inline]
    pub const fn is_vhea(self) -> bool {
        self.matches_bytes(b"vhea")
    }

    /// Vertical Metrics
    #[inline]
    pub const fn is_vmtx(self) -> bool {
        self.matches_bytes(b"vmtx")
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
