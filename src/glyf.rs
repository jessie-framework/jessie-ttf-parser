use core::fmt::Display;

use crate::{
    endian::U16BE,
    f2dot14::F2Dot14,
    loca::LocaOffset,
    parser::{Parser, TableRecord},
    stream::Stream,
};

pub struct GlyfTable<'a> {
    bytes: &'a [u8],
}

impl<'a> GlyfTable<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn index(&self, input: LocaOffset) -> Option<GlyphDescription<'a>> {
        match input.has_no_outline_or_instructions() {
            true => None,
            false => {
                let bytes = &self.bytes[input.start as usize..input.end as usize];
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
    pub fn iter_glyphs(&self) -> ComponentGlyphIter<'a> {
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
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            ended: false,
        }
    }
}

impl<'a> Iterator for ComponentGlyphIter<'a> {
    type Item = ComponentGlyph;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        let flags = ComponentGlyphFlags(self.stream.parse_u16()?);
        let glyph_index = self.stream.parse_u16()?;
        let argument_1;
        let argument_2;
        if flags.args_1_and_2_are_words() {
            argument_1 = self.stream.parse_i16()?;
            argument_2 = self.stream.parse_i16()?;
        } else {
            argument_1 = self.stream.parse_i8()? as i16;
            argument_2 = self.stream.parse_i8()? as i16;
        }

        let mut scale = F2Dot14::default();
        let mut x_scale = F2Dot14::default();
        let mut y_scale = F2Dot14::default();
        let mut scale_01 = F2Dot14::default();
        let mut scale_10 = F2Dot14::default();

