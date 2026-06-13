use core::fmt::Debug;

use crate::endian::U32BE;

pub struct Parser<'a> {
    pub(crate) stream: crate::stream::Stream<'a>,
}

impl<'a> Parser<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: crate::stream::Stream::new(bytes),
        }
    }

    pub const fn parse(&mut self) -> Option<TableDirectory<'a>> {
        let sfnt_version = match self.stream.parse_u32() {
            Some(v) => v,
            None => return None,
        };
        let num_tables = match self.stream.parse_u16() {
            Some(v) => v,
            None => return None,
        };
        let search_range = match self.stream.parse_u16() {
            Some(v) => v,
            None => return None,
        };
        let entry_selector = match self.stream.parse_u16() {
            Some(v) => v,
            None => return None,
        };
        let range_shift = match self.stream.parse_u16() {
            Some(v) => v,
            None => return None,
        };
        let table_records = match self.stream.parse_slice(num_tables as usize) {
            Some(v) => v,
            None => return None,
        };
        Some(TableDirectory {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records,
        })
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TableRecord {
    /// Table identifier.
    pub table_tag: crate::tag::Tag,
    /// Checksum for this table.
    pub checksum: U32BE,
    /// Offset from beginning of font file.
    pub offset: U32BE,
    /// Length of this table.
    pub length: U32BE,
}
