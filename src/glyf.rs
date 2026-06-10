use core::fmt::Display;

use crate::{
    endian::U16BE,
    f2dot14::F2Dot14,
    loca::LocaOffset,
    parser::{Parser, TableRecord},
    stream::Stream,
    util::{slice_first, slice_get, slice_last, slice_range},
};

pub struct GlyfTable<'a> {
    bytes: &'a [u8],
}

impl<'a> GlyfTable<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub const fn index(&self, input: LocaOffset) -> Option<GlyphDescription<'a>> {
        match input.has_no_outline_or_instructions() {
            true => None,
            false => {
                let bytes = slice_range(self.bytes, input.start as usize..input.end as usize);
                let mut parser = GlyfParser::new(bytes);
                parser.parse()
            }
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
/// The 'glyf' table is comprised of a list of glyph data blocks, each of which provides the description for a single glyph. Glyphs are referenced by identifiers (glyph IDs), which are sequential integers beginning at zero. The total number of glyphs is specified by the numGlyphs field in the 'maxp' table. The 'glyf' table does not include any overall table header or records providing offsets to glyph data blocks. Rather, the 'loca' table provides an array of offsets, indexed by glyph IDs, which provide the location of each glyph data block within the 'glyf' table. Note that the 'glyf' table must always be used in conjunction with the 'loca' and 'maxp' tables. The size of each glyph data block is inferred from the difference between two consecutive offsets in the 'loca' table (with one extra offset provided to give the size of the last glyph data block). As a result of the 'loca' format, glyph data blocks within the 'glyf' table must be in glyph ID order
pub enum GlyphDescription<'a> {
    SimpleGlyphDescription(SimpleGlyphDescription<'a>),
    CompositeGlyphDescription(CompositeGlyphDescription<'a>),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SimpleGlyphDescription<'a> {
    /// If the number of contours is greater than or equal to zero, this is a simple glyph. If negative, this is a composite glyph — the value -1 should be used for composite glyphs.
    pub number_of_contours: i16,
    /// Minimum x for coordinate data.
    pub x_min: i16,
    /// Minimum y for coordinate data.
    pub y_min: i16,
    /// Maximum x for coordinate data.
    pub x_max: i16,
    /// Maximum y for coordinate data.
    pub y_max: i16,
    pub end_pts_of_contours: &'a [U16BE],
    pub instruction_length: u16,
    pub instructions: &'a [u8],
    flags: &'a [u8],
    x_coordinates: &'a [u8],
    y_coordinates: &'a [u8],
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CompositeGlyphDescription<'a> {
    /// If the number of contours is greater than or equal to zero, this is a simple glyph. If negative, this is a composite glyph — the value -1 should be used for composite glyphs.
    pub number_of_contours: i16,
    /// Minimum x for coordinate data.
    pub x_min: i16,
    /// Minimum y for coordinate data.
    pub y_min: i16,
    /// Maximum x for coordinate data.
    pub x_max: i16,
    /// Maximum y for coordinate data.
    pub y_max: i16,
    data: &'a [u8],
}

impl<'a> CompositeGlyphDescription<'a> {
    pub const fn iter_glyphs(&self) -> ComponentGlyphIter<'a> {
        ComponentGlyphIter::new(self.data)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComponentGlyph {
    /// component flag
    pub flags: ComponentGlyphFlags,
    /// glyph index of component
    pub glyph_index: u16,
    /// x-offset for component or point number; type depends on bits 0 and 1 in component flags
    pub argument_1: i16,
    /// y-offset for component or point number; type depends on bits 0 and 1 in component flags
    pub argument_2: i16,
    pub scale: F2Dot14,
    pub x_scale: F2Dot14,
    pub y_scale: F2Dot14,
    pub scale_01: F2Dot14,
    pub scale_10: F2Dot14,
}

pub struct ComponentGlyphIter<'a> {
    stream: Stream<'a>,
    ended: bool,
}

impl<'a> ComponentGlyphIter<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            ended: false,
        }
    }

    pub const fn next(&mut self) -> Option<ComponentGlyph> {
        if self.ended {
            return None;
        }
        let flags = ComponentGlyphFlags(match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        });
        let glyph_index = match self.stream.parse_u16() {
            Some(v) => v,
            _ => return None,
        };
        let argument_1;
        let argument_2;
        if flags.args_1_and_2_are_words() {
            argument_1 = match self.stream.parse_i16() {
                Some(v) => v,
                _ => return None,
            };
            argument_2 = match self.stream.parse_i16() {
                Some(v) => v,
                _ => return None,
            };
        } else {
            argument_1 = match self.stream.parse_i8() {
                Some(v) => v,
                _ => return None,
            } as i16;
            argument_2 = match self.stream.parse_i8() {
                Some(v) => v,
                _ => return None,
            } as i16;
        }

        let mut scale = F2Dot14::default();
        let mut x_scale = F2Dot14::default();
        let mut y_scale = F2Dot14::default();
        let mut scale_01 = F2Dot14::default();
        let mut scale_10 = F2Dot14::default();

        if flags.we_have_a_scale() {
            scale = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
        } else if flags.we_have_an_x_and_y_scale() {
            x_scale = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
            y_scale = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
        } else if flags.we_have_a_two_by_two() {
            x_scale = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
            scale_01 = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
            scale_10 = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
            y_scale = match self.stream.parse_f2_dot_14() {
                Some(v) => v,
                _ => return None,
            };
        }

        if !flags.more_components() {
            self.ended = true;
        }

        Some(ComponentGlyph {
            flags,
            glyph_index,
            argument_1,
            argument_2,
            scale,
            x_scale,
            y_scale,
            scale_01,
            scale_10,
        })
    }
}