        if flags.we_have_a_scale() {
            scale = self.stream.parse_f2_dot_14()?;
        } else if flags.we_have_an_x_and_y_scale() {
            x_scale = self.stream.parse_f2_dot_14()?;
            y_scale = self.stream.parse_f2_dot_14()?;
        } else if flags.we_have_a_two_by_two() {
            x_scale = self.stream.parse_f2_dot_14()?;
            scale_01 = self.stream.parse_f2_dot_14()?;
            scale_10 = self.stream.parse_f2_dot_14()?;
            y_scale = self.stream.parse_f2_dot_14()?;
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ComponentGlyphFlags(u16);

impl ComponentGlyphFlags {
    /// Bit 0: If this is set, the arguments are 16-bit (uint16 or int16); otherwise, they are bytes (uint8 or int8).
    pub fn args_1_and_2_are_words(self) -> bool {
        self.0 & 0x0001 != 0
    }

    /// Bit 1: If this is set, the arguments are signed xy values; otherwise, they are unsigned point numbers.
    pub fn args_are_xy_values(self) -> bool {
        self.0 & 0x0002 != 0
    }

    /// Bit 2: If set and ARGS_ARE_XY_VALUES is also set, the xy values are rounded to the nearest grid line. Ignored if ARGS_ARE_XY_VALUES is not set.
    pub fn round_xy_to_grid(self) -> bool {
        self.0 & 0x0004 != 0
    }

    /// Bit 3: This indicates that there is a simple scale for the component. Otherwise, scale = 1.0.
    pub fn we_have_a_scale(self) -> bool {
        self.0 & 0x0008 != 0
    }

    /// Bit 5: Indicates at least one more glyph after this one.
    pub fn more_components(self) -> bool {
        self.0 & 0x0020 != 0
    }

    /// Bit 6: The x direction will use a different scale from the y direction.
    pub fn we_have_an_x_and_y_scale(self) -> bool {
        self.0 & 0x0040 != 0
    }

    /// Bit 7: There is a 2 by 2 transformation that will be used to scale the component.
    pub fn we_have_a_two_by_two(self) -> bool {
        self.0 & 0x0080 != 0
    }

    /// Bit 8: Following the last component are instructions for the composite glyph.
    pub fn we_have_instructions(self) -> bool {
        self.0 & 0x0100 != 0
    }

    /// Bit 9: If set, this forces the aw and lsb (and rsb) for the composite to be equal to those from this component glyph. This works for hinted and unhinted glyphs.
    pub fn use_my_metrics(self) -> bool {
        self.0 & 0x0200 != 0
    }

    /// Bit 10: If set, the components of the compound glyph overlap. Use of this flag is not required — that is, component glyphs may overlap without having this flag set. When used, it must be set on the flag word for the first component. Some rasterizer implementations may require fonts to use this flag to obtain correct behavior — see additional remarks, above, for the similar OVERLAP_SIMPLE flag used in simple-glyph descriptions.
    pub fn overlap_compound(self) -> bool {
        self.0 & 0x0400 != 0
    }

    /// Bit 11: The composite is designed to have the component offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set.
    pub fn scaled_component_offset(self) -> bool {
        self.0 & 0x0800 != 0
    }

    /// Bit 12: The composite is designed not to have the component offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set.
    pub fn unscaled_component_offset(self) -> bool {
        self.0 & 0x1000 != 0
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SimpleGlyphFlag(u8);

impl SimpleGlyphFlag {
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Bit 0: If set, the point is on the curve; otherwise, it is off the curve.
    pub fn is_on_curve_point(&self) -> bool {
        self.0 & 0x01 != 0
    }

    /// Bit 1: If set, the corresponding x-coordinate is 1 byte long, and the sign is determined by the X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag. If not set, its interpretation depends on the X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag: If that other flag is set, the x-coordinate is the same as the previous x-coordinate, and no element is added to the xCoordinates array. If both flags are not set, the corresponding element in the xCoordinates array is two bytes and interpreted as a signed integer. See the description of the X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR flag for additional information.
    pub fn is_x_short_vector(&self) -> bool {
        self.0 & 0x02 != 0
    }

    /// Bit 2: If set, the corresponding y-coordinate is 1 byte long, and the sign is determined by the Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag. If not set, its interpretation depends on the Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag: If that other flag is set, the y-coordinate is the same as the previous y-coordinate, and no element is added to the yCoordinates array. If both flags are not set, the corresponding element in the yCoordinates array is two bytes and interpreted as a signed integer. See the description of the Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR flag for additional information.
    pub fn is_y_short_vector(&self) -> bool {
        self.0 & 0x04 != 0
    }

    /// Bit 3: If set, the next byte (read as unsigned) specifies the number of additional times this flag byte is to be repeated in the logical flags array — that is, the number of additional logical flag entries inserted after this entry. (In the expanded logical array, this bit is ignored.) In this way, the number of flags listed can be smaller than the number of points in the glyph description.
    pub fn is_repeat_flag(&self) -> bool {
        self.0 & 0x08 != 0
    }

    /// Bit 4: This flag has two meanings, depending on how the X_SHORT_VECTOR flag is set. If X_SHORT_VECTOR is set, this bit describes the sign of the value, with 1 equaling positive and 0 negative. If X_SHORT_VECTOR is not set and this bit is set, then the current x-coordinate is the same as the previous x-coordinate. If X_SHORT_VECTOR is not set and this bit is also not set, the current x-coordinate is a signed 16-bit delta vector.C
    pub fn is_x_is_same_or_positive_x_short_vector(&self) -> bool {
        self.0 & 0x10 != 0
    }

    /// Bit 5: This flag has two meanings, depending on how the Y_SHORT_VECTOR flag is set. If Y_SHORT_VECTOR is set, this bit describes the sign of the value, with 1 equaling positive and 0 negative. If Y_SHORT_VECTOR is not set and this bit is set, then the current y-coordinate is the same as the previous y-coordinate. If Y_SHORT_VECTOR is not set and this bit is also not set, the current y-coordinate is a signed 16-bit delta vector.
    pub fn is_y_is_same_or_positive_y_short_vector(&self) -> bool {
        self.0 & 0x20 != 0
    }

    ///  Bit 6: If set, contours in the glyph description could overlap. Use of this flag is not required — that is, contours may overlap without having this flag set. When used, it must be set on the first flag byte for the glyph. See additional details below.
    pub fn is_overlap_simple(&self) -> bool {
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
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            prev: 0,
        }
    }

    fn next(&mut self, is_short: bool, is_same_or_short: bool) -> i16 {
        let mut n = 0;
        if is_short {
            n = self.stream.parse_u8().unwrap_or(0) as i16;
            if !is_same_or_short {
                n = -n;
            }
        } else if !is_same_or_short {
            n = self.stream.parse_i16().unwrap_or(0);
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
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
            repeat: 0,
            prev: SimpleGlyphFlag(0),
        }
    }
}

impl<'a> Iterator for FlagsIter<'a> {
    type Item = SimpleGlyphFlag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeat == 0 {
            self.prev = SimpleGlyphFlag(self.stream.parse_u8().unwrap_or(0));
            if self.prev.is_repeat_flag() {
                self.repeat = self.stream.parse_u8().unwrap_or(0);
            }
        } else {
            self.repeat -= 1;
        }
        Some(self.prev)
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
    fn new(endpoints: &'a [U16BE]) -> Option<Self> {
        Some(Self {
            endpoints,
            idx: 1,
            left: endpoints.first()?.into_u16(),
        })
    }

    fn next(&mut self) -> bool {
        if self.left == 0 {
            if let Some(end) = self.endpoints.get(self.idx as usize) {
                let prev = self
                    .endpoints
                    .get(self.idx as usize - 1)
                    .unwrap_or(&U16BE(0))
                    .into_u16();
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
    pub(crate) fn new(
        endpoints: &'a [U16BE],
        flags: &'a [u8],
        x_coordinates: &'a [u8],
        y_coordinates: &'a [u8],
        points_total: u16,
    ) -> Option<Self> {
        Some(Self {
            endpoints: EndpointsIter::new(endpoints)?,
            flags: FlagsIter::new(flags),
            x_coordinates: CoordsIter::new(x_coordinates),
            y_coordinates: CoordsIter::new(y_coordinates),
            points_left: points_total,
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
        self.points_left = self.points_left.checked_sub(1)?;

        let last_point = self.endpoints.next();
        let flags = self.flags.next()?;
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

impl<'a> SimpleGlyphDescription<'a> {
    pub fn get_flag(&self, idx: usize) -> Option<SimpleGlyphFlag> {
        self.flags.get(idx).copied().map(SimpleGlyphFlag)
    }

    pub fn iter_points(&self) -> Option<PointsIter<'a>> {
        let total_points = self.end_pts_of_contours.last()?.into_u16().checked_add(1)?;
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
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(bytes),
        }
    }

    pub(crate) fn parse(&mut self) -> Option<GlyphDescription<'a>> {
        let number_of_contours = self.stream.parse_i16()?;
        let x_min = self.stream.parse_i16()?;
        let y_min = self.stream.parse_i16()?;
        let x_max = self.stream.parse_i16()?;
        let y_max = self.stream.parse_i16()?;
        if number_of_contours >= 0 {
            let end_pts_of_contours = self.stream.parse_slice(number_of_contours as usize)?;
            let num_flags = end_pts_of_contours
                .last()
                .unwrap_or(&U16BE(0))
                .into_u16()
                .checked_add(1)?;
            let instruction_length = self.stream.parse_u16()?;
            let instructions = self.stream.parse_slice(instruction_length as usize)?;
            let flags_start = self.stream.idx;
            let (x_len, y_len) = Self::x_y_len(&mut self.stream, num_flags)?;
            let flags_end = self.stream.idx;
            let flags = &self.stream.bytes[flags_start..flags_end];
            let x_coordinates = self.stream.parse_slice(x_len as usize)?;
            let y_coordinates = self.stream.parse_slice(y_len as usize)?;
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

    pub(crate) fn x_y_len(stream: &mut Stream<'a>, num_flags: u16) -> Option<(u32, u32)> {
        let mut flags_left = num_flags as u32;
        let mut x_len = 0;
        let mut y_len = 0;
        while flags_left > 0 {
            let f = SimpleGlyphFlag(stream.parse_u8()?);
            let count = if f.is_repeat_flag() {
                stream.parse_u8()? as u32 + 1
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
    pub fn parse_glyf(&self, input: TableRecord) -> Option<GlyfTable<'a>> {
        if input.table_tag.is_glyf() {
            let bytes = &self.stream.bytes[input.offset.into_u32() as usize
                ..input.offset.into_u32() as usize + input.length.into_u32() as usize];
            return Some(GlyfTable::new(bytes));
        }
        None
    }
}
