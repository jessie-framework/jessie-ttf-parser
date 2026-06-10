use crate::{
    att,
    endian::{U16BE, U32BE},
    parser::{Parser, TableRecord},
    stream::Stream,
    util::{slice_range, slice_rest},
};

#[repr(C)]
#[derive(Debug, Clone)]
/// OpenType provides various formats for color fonts, one of which is the SVG table. The SVG table provides the benefits of supporting scalable color graphics using the Scalable Vector Graphics markup language, a vector graphics file format that is widely used on the Web and that provides rich graphics capabilities, such as gradients.
///
/// Compared to the other color formats, the SVG table also provides certain other benefits:
///
/// SVG allows for raster artwork as well as vectors, and also allows for combining raster and vector elements in a color glyph. The other formats allow for one or the other, but not both.
/// The SVG description for a color glyph is a complete, integrated piece of artwork; no additional glyphs are required for the SVG description. In the COLR table, however, a color glyph may require components that have separate glyph IDs. The conservation of 16-bit glyph IDs when using the SVG table, in comparison with the COLR table, may be beneficial for fonts that support a large number of characters.
///
/// There are certain trade-offs, however. File sizes are often larger when using the SVG table than for the other color formats. Also, glyph outlines used in the COLR table can be variable (in a variable font) and can also be hinted, whereas these capabilities are not supported with the SVG table.
///
/// SVG was developed for use in environments that allow for a rich set of functionality, including leveraging the full functionality of Cascading Style Sheets for styling, and programmatic manipulation of graphics objects using the SVG Document Object Model. Adoption of SVG for use in OpenType does not entail wholesale incorporation of all SVG capabilities. Text-rendering engines typically have more stringent security, performance and architectural requirements than general-purpose SVG engines. For this reason, when used within OpenType fonts, the expressiveness of the language is limited and simplified to be appropriate for environments in which font processing and text layout occurs.
///
/// The SVG table is optional, and may be used in OpenType fonts with TrueType, CFF or CFF2 outlines. For every SVG glyph description, there must be a corresponding TrueType, CFF or CFF2 glyph description in the font.
pub struct SVGTable<'a> {
    /// Table version (starting at 0). Set to 0.
    pub version: u16,
    /// Offset to the SVGDocumentList, from the start of the SVG table. Must be non-zero.
    pub svg_document_list_offset: u32,
    /// Set to 0.
    pub reserved: u32,
    pub svg_document_list: SVGDocumentList<'a>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SVGDocumentList<'a> {
    /// Number of SVGDocumentRecords. Must be non-zero.
    pub num_entries: u16,
    /// Array of SVGDocumentRecords.
    pub document_records: &'a [SVGDocumentRecord],
    data: &'a [u8],
}

impl<'a> SVGDocumentList<'a> {
    pub const fn parse_document_record(&self, input: SVGDocumentRecord) -> Option<&'a str> {
        let bytes = slice_rest(self.data, input.svg_doc_offset.into_u32() as usize);
        let mut stream = Stream::new(bytes);
        stream.parse_utf8(input.svg_doc_length.into_u32() as usize)
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct SVGDocumentRecord {
    /// The first glyph ID for the range covered by this record.
    pub start_glyph_id: U16BE,
    /// The last glyph ID for the range covered by this record.
    pub end_glyph_id: U16BE,
    /// Offset from the beginning of the SVGDocumentList to an SVG document. Must be non-zero.
    pub svg_doc_offset: U32BE,
    /// Length of the SVG document data. Must be non-zero.
    pub svg_doc_length: U32BE,
}

pub(crate) struct SVGParser<'a> {
    stream: Stream<'a>,
}

impl<'a> SVGParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<SVGTable<'a>> {
        let version = att!(self.stream.parse_u16());
        let svg_document_list_offset = att!(self.stream.parse_u32());
        let reserved = att!(self.stream.parse_u32());
        let mut parser = Self::new(slice_rest(
            self.stream.bytes,
            svg_document_list_offset as usize,
        ));
        let svg_document_list = att!(parser.parse_svg_document_list());
        Some(SVGTable {
            version,
            svg_document_list_offset,
            reserved,
            svg_document_list,
        })
    }

    pub(crate) const fn parse_svg_document_list(&mut self) -> Option<SVGDocumentList<'a>> {
        let num_entries = att!(self.stream.parse_u16());
        let document_records = att!(self.stream.parse_slice(num_entries as usize));
        let data = self.stream.bytes;
        Some(SVGDocumentList {
            num_entries,
            document_records,
            data,
        })
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_svg(&self, input: TableRecord) -> Option<SVGTable<'a>> {
        if input.table_tag.is_svg() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            let mut parser = SVGParser::new(bytes);
            return parser.parse();
        }
        None
    }
}
