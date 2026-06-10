#![no_std]
//! A `const` TrueType parser that is made to keep as much information as possible.
//!
//! ### Usage example :
//!
//! ```rust
//! use jessie_ttf_parser::parser::Parser;
//! fn main() -> Result<(),()> {
//!     let bytes = include_bytes!("path/to/font/file.ttf");
//!     let mut parser = Parser::new(bytes);
//!     let table_directory = parser.parse().ok_or(())?;
//!     for table_record in table_directory.table_records {
//!         println!("This font supports the {} table!",table_record.table_tag);
//!     }
//!     Ok(())
//! }
//! ```

/// BASE table implementation.
pub mod base;
/// cmap table implementation.
pub mod cmap;
/// Singular module for common tables.
pub mod common;
/// cvt table implementation.
pub mod cvt;
/// Various data structures for handling endianness.
pub mod endian;
/// 16-bit signed fixed number with the low 14 bits of fraction (2.14).
pub mod f2dot14;
/// 32-bit signed fixed-point number (16.16)
pub mod fixed;
/// fpgm table implementation.
pub mod fpgm;
/// int16 that describes a quantity in font design units.
pub mod fword;
/// gasp table implementation.
pub mod gasp;
/// GDEF table implementation.
pub mod gdef;
/// glyf table implementation.
pub mod glyf;
/// GPOS table implementation.
pub mod gpos;
/// GSUB table implementation.
pub mod gsub;
/// head table implementation.
pub mod head;
/// hhea table implementation.
pub mod hhea;
/// hmtx table implementation.
pub mod hmtx;
/// loca table implementation.
pub mod loca;
/// Date and time represented in number of seconds since 12:00 midnight, January 1, 1904, UTC. The value is represented as a signed 64-bit integer.
pub mod longdatetime;
/// maxp table implementation.
pub mod maxp;
/// name table implementation.
pub mod name;
/// os2 table implementation.
pub mod os2;
/// Entry point for the parser.
pub mod parser;
/// post table implementation.
pub mod post;
/// prep table implementation.
pub mod prep;
mod stream;
/// SVG table implementation.
pub mod svg;
/// Tags for tables.
pub mod tag;
/// uint16 that describes a quantity in font design units.
pub mod ufword;
/// Utilities for the crate.
mod util;
/// VORG table implementation.
pub mod vorg;