impl<'a> Iterator for ComponentGlyphIter<'a> {
    type Item = ComponentGlyph;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ComponentGlyphFlags(u16);

impl ComponentGlyphFlags {
    /// Bit 0: If this is set, the arguments are 16-bit (uint16 or int16); otherwise, they are bytes (uint8 or int8).
    pub const fn args_1_and_2_are_words(self) -> bool {
        self.0 & 0x0001 != 0
    }

    /// Bit 1: If this is set, the arguments are signed xy values; otherwise, they are unsigned point numbers.
    pub const fn args_are_xy_values(self) -> bool {
        self.0 & 0x0002 != 0
    }

    /// Bit 2: If set and ARGS_ARE_XY_VALUES is also set, the xy values are rounded to the nearest grid line. Ignored if ARGS_ARE_XY_VALUES is not set.
    pub const fn round_xy_to_grid(self) -> bool {
        self.0 & 0x0004 != 0
    }

    /// Bit 3: This indicates that there is a simple scale for the component. Otherwise, scale = 1.0.
    pub const fn we_have_a_scale(self) -> bool {
        self.0 & 0x0008 != 0
    }

    /// Bit 5: Indicates at least one more glyph after this one.
    pub const fn more_components(self) -> bool {
        self.0 & 0x0020 != 0
    }

    /// Bit 6: The x direction will use a different scale from the y direction.
    pub const fn we_have_an_x_and_y_scale(self) -> bool {
        self.0 & 0x0040 != 0
    }

    /// Bit 7: There is a 2 by 2 transformation that will be used to scale the component.
    pub const fn we_have_a_two_by_two(self) -> bool {
        self.0 & 0x0080 != 0
    }

    /// Bit 8: Following the last component are instructions for the composite glyph.
    pub const fn we_have_instructions(self) -> bool {
        self.0 & 0x0100 != 0
    }

    /// Bit 9: If set, this forces the aw and lsb (and rsb) for the composite to be equal to those from this component glyph. This works for hinted and unhinted glyphs.
    pub const fn use_my_metrics(self) -> bool {
        self.0 & 0x0200 != 0
    }

    /// Bit 10: If set, the components of the compound glyph overlap. Use of this flag is not required — that is, component glyphs may overlap without having this flag set. When used, it must be set on the flag word for the first component. Some rasterizer implementations may require fonts to use this flag to obtain correct behavior — see additional remarks, above, for the similar OVERLAP_SIMPLE flag used in simple-glyph descriptions.
    pub const fn overlap_compound(self) -> bool {
        self.0 & 0x0400 != 0
    }

    /// Bit 11: The composite is designed to have the component offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set.
    pub const fn scaled_component_offset(self) -> bool {
        self.0 & 0x0800 != 0
    }

    /// Bit 12: The composite is designed not to have the component offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set.
    pub const fn unscaled_component_offset(self) -> bool {
        self.0 & 0x1000 != 0
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SimpleGlyphFlag(u8);

impl SimpleGlyphFlag {
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Bit 0: If set, the point is on the curve; otherwise, it is off the curve.
    pub const fn is_on_curve_point(&self) -> bool {
        self.0 & 0x01 != 0
    }

    /// Bit 1: If set, the corresponding x-coordinate is 1 byte long, and the sign is determined by the X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag. If not set, its interpretation depends on the X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag: If that other flag is set, the x-coordinate is the same as the previous x-coordinate, and no element is added to the xCoordinates array. If both flags are not set, the corresponding element in the xCoordinates array is two bytes and interpreted as a signed integer. See the description of the X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag for additional information.
    pub const fn is_x_short_vector(&self) -> bool {
        self.0 & 0x02 != 0
    }

    /// Bit 2: If set, the corresponding y-coordinate is 1 byte long, and the sign is determined by the Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag. If not set, its interpretation depends on the Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag: If that other flag is set, the y-coordinate is the same as the previous y-coordinate, and no element is added to the yCoordinates array. If both flags are not set, the corresponding element in the yCoordinates array is two bytes and interpreted as a signed integer. See the description of the Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag for additional information.
    pub const fn is_y_short_vector(&self) -> bool {
        self.0 & 0x04 != 0
    }

    /// Bit 3: If set, the next byte (read as unsigned) specifies the number of additional times this flag byte is to be repeated in the logical flags array — that is, the number of additional logical flag entries inserted after this entry. (In the expanded logical array, this bit is ignored.) In this way, the number of flags listed can be smaller than the number of points in the glyph description.
    pub const fn is_repeat_flag(&self) -> bool {
        self.0 & 0x08 != 0
    }

    /// Bit 4: This flag has two meanings, depending on how the X_SHORT_VECTOR flag is set. If X_SHORT_VECTOR is set, this bit describes the sign of the value, with 1 equaling positive and 0 negative. If X_SHORT_VECTOR is not set and this bit is set, then the current x-coordinate is the same as the previous x-coordinate. If X_SHORT_VECTOR is not set and this bit is also not set, the current x-coordinate is a signed 16-bit delta vector.C
    pub const fn is_x_is_same_or_positive_x_short_vector(&self) -> bool {
        self.0 & 0x10 != 0
    }

    /// Bit 5: This flag has two meanings, depending on how the Y_SHORT_VECTOR flag is set. If Y_SHORT_VECTOR is set, this bit describes the sign of the value, with 1 equaling positive and 0 negative. If Y_SHORT_VECTOR is not set and this bit is set, then the current y-coordinate is the same as the previous y-coordinate. If Y_SHORT_VECTOR is not set and this bit is also not set, the current y-coordinate is a signed 16-bit delta vector.
    pub const fn is_y_is_same_or_positive_y_short_vector(&self) -> bool {
        self.0 & 0x20 != 0
    }

    ///  Bit 6: If set, contours in the glyph description could overlap. Use of this flag is not required — that is, contours may overlap without having this flag set. When used, it must be set on the first flag byte for the glyph. See additional details below.
    pub const fn is_overlap_simple(&self) -> bool {
        self.0 & 0x40 != 0
    }
}

impl Display for SimpleGlyphFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_on_curve_point() {
            write!(f, "ON_CURVE_POINT,")?;
        }

        if self.is_x_short_vector() {
            write!(f, "X_SHORT_VECTOR,")?;
        }

        if self.is_y_short_vector() {
            write!(f, "Y_SHORT_VECTOR,")?;
        }

        if self.is_x_is_same_or_positive_x_short_vector() {
            write!(f, "Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR,")?;
        }

        if self.is_overlap_simple() {
            write!(f, "OVERLAP_SIMPLE,")?;
        }

        Ok(())
    }
}

pub(crate) struct CoordsIter<'a> {
    stream: Stream<'a>,
    prev: i16,
}

impl<'a> CoordsIter<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            prev: 0,
        }
    }

    const fn next(&mut self, is_short: bool, is_same_or_short: bool) -> i16 {
        let mut n = 0;
        if is_short {
            n = match self.stream.parse_u8() {
                Some(v) => v,
                _ => 0,
            } as i16;
            if !is_same_or_short {
                n = -n;
            }
        } else if !is_same_or_short {
            n = match self.stream.parse_i16() {
                Some(v) => v,
                _ => 0,
            };
        }
        self.prev = self.prev.wrapping_add(n);
        self.prev
    }
}

pub(crate) struct FlagsIter<'a> {
    stream: Stream<'a>,
    repeat: u8,
    prev: SimpleGlyphFlag,
}

impl<'a> FlagsIter<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            repeat: 0,
            prev: SimpleGlyphFlag(0),
        }
    }

    const fn next(&mut self) -> Option<SimpleGlyphFlag> {
        if self.repeat == 0 {
            self.prev = SimpleGlyphFlag(match self.stream.parse_u8() {
                Some(v) => v,
                _ => 0,
            });
            if self.prev.is_repeat_flag() {
                self.repeat = match self.stream.parse_u8() {
                    Some(v) => v,
                    _ => 0,
                };
            }
        } else {
            self.repeat -= 1;
        }
        Some(self.prev)
    }
}

impl<'a> Iterator for FlagsIter<'a> {
    type Item = SimpleGlyphFlag;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub is_on_curve: bool,
    pub last_point: bool,
}

pub struct EndpointsIter<'a> {
    endpoints: &'a [U16BE],
    idx: u16,
    left: u16,
}

impl<'a> EndpointsIter<'a> {
    const fn new(endpoints: &'a [U16BE]) -> Option<Self> {
        Some(Self {
            endpoints,
            idx: 1,
            left: match slice_first(endpoints) {
                Some(v) => v.into_u16(),
                _ => return None,
            },
        })
    }

    const fn next(&mut self) -> bool {
        if self.left == 0 {
            if let Some(end) = slice_get(self.endpoints, self.idx as usize) {
                let prev = match slice_get(self.endpoints, self.idx as usize - 1) {
                    Some(v) => v.into_u16(),
                    _ => 0,
                };
                self.left = end.into_u16().saturating_sub(prev);
                self.left = self.left.saturating_sub(1);
            }

            if let Some(n) = self.idx.checked_add(1) {
                self.idx = n;
            }

            true
        } else {
            self.left -= 1;
            false
        }
    }
}

impl<'a> PointsIter<'a> {
    pub(crate) const fn new(
        endpoints: &'a [U16BE],
        flags: &'a [u8],
        x_coordinates: &'a [u8],
        y_coordinates: &'a [u8],
        points_total: u16,
    ) -> Option<Self> {
        Some(Self {
            endpoints: match EndpointsIter::new(endpoints) {
                Some(v) => v,
                _ => return None,
            },
            flags: FlagsIter::new(flags),
            x_coordinates: CoordsIter::new(x_coordinates),
            y_coordinates: CoordsIter::new(y_coordinates),
            points_left: points_total,
        })
    }

    pub const fn next(&mut self) -> Option<Point> {
        self.points_left = match self.points_left.checked_sub(1) {
            Some(v) => v,
            _ => return None,
        };

        let last_point = self.endpoints.next();
        let flags = match self.flags.next() {
            Some(v) => v,
            _ => return None,
        };
        Some(Point {
            x: self.x_coordinates.next(
                flags.is_x_short_vector(),
                flags.is_x_is_same_or_positive_x_short_vector(),
            ),
            y: self.y_coordinates.next(
                flags.is_y_short_vector(),
                flags.is_y_is_same_or_positive_y_short_vector(),
            ),
            is_on_curve: flags.is_on_curve_point(),
            last_point,
        })
    }
}

pub struct PointsIter<'a> {
    endpoints: EndpointsIter<'a>,
    flags: FlagsIter<'a>,
    x_coordinates: CoordsIter<'a>,
    y_coordinates: CoordsIter<'a>,
    points_left: u16,
}
impl<'a> Iterator for PointsIter<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a> SimpleGlyphDescription<'a> {
    pub const fn get_flag(&self, idx: usize) -> Option<SimpleGlyphFlag> {
        match slice_get(self.flags, idx) {
            Some(v) => Some(SimpleGlyphFlag(*v)),
            _ => None,
        }
    }

    pub const fn iter_points(&self) -> Option<PointsIter<'a>> {
        let total_points = match slice_last(self.end_pts_of_contours) {
            Some(v) => match v.into_u16().checked_add(1) {
                Some(v) => v,
                _ => return None,
            },
            _ => return None,
        };
        PointsIter::new(
            self.end_pts_of_contours,
            self.flags,
            self.x_coordinates,
            self.y_coordinates,
            total_points,
        )
    }
}

pub(crate) struct GlyfParser<'a> {
    stream: Stream<'a>,
}

impl<'a> GlyfParser<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) const fn parse(&mut self) -> Option<GlyphDescription<'a>> {
        let number_of_contours = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let x_min = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let y_min = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let x_max = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        let y_max = match self.stream.parse_i16() {
            Some(v) => v,
            _ => return None,
        };
        if number_of_contours >= 0 {
            let end_pts_of_contours: &[U16BE] =
                match self.stream.parse_slice(number_of_contours as usize) {
                    Some(v) => v,
                    _ => return None,
                };
            let num_flags = match match slice_last(end_pts_of_contours) {
                Some(v) => v.into_u16(),
                _ => 0,
            }
            .checked_add(1)
            {
                Some(v) => v,
                _ => return None,
            };
            let instruction_length = match self.stream.parse_u16() {
                Some(v) => v,
                _ => return None,
            };
            let instructions = match self.stream.parse_slice(instruction_length as usize) {
                Some(v) => v,
                _ => return None,
            };
            let flags_start = self.stream.idx;
            let (x_len, y_len) = match Self::x_y_len(&mut self.stream, num_flags) {
                Some(v) => v,
                _ => return None,
            };
            let flags_end = self.stream.idx;
            let flags = slice_range(self.stream.bytes, flags_start..flags_end);
            let x_coordinates = match self.stream.parse_slice(x_len as usize) {
                Some(v) => v,
                _ => return None,
            };
            let y_coordinates = match self.stream.parse_slice(y_len as usize) {
                Some(v) => v,
                _ => return None,
            };
            Some(GlyphDescription::SimpleGlyphDescription(
                SimpleGlyphDescription {
                    number_of_contours,
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                    end_pts_of_contours,
                    instruction_length,
                    instructions,
                    flags,
                    x_coordinates,
                    y_coordinates,
                },
            ))
        } else {
            let data = self.stream.parse_slice_rest();
            Some(GlyphDescription::CompositeGlyphDescription(
                CompositeGlyphDescription {
                    number_of_contours,
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                    data,
                },
            ))
        }
    }

    pub(crate) const fn x_y_len(stream: &mut Stream<'a>, num_flags: u16) -> Option<(u32, u32)> {
        let mut flags_left = num_flags as u32;
        let mut x_len = 0;
        let mut y_len = 0;
        while flags_left > 0 {
            let f = SimpleGlyphFlag(match stream.parse_u8() {
                Some(v) => v,
                _ => return None,
            });
            let count = if f.is_repeat_flag() {
                (match stream.parse_u8() {
                    Some(v) => v,
                    _ => return None,
                }) as u32
                    + 1
            } else {
                1
            };

            if count > flags_left {
                return None;
            }

            x_len += (f.0 & 0x02 != 0) as u32 * count;
            x_len += (f.0 & (0x02 | 0x10) == 0) as u32 * (count * 2);
            y_len += (f.0 & 0x04 != 0) as u32 * count;
            y_len += (f.0 & (0x04 | 0x20) == 0) as u32 * (count * 2);
            flags_left -= count;
        }

        Some((x_len, y_len))
    }
}

impl<'a> Parser<'a> {
    pub const fn parse_glyf(&self, input: TableRecord) -> Option<GlyfTable<'a>> {
        if input.table_tag.is_glyf() {
            let bytes = slice_range(
                self.stream.bytes,
                input.offset.into_u32() as usize
                    ..input.offset.into_u32() as usize + input.length.into_u32() as usize,
            );
            return Some(GlyfTable::new(bytes));
        }
        None
    }
}
