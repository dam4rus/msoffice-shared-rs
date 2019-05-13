use crate::error::AdjustParseError;
use enum_from_str::ParseEnumVariantError;
use enum_from_str_derive::FromStr;
use std::str::FromStr;

/// This simple type specifies that its values shall be a 128-bit globally unique identifier (GUID) value.
///
/// This simple type's contents shall match the following regular expression pattern:
/// \{[0-9A-F]{8}-[0-9AF]{/// 4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\}.
pub type Guid = String;

/// This simple type specifies that its contents will contain a percentage value. See the union's member types for
/// details.
pub type Percentage = f32;

/// This simple type specifies that its contents will contain a positive percentage value. See the union's member
/// types for details.
pub type PositivePercentage = f32;

/// This simple type specifies that its contents will contain a positive percentage value from zero through one
/// hundred percent.
///
/// Values represented by this type are restricted to: 0 <= n <= 100000
pub type PositiveFixedPercentage = f32;

/// This simple type represents a fixed percentage from negative one hundred to positive one hundred percent. See
/// the union's member types for details.
///
/// Values represented by this type are restricted to: -100000 <= n <= 100000
pub type FixedPercentage = f32;

/// This simple type specifies that its contents shall contain a color value in RRGGBB hexadecimal format, specified
/// using six hexadecimal digits. Each of the red, green, and blue color values, from 0-255, is encoded as two
/// hexadecimal digits.
///
/// # Example
/// Consider a color defined as follows:
///
/// Red:   122
/// Green:  23
/// Blue:  209
///
/// The resulting RRGGBB value would be 7A17D1, as each color is transformed into its hexadecimal equivalent.
pub type HexColorRGB = String;

/// This simple type represents a one dimensional position or length as either:
///
/// * EMUs.
/// * A number followed immediately by a unit identifier.
pub type Coordinate = i64;

/// This simple type represents a positive position or length in EMUs.
pub type PositiveCoordinate = u64;

/// This simple type specifies a coordinate within the document. This can be used for measurements or spacing; its
/// maximum size is 2147483647 EMUs.
///
/// Its contents can contain either:
///
/// * A whole number, whose contents consist of a measurement in EMUs (English Metric Units)
/// * A number immediately followed by a unit identifier
pub type Coordinate32 = i32;

/// This simple type specifies the a positive coordinate point that has a maximum size of 32 bits.
///
/// The units of measurement used here are EMUs (English Metric Units).
pub type PositiveCoordinate32 = u32;

/// This simple type specifies the width of a line in EMUs. 1 pt = 12700 EMUs
///
/// Values represented by this type are restricted to: 0 <= n <= 20116800
pub type LineWidth = Coordinate32;

/// This simple type specifies a unique integer identifier for each drawing element.
pub type DrawingElementId = u32;

/// This simple type represents an angle in 60,000ths of a degree. Positive angles are clockwise (i.e., towards the
/// positive y axis); negative angles are counter-clockwise (i.e., towards the negative y axis).
pub type Angle = i32;

/// This simple type represents a fixed range angle in 60000ths of a degree. Range from (-90, 90 degrees).
///
/// Values represented by this type are restricted to: -5400000 <= n <= 5400000
pub type FixedAngle = Angle;

/// This simple type represents a positive angle in 60000ths of a degree. Range from [0, 360 degrees).
///
/// Values represented by this type are restricted to: 0 <= n <= 21600000
pub type PositiveFixedAngle = Angle;

/// This simple type specifies a geometry guide name.
pub type GeomGuideName = String;

/// This simple type specifies a geometry guide formula.
pub type GeomGuideFormula = String;

/// This simple type specifies an index into one of the lists in the style matrix specified by the
/// BaseStyles::format_scheme element (StyleMatrix::bg_fill_style_list, StyleMatrix::effect_style_list,
/// StyleMatrix::fill_style_list, or StyleMatrix::line_style_list).
pub type StyleMatrixColumnIndex = u32;

/// This simple type specifies the number of columns.
///
/// Values represented by this type are restricted to: 1 <= n <= 16
pub type TextColumnCount = i32;

/// Values represented by this type are restricted to: 1000 <= n <= 100000
pub type TextFontScalePercent = Percentage;

/// Values represented by this type are restricted to: 0 <= n <= 13200000
pub type TextSpacingPercent = Percentage;

/// This simple type specifies the Text Spacing that is used in terms of font point size.
///
/// Values represented by this type are restricted to: 0 <= n <= 158400
pub type TextSpacingPoint = i32;

/// This simple type specifies the margin that is used and its corresponding size.
///
/// Values represented by this type are restricted to: 0 <= n <= 51206400
pub type TextMargin = Coordinate32;

/// This simple type specifies the text indentation amount to be used.
///
/// Values represented by this type are restricted to: -51206400 <= n <= 51206400
pub type TextIndent = Coordinate32;

/// This simple type specifies the indent level type. We support list level 0 to 8, and we use -1 and -2 for outline
/// mode levels that should only exist in memory.
///
/// Values represented by this type are restricted to: 0 <= n <= 8
pub type TextIndentLevelType = i32;

/// This simple type specifies the range that the bullet percent can be. A bullet percent is the size of the bullet with
/// respect to the text that should follow it.
///
/// Values represented by this type are restricted to: 25000 <= n <= 400000
pub type TextBulletSizePercent = Percentage;

/// This simple type specifies the size of any text in hundredths of a point. Shall be at least 1 point.
///
/// Values represented by this type are restricted to: 100 <= n <= 400000
pub type TextFontSize = i32;

/// This simple type specifies the way we represent a font typeface.
pub type TextTypeFace = String;

/// Specifies a language tag as defined by RFC 3066. See simple type for additional information.
pub type TextLanguageID = String;

/// This simple type specifies a number consisting of 20 hexadecimal digits which defines the Panose-1 font
/// classification.
///
/// This simple type's contents have a length of exactly 20 hexadecimal digit(s).
///
/// # Xml example
///
/// ```xml
/// <w:font w:name="Times New Roman">
///   <w:panose1 w:val="02020603050405020304" />
///   …
/// </w:font>
/// ```
pub type Panose = String; // TODO: hex, length=10

/// This simple type specifies the range that the start at number for a bullet's auto-numbering sequence can begin
/// at. When the numbering is alphabetical, then the numbers map to the appropriate letter. 1->a, 2->b, etc. If the
/// numbers go above 26, then the numbers begin to double up. For example, 27->aa and 53->aaa.
///
/// Values represented by this type are restricted to: 1 <= n <= 32767
pub type TextBulletStartAtNum = i32;

/// This simple type specifies that its contents contains a language identifier as defined by RFC 4646/BCP 47.
///
/// The contents of this language are interpreted based on the context of the parent XML element.
///
/// # Xml example
///
/// ```xml
/// <w:lang w:val="en-CA" />
/// ```
///
/// This language is therefore specified as English (en) and Canada (CA), resulting in use of the English (Canada)
/// language setting.
pub type Lang = String;

/// This simple type specifies a non-negative font size in hundredths of a point.
///
/// Values represented by this type are restricted to: 0 <= n <= 400000
pub type TextNonNegativePoint = i32;

/// This simple type specifies a coordinate within the document. This can be used for measurements or spacing
///
/// Values represented by this type are restricted to: -400000 <= n <= 400000
pub type TextPoint = i32;

/// Specifies the shape ID for legacy shape identification purposes.
pub type ShapeId = String;

/// This simple type is an adjustable coordinate is either an absolute coordinate position or a reference to a
/// geometry guide.
#[derive(Debug, Clone)]
pub enum AdjCoordinate {
    Coordinate(Coordinate),
    GeomGuideName(GeomGuideName),
}

impl FromStr for AdjCoordinate {
    type Err = AdjustParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s.parse::<Coordinate>() {
            Ok(coord) => Ok(AdjCoordinate::Coordinate(coord)),
            Err(_) => Ok(AdjCoordinate::GeomGuideName(GeomGuideName::from(s))),
        }
    }
}

/// This simple type is an adjustable angle, either an absolute angle or a reference to a geometry guide. The units
/// for an adjustable angle are 60,000ths of a degree.
#[derive(Debug, Clone)]
pub enum AdjAngle {
    Angle(Angle),
    GeomGuideName(GeomGuideName),
}

impl FromStr for AdjAngle {
    type Err = AdjustParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s.parse::<Angle>() {
            Ok(angle) => Ok(AdjAngle::Angle(angle)),
            Err(_) => Ok(AdjAngle::GeomGuideName(GeomGuideName::from(s))),
        }
    }
}

/// This simple type indicates whether/how to flip the contents of a tile region when using it to fill a larger fill
/// region.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TileFlipMode {
    #[from_str = "none"]
    None,
    #[from_str = "x"]
    X,
    #[from_str = "y"]
    Y,
    #[from_str = "xy"]
    XY,
}

/// This simple type describes how to position two rectangles relative to each other.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum RectAlignment {
    #[from_str = "l"]
    Left,
    #[from_str = "t"]
    Top,
    #[from_str = "r"]
    Right,
    #[from_str = "b"]
    Bottom,
    #[from_str = "tl"]
    TopLeft,
    #[from_str = "tr"]
    TopRight,
    #[from_str = "bl"]
    BottomLeft,
    #[from_str = "br"]
    BottomRight,
    #[from_str = "ctr"]
    Center,
}

/// This simple type specifies the manner in which a path should be filled. The lightening and darkening of a path
/// allow for certain parts of the shape to be colored lighter of darker depending on user preference.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PathFillMode {
    /// This specifies that the corresponding path should have no fill.
    #[from_str = "none"]
    None,
    /// This specifies that the corresponding path should have a normally shaded color applied to it’s fill.
    #[from_str = "norm"]
    Norm,
    /// This specifies that the corresponding path should have a lightly shaded color applied to it’s fill.
    #[from_str = "lighten"]
    Lighten,
    /// This specifies that the corresponding path should have a slightly lighter shaded color applied to it’s fill.
    #[from_str = "lightenLess"]
    LightenLess,
    /// This specifies that the corresponding path should have a darker shaded color applied to it’s fill.
    #[from_str = "darken"]
    Darken,
    /// This specifies that the corresponding path should have a slightly darker shaded color applied to it’s fill.
    #[from_str = "darkenLess"]
    DarkenLess,
}

/// This simple type specifies the preset shape geometry that is to be used for a shape. An enumeration of this
/// simple type is used so that a custom geometry does not have to be specified but instead can be constructed
/// automatically by the generating application. For each enumeration listed there is also the corresponding
/// DrawingML code that would be used to construct this shape were it a custom geometry. Within the construction
/// code for each of these preset shapes there are predefined guides that the generating application shall maintain
/// for calculation purposes at all times. The necessary guides should have the following values:
///
/// * **3/4 of a Circle ('3cd4') - Constant value of "16200000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 270 degrees.
///
/// * **3/8 of a Circle ('3cd8') - Constant value of "8100000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 135 degrees.
///
/// * **5/8 of a Circle ('5cd8') - Constant value of "13500000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 225 degrees.
///
/// * **7/8 of a Circle ('7cd8') - Constant value of "18900000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 315 degrees.
///
/// * **Shape Bottom Edge ('b') - Constant value of "h"**
///
///     This is the bottom edge of the shape and since the top edge of the shape is considered the 0 point, the
///     bottom edge is thus the shape height.
///
/// * **1/2 of a Circle ('cd2') - Constant value of "10800000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 180 degrees.
///
/// * **1/4 of a Circle ('cd4') - Constant value of "5400000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 90 degrees.
///
/// * **1/8 of a Circle ('cd8') - Constant value of "2700000.0"**
///
///     The units here are in 60,000ths of a degree. This is equivalent to 45 degrees.
///
/// * **Shape Height ('h')**
///
///     This is the variable height of the shape defined in the shape properties. This value is received from the shape
///     transform listed within the <spPr> element.
///
/// * **Horizontal Center ('hc') - Calculated value of "\*/ w 1.0 2.0"**
///
///     This is the horizontal center of the shape which is just the width divided by 2.
///
/// * **1/2 of Shape Height ('hd2') - Calculated value of "\*/ h 1.0 2.0"**
///
///     This is 1/2 the shape height.
///
/// * **1/4 of Shape Height ('hd4') - Calculated value of "\*/ h 1.0 4.0"**
///
///     This is 1/4 the shape height.
///
/// * **1/5 of Shape Height ('hd5') - Calculated value of "\*/ h 1.0 5.0"**
///
///     This is 1/5 the shape height.
///
/// * **1/6 of Shape Height ('hd6') - Calculated value of "\*/ h 1.0 6.0"**
///
///     This is 1/6 the shape height.
///
/// * **1/8 of Shape Height ('hd8') - Calculated value of "\*/ h 1.0 8.0"**
///
///     This is 1/8 the shape height.
///
/// * **Shape Left Edge ('l') - Constant value of "0"**
///
///     This is the left edge of the shape and the left edge of the shape is considered the horizontal 0 point.
///
/// * **Longest Side of Shape ('ls') - Calculated value of "max w h"**
///
///     This is the longest side of the shape. This value is either the width or the height depending on which is greater.
///
/// * **Shape Right Edge ('r') - Constant value of "w"**
///
///     This is the right edge of the shape and since the left edge of the shape is considered the 0 point, the right edge
///     is thus the shape width.
///
/// * **Shortest Side of Shape ('ss') - Calculated value of "min w h"**
///
///     This is the shortest side of the shape. This value is either the width or the height depending on which is
///     smaller.
///
/// * **1/2 Shortest Side of Shape ('ssd2') - Calculated value of "\*/ ss 1.0 2.0"**
///
///     This is 1/2 the shortest side of the shape.
///
/// * **1/4 Shortest Side of Shape ('ssd4') - Calculated value of "\*/ ss 1.0 4.0"**
///
///     This is 1/4 the shortest side of the shape.
///
/// * **1/6 Shortest Side of Shape ('ssd6') - Calculated value of "\*/ ss 1.0 6.0"**
///
///     This is 1/6 the shortest side of the shape.
///
/// * **1/8 Shortest Side of Shape ('ssd8') - Calculated value of "\*/ ss 1.0 8.0"**
///
///     This is 1/8 the shortest side of the shape.
///
/// * **Shape Top Edge ('t') - Constant value of "0"**
///
///     This is the top edge of the shape and the top edge of the shape is considered the vertical 0 point.
///
/// * **Vertical Center of Shape ('vc') - Calculated value of "\*/ h 1.0 2.0"**
///
///     This is the vertical center of the shape which is just the height divided by 2.
///
/// * **Shape Width ('w')**
///
///     This is the variable width of the shape defined in the shape properties. This value is received from the shape
///     transform listed within the <spPr> element.
///
/// * **1/2 of Shape Width ('wd2') - Calculated value of "\*/ w 1.0 2.0"**
///
///     This is 1/2 the shape width.
///
/// * **1/4 of Shape Width ('wd4') - Calculated value of "\*/ w 1.0 4.0"**
///
///     This is 1/4 the shape width.
///
/// * **1/5 of Shape Width ('wd5') - Calculated value of "\*/ w 1.0 5.0"**
///
///     This is 1/5 the shape width.
///
/// * **1/6 of Shape Width ('wd6') - Calculated value of "\*/ w 1.0 6.0"**
///
///     This is 1/6 the shape width.
///
/// * **1/8 of Shape Width ('wd8') - Calculated value of "\*/ w 1.0 8.0"**
///
///     This is 1/8 the shape width.
///
/// * **1/10 of Shape Width ('wd10') - Calculated value of "\*/ w 1.0 10.0"**
///
///     This is 1/10 the shape width.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum ShapeType {
    #[from_str = "line"]
    Line,
    #[from_str = "lineInv"]
    LineInverse,
    #[from_str = "triangle"]
    Triangle,
    #[from_str = "rtTriangle"]
    RightTriangle,
    #[from_str = "rect"]
    Rect,
    #[from_str = "diamond"]
    Diamond,
    #[from_str = "parallelogram"]
    Parallelogram,
    #[from_str = "trapezoid"]
    Trapezoid,
    #[from_str = "nonIsoscelesTrapezoid"]
    NonIsoscelesTrapezoid,
    #[from_str = "pentagon"]
    Pentagon,
    #[from_str = "hexagon"]
    Hexagon,
    #[from_str = "heptagon"]
    Heptagon,
    #[from_str = "octagon"]
    Octagon,
    #[from_str = "decagon"]
    Decagon,
    #[from_str = "dodecagon"]
    Dodecagon,
    #[from_str = "star4"]
    Star4,
    #[from_str = "star5"]
    Star5,
    #[from_str = "star6"]
    Star6,
    #[from_str = "star7"]
    Star7,
    #[from_str = "star8"]
    Star8,
    #[from_str = "star10"]
    Star10,
    #[from_str = "star12"]
    Star12,
    #[from_str = "star16"]
    Star16,
    #[from_str = "star24"]
    Star24,
    #[from_str = "star32"]
    Star32,
    #[from_str = "roundRect"]
    RoundRect,
    #[from_str = "round1Rect"]
    Round1Rect,
    #[from_str = "round2SameRect"]
    Round2SameRect,
    #[from_str = "round2DiagRect"]
    Round2DiagRect,
    #[from_str = "snipRoundRect"]
    SnipRoundRect,
    #[from_str = "snip1Rect"]
    Snip1Rect,
    #[from_str = "snip2SameRect"]
    Snip2SameRect,
    #[from_str = "snip2DiagRect"]
    Snip2DiagRect,
    #[from_str = "plaque"]
    Plaque,
    #[from_str = "ellipse"]
    Ellipse,
    #[from_str = "teardrop"]
    Teardrop,
    #[from_str = "homePlate"]
    HomePlate,
    #[from_str = "chevron"]
    Chevron,
    #[from_str = "pieWedge"]
    PieWedge,
    #[from_str = "pie"]
    Pie,
    #[from_str = "blockArc"]
    BlockArc,
    #[from_str = "donut"]
    Donut,
    #[from_str = "noSmoking"]
    NoSmoking,
    #[from_str = "rightArrow"]
    RightArrow,
    #[from_str = "leftArrow"]
    LeftArrow,
    #[from_str = "upArrow"]
    UpArrow,
    #[from_str = "downArrow"]
    DownArrow,
    #[from_str = "stripedRightArrow"]
    StripedRightArrow,
    #[from_str = "notchedRightArrow"]
    NotchedRightArrow,
    #[from_str = "bentUpArrow"]
    BentUpArrow,
    #[from_str = "leftRightArrow"]
    LeftRightArrow,
    #[from_str = "upDownArrow"]
    UpDownArrow,
    #[from_str = "leftUpArrow"]
    LeftUpArrow,
    #[from_str = "leftRightUpArrow"]
    LeftRightUpArrow,
    #[from_str = "quadArrow"]
    QuadArrow,
    #[from_str = "leftArrowCallout"]
    LeftArrowCallout,
    #[from_str = "rightArrowCallout"]
    RightArrowCallout,
    #[from_str = "upArrowCallout"]
    UpArrowCallout,
    #[from_str = "downArrowCallout"]
    DownArrowCallout,
    #[from_str = "leftRightArrowCallout"]
    LeftRightArrowCallout,
    #[from_str = "upDownArrowCallout"]
    UpDownArrowCallout,
    #[from_str = "quadArrowCallout"]
    QuadArrowCallout,
    #[from_str = "bentArrow"]
    BentArrow,
    #[from_str = "uturnArrow"]
    UturnArrow,
    #[from_str = "circularArrow"]
    CircularArrow,
    #[from_str = "leftCircularArrow"]
    LeftCircularArrow,
    #[from_str = "leftRightCircularArrow"]
    LeftRightCircularArrow,
    #[from_str = "curvedRightArrow"]
    CurvedRightArrow,
    #[from_str = "curvedLeftArrow"]
    CurvedLeftArrow,
    #[from_str = "curvedUpArrow"]
    CurvedUpArrow,
    #[from_str = "curvedDownArrow"]
    CurvedDownArrow,
    #[from_str = "swooshArrow"]
    SwooshArrow,
    #[from_str = "cube"]
    Cube,
    #[from_str = "can"]
    Can,
    #[from_str = "lightningBolt"]
    LightningBolt,
    #[from_str = "heart"]
    Heart,
    #[from_str = "sun"]
    Sun,
    #[from_str = "moon"]
    Moon,
    #[from_str = "smileyFace"]
    SmileyFace,
    #[from_str = "irregularSeal1"]
    IrregularSeal1,
    #[from_str = "irregularSeal2"]
    IrregularSeal2,
    #[from_str = "foldedCorner"]
    FoldedCorner,
    #[from_str = "bevel"]
    Bevel,
    #[from_str = "frame"]
    Frame,
    #[from_str = "halfFrame"]
    HalfFrame,
    #[from_str = "corner"]
    Corner,
    #[from_str = "diagStripe"]
    DiagStripe,
    #[from_str = "chord"]
    Chord,
    #[from_str = "arc"]
    Arc,
    #[from_str = "leftBracket"]
    LeftBracket,
    #[from_str = "rightBracket"]
    RightBracket,
    #[from_str = "leftBrace"]
    LeftBrace,
    #[from_str = "rightBrace"]
    RightBrace,
    #[from_str = "bracketPair"]
    BracketPair,
    #[from_str = "bracePair"]
    BracePair,
    #[from_str = "straightConnector1"]
    StraightConnector1,
    #[from_str = "bentConnector2"]
    BentConnector2,
    #[from_str = "bentConnector3"]
    BentConnector3,
    #[from_str = "bentConnector4"]
    BentConnector4,
    #[from_str = "bentConnector5"]
    BentConnector5,
    #[from_str = "curvedConnector2"]
    CurvedConnector2,
    #[from_str = "curvedConnector3"]
    CurvedConnector3,
    #[from_str = "curvedConnector4"]
    CurvedConnector4,
    #[from_str = "curvedConnector5"]
    CurvedConnector5,
    #[from_str = "callout1"]
    Callout1,
    #[from_str = "callout2"]
    Callout2,
    #[from_str = "callout3"]
    Callout3,
    #[from_str = "accentCallout1"]
    AccentCallout1,
    #[from_str = "accentCallout2"]
    AccentCallout2,
    #[from_str = "accentCallout3"]
    AccentCallout3,
    #[from_str = "borderCallout1"]
    BorderCallout1,
    #[from_str = "borderCallout2"]
    BorderCallout2,
    #[from_str = "borderCallout3"]
    BorderCallout3,
    #[from_str = "accentBorderCallout1"]
    AccentBorderCallout1,
    #[from_str = "accentBorderCallout2"]
    AccentBorderCallout2,
    #[from_str = "accentBorderCallout3"]
    AccentBorderCallout3,
    #[from_str = "wedgeRectCallout"]
    WedgeRectCallout,
    #[from_str = "wedgeRoundRectCallout"]
    WedgeRoundRectCallout,
    #[from_str = "wedgeEllipseCallout"]
    WedgeEllipseCallout,
    #[from_str = "cloudCallout"]
    CloudCallout,
    #[from_str = "cloud"]
    Cloud,
    #[from_str = "ribbon"]
    Ribbon,
    #[from_str = "ribbon2"]
    Ribbon2,
    #[from_str = "ellipseRibbon"]
    EllipseRibbon,
    #[from_str = "ellipseRibbon2"]
    EllipseRibbon2,
    #[from_str = "leftRightRibbon"]
    LeftRightRibbon,
    #[from_str = "verticalScroll"]
    VerticalScroll,
    #[from_str = "horizontalScroll"]
    HorizontalScroll,
    #[from_str = "wave"]
    Wave,
    #[from_str = "doubleWave"]
    DoubleWave,
    #[from_str = "plus"]
    Plus,
    #[from_str = "flowChartProcess"]
    FlowChartProcess,
    #[from_str = "flowChartDecision"]
    FlowChartDecision,
    #[from_str = "flowChartInputOutput"]
    FlowChartInputOutput,
    #[from_str = "flowChartPredefinedProcess"]
    FlowChartPredefinedProcess,
    #[from_str = "flowChartInternalStorage"]
    FlowChartInternalStorage,
    #[from_str = "flowChartDocument"]
    FlowChartDocument,
    #[from_str = "flowChartMultidocument"]
    FlowChartMultidocument,
    #[from_str = "flowChartTerminator"]
    FlowChartTerminator,
    #[from_str = "flowChartPreparation"]
    FlowChartPreparation,
    #[from_str = "flowChartManualInput"]
    FlowChartManualInput,
    #[from_str = "flowChartOperation"]
    FlowChartManualOperation,
    #[from_str = "flowChartConnector"]
    FlowChartConnector,
    #[from_str = "flowChartPunchedCard"]
    FlowChartPunchedCard,
    #[from_str = "flowChartPunchedTape"]
    FlowChartPunchedTape,
    #[from_str = "flowChartSummingJunction"]
    FlowChartSummingJunction,
    #[from_str = "flowChartOr"]
    FlowChartOr,
    #[from_str = "flowChartCollate"]
    FlowChartCollate,
    #[from_str = "flowChartSort"]
    FlowChartSort,
    #[from_str = "flowChartExtract"]
    FlowChartExtract,
    #[from_str = "flowChartMerge"]
    FlowChartMerge,
    #[from_str = "flowChartOfflineStorage"]
    FlowChartOfflineStorage,
    #[from_str = "flowChartOnlineStorage"]
    FlowChartOnlineStorage,
    #[from_str = "flowChartMagneticTape"]
    FlowChartMagneticTape,
    #[from_str = "flowChartMagneticDisk"]
    FlowChartMagneticDisk,
    #[from_str = "flowChartMagneticDrum"]
    FlowChartMagneticDrum,
    #[from_str = "flowChartDisplay"]
    FlowChartDisplay,
    #[from_str = "flowChartDelay"]
    FlowChartDelay,
    #[from_str = "flowChartAlternateProcess"]
    FlowChartAlternateProcess,
    #[from_str = "flowChartOffpageConnector"]
    FlowChartOffpageConnector,
    #[from_str = "actionButtonBlank"]
    ActionButtonBlank,
    #[from_str = "actionButtonHome"]
    ActionButtonHome,
    #[from_str = "actionButtonHelp"]
    ActionButtonHelp,
    #[from_str = "actionButtonInformation"]
    ActionButtonInformation,
    #[from_str = "actionButtonForwardNext"]
    ActionButtonForwardNext,
    #[from_str = "actionButtonBackPrevious"]
    ActionButtonBackPrevious,
    #[from_str = "actionButtonEnd"]
    ActionButtonEnd,
    #[from_str = "actionButtonBeginning"]
    ActionButtonBeginning,
    #[from_str = "actionButtonReturn"]
    ActionButtonReturn,
    #[from_str = "actionButtonDocument"]
    ActionButtonDocument,
    #[from_str = "actionButtonSound"]
    ActionButtonSound,
    #[from_str = "actionButtonMovie"]
    ActionButtonMovie,
    #[from_str = "gear6"]
    Gear6,
    #[from_str = "gear9"]
    Gear9,
    #[from_str = "funnel"]
    Funnel,
    #[from_str = "mathPlus"]
    MathPlus,
    #[from_str = "mathMinus"]
    MathMinus,
    #[from_str = "mathMultiply"]
    MathMultiply,
    #[from_str = "mathDivide"]
    MathDivide,
    #[from_str = "mathEqual"]
    MathEqual,
    #[from_str = "mathNotEqual"]
    MathNotEqual,
    #[from_str = "cornerTabs"]
    CornerTabs,
    #[from_str = "squareTabs"]
    SquareTabs,
    #[from_str = "plaqueTabs"]
    PlaqueTabs,
    #[from_str = "chartX"]
    ChartX,
    #[from_str = "chartStar"]
    ChartStar,
    #[from_str = "chartPlus"]
    ChartPlus,
}

/// This simple type specifies how to cap the ends of lines. This also affects the ends of line segments for dashed
/// lines.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineCap {
    /// Rounded ends. Semi-circle protrudes by half line width.
    #[from_str = "rnd"]
    Round,
    /// Square protrudes by half line width.
    #[from_str = "sq"]
    Square,
    /// Line ends at end point.
    #[from_str = "flat"]
    Flat,
}

/// This simple type specifies the compound line type that is to be used for lines with text such as underlines.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum CompoundLine {
    /// Single line: one normal width
    #[from_str = "sng"]
    Single,
    /// Double lines of equal width
    #[from_str = "dbl"]
    Double,
    /// Double lines: one thick, one thin
    #[from_str = "thickThin"]
    ThickThin,
    /// Double lines: one thin, one thick
    #[from_str = "thinThick"]
    ThinThick,
    /// Three lines: thin, thick, thin
    #[from_str = "tri"]
    Triple,
}

/// This simple type specifies the Pen Alignment type for use within a text body.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PenAlignment {
    /// Center pen (line drawn at center of path stroke).
    #[from_str = "ctr"]
    Center,
    /// Inset pen (the pen is aligned on the inside of the edge of the path).
    #[from_str = "in"]
    Inset,
}

/// This simple type represents preset line dash values. The description for each style shows an illustration of the
/// line style. Each style also contains a precise binary representation of the repeating dash style. Each 1
/// corresponds to a line segment of the same length as the line width, and each 0 corresponds to a space of the
/// same length as the line width.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetLineDashVal {
    /// 1
    #[from_str = "solid"]
    Solid,
    /// 1000
    #[from_str = "dot"]
    Dot,
    /// 1111000
    #[from_str = "dash"]
    Dash,
    /// 11111111000
    #[from_str = "lgDash"]
    LargeDash,
    /// 11110001000
    #[from_str = "dashDot"]
    DashDot,
    /// 111111110001000
    #[from_str = "lgDashDot"]
    LargeDashDot,
    /// 1111111100010001000
    #[from_str = "ldDashDotDot"]
    LargeDashDotDot,
    /// 1110
    #[from_str = "sysDash"]
    SystemDash,
    /// 10
    #[from_str = "sysDot"]
    SystemDot,
    /// 111010
    #[from_str = "sysDashDot"]
    SystemDashDot,
    /// 11101010
    #[from_str = "sysDashDotDot"]
    SystemDashDotDot,
}

/// This simple type represents the shape decoration that appears at the ends of lines. For example, one choice is an
/// arrow head.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineEndType {
    #[from_str = "none"]
    None,
    #[from_str = "triangle"]
    Triangle,
    #[from_str = "stealth"]
    Stealth,
    #[from_str = "diamond"]
    Diamond,
    #[from_str = "oval"]
    Oval,
    #[from_str = "arrow"]
    Arrow,
}

/// This simple type represents the width of the line end decoration (e.g., arrowhead) relative to the width of the
/// line itself.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineEndWidth {
    #[from_str = "sm"]
    Small,
    #[from_str = "med"]
    Medium,
    #[from_str = "lg"]
    Large,
}

/// This simple type represents the length of the line end decoration (e.g., arrowhead) relative to the width of the
/// line itself.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineEndLength {
    #[from_str = "sm"]
    Small,
    #[from_str = "med"]
    Medium,
    #[from_str = "lg"]
    Large,
}

/// This simple type indicates one of 20 preset shadow types. Each enumeration value description illustrates the
/// type of shadow represented by the value. Each description contains the parameters to the outer shadow effect
/// represented by the preset, in addition to those attributes common to all prstShdw effects.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetShadowVal {
    /// No additional attributes specified.
    #[from_str = "shdw1"]
    TopLeftDropShadow,
    /// No additional attributes specified.
    #[from_str = "shdw2"]
    TopRightDropShadow,
    /// align = "b"
    /// ky = 40.89°
    /// sy = 50%
    #[from_str = "shdw3"]
    BackLeftPerspectiveShadow,
    /// align = "b"
    /// kx = -40.89°
    /// sy = 50%
    #[from_str = "shdw4"]
    BackRightPerspectiveShadow,
    /// No additional attributes specified.
    #[from_str = "shdw5"]
    BottomLeftDropShadow,
    /// No additional attributes specified.
    #[from_str = "shdw6"]
    BottomRightDropShadow,
    /// align = "b"
    /// kx = 40.89°
    /// sy = -50%
    #[from_str = "shdw7"]
    FrontLeftPerspectiveShadow,
    /// align = "b"
    /// kx = -40.89°
    /// sy = -50%
    #[from_str = "shdw8"]
    FrontRightPerspectiveShadow,
    /// align = "tl"
    /// sx = 75%
    /// sy = 75%
    #[from_str = "shdw9"]
    TopLeftSmallDropShadow,
    /// align = "br"
    /// sx = 125%
    /// sy = 125%
    #[from_str = "shdw10"]
    TopLeftLargeDropShadow,
    /// align = "b"
    /// kx = 40.89°
    /// sy = 50%
    #[from_str = "shdw11"]
    BackLeftLongPerspectiveShadow,
    /// align = "b"
    /// kx = -40.89°
    /// sy = 50%
    #[from_str = "shdw12"]
    BackRightLongPerspectiveShadow,
    /// Equivalent to two outer shadow effects.
    ///
    /// Shadow 1:
    /// No additional attributes specified.
    ///
    /// Shadow 2:
    /// color = min(1, shadow 1's color (0 <= r, g, b <= 1) +
    /// 102/255), per r, g, b component
    /// dist = 2 * shadow 1's distance
    #[from_str = "shdw13"]
    TopLeftDoubleDropShadow,
    /// No additional attributes specified.
    #[from_str = "shdw14"]
    BottomRightSmallDropShadow,
    /// align = "b"
    /// kx = 40.89°
    /// sy = -50%
    #[from_str = "shdw15"]
    FrontLeftLongPerspectiveShadow,
    /// align = "b"
    /// kx = -40.89°
    /// sy = -50%
    #[from_str = "shdw16"]
    FrontRightLongPerspectiveShadow,
    /// Equivalent to two outer shadow effects.
    ///
    /// Shadow 1:
    /// No additional attributes specified.
    ///
    /// Shadow 2:
    /// color = min(1, shadow 1's color (0 <= r, g, b <= 1) +
    /// 102/255), per r, g, b component
    /// dir = shadow 1's direction + 180°
    #[from_str = "shdw17"]
    ThreeDOuterBoxShadow,
    /// Equivalent to two outer shadow effects.
    ///
    /// Shadow 1:
    /// No additional attributes specified.
    ///
    /// Shadow 2:
    /// color = min(1, shadow 1's color (0 <= r, g, b <= 1) +
    /// 102/255), per r, g, b component
    /// dir = shadow 1's direction + 180°
    #[from_str = "shdw18"]
    ThreeDInnerBoxShadow,
    /// align = "b"
    /// sy = 50°
    #[from_str = "shdw19"]
    BackCenterPerspectiveShadow,
    /// align = "b"
    /// sy = -100°
    #[from_str = "shdw20"]
    FrontBottomShadow,
}

/// This simple type determines the relationship between effects in a container, either sibling or tree.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum EffectContainerType {
    /// Each effect is separately applied to the parent object.
    ///
    /// # Example
    ///
    /// If the parent element contains an outer shadow and a reflection, the resulting effect is a
    /// shadow around the parent object and a reflection of the object. The reflection does not have a shadow.
    #[from_str = "sib"]
    Sib,
    /// Each effect is applied to the result of the previous effect.
    ///
    /// # Example
    ///
    /// If the parent element contains an outer shadow followed by a glow, the shadow is first applied
    /// to the parent object. Then, the glow is applied to the shadow (rather than the original object). The resulting
    /// effect would be a glowing shadow.
    #[from_str = "tree"]
    Tree,
}

/// This simple type represents one of the fonts associated with the style.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum FontCollectionIndex {
    /// The major font of the style's font scheme.
    #[from_str = "major"]
    Major,
    /// The minor font of the style's font scheme.
    #[from_str = "minor"]
    Minor,
    /// No font reference.
    #[from_str = "none"]
    None,
}

/// This simple type specifies an animation build step within a diagram animation.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum DgmBuildStep {
    /// Animate a diagram shape for this animation build step
    #[from_str = "sp"]
    Shape,
    /// Animate the diagram background for this animation build step
    #[from_str = "bg"]
    Background,
}

/// This simple type specifies an animation build step within a chart animation.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum ChartBuildStep {
    /// Animate a chart category for this animation build step
    #[from_str = "category"]
    Category,
    /// Animate a point in a chart category for this animation build step
    #[from_str = "ptInCategory"]
    PtInCategory,
    /// Animate a chart series for this animation build step
    #[from_str = "series"]
    Series,
    /// Animate a point in a chart series for this animation build step
    #[from_str = "ptInSeries"]
    PtInSeries,
    /// Animate all points within the chart for this animation build step
    #[from_str = "allPts"]
    AllPts,
    /// Animate the chart grid and legend for this animation build step
    #[from_str = "gridLegend"]
    GridLegend,
}

/// This simple type represents whether a style property should be applied.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum OnOffStyleType {
    /// Property is on.
    #[from_str = "on"]
    On,
    /// Property is off.
    #[from_str = "off"]
    Off,
    /// Follow parent settings. For a themed property, follow the theme settings. For an unthemed property, follow
    /// the parent setting in the property inheritance chain.
    #[from_str = "def"]
    Default,
}

/// This simple type specifies a system color value. This color is based upon the value that this color currently has
/// within the system on which the document is being viewed.
///
/// Applications shall use the lastClr attribute to determine the absolute value of the last color used if system colors
/// are not supported.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum SystemColorVal {
    /// Specifies the scroll bar gray area color.
    #[from_str = "scrollBar"]
    ScrollBar,
    ///Specifies the desktop background color.
    #[from_str = "background"]
    Background,
    /// Specifies the active window title bar color. In particular the left side color in the color gradient of an
    /// active window's title bar if the gradient effect is enabled.
    #[from_str = "activeCaption"]
    ActiveCaption,
    /// Specifies the color of the Inactive window caption.
    /// Specifies the left side color in the color gradient of an inactive window's title bar if the gradient effect is
    /// enabled.
    #[from_str = "inactiveCaption"]
    InactiveCaption,
    /// Specifies the menu background color.
    #[from_str = "menu"]
    Menu,
    /// Specifies window background color.
    #[from_str = "window"]
    Window,
    /// Specifies the window frame color.
    #[from_str = "windowFrame"]
    WindowFrame,
    /// Specifies the color of Text in menus.
    #[from_str = "menuText"]
    MenuText,
    /// Specifies the color of text in windows.
    #[from_str = "windowText"]
    WindowText,
    /// Specifies the color of text in the caption, size box, and scroll bar arrow box.
    #[from_str = "captionText"]
    CaptionText,
    /// Specifies an Active Window Border Color.
    #[from_str = "activeBorder"]
    ActiveBorder,
    /// Specifies the color of the Inactive window border.
    #[from_str = "inactiveBorder"]
    InactiveBorder,
    /// Specifies the Background color of multiple document interface (MDI) applications
    #[from_str = "appWorkspace"]
    AppWorkspace,
    /// Specifies the color of Item(s) selected in a control.
    #[from_str = "highlight"]
    Highlight,
    /// Specifies the text color of item(s) selected in a control.
    #[from_str = "highlightText"]
    HighlightText,
    /// Specifies the face color for three-dimensional display elements and for dialog box backgrounds.
    #[from_str = "btnFace"]
    ButtonFace,
    /// Specifies the shadow color for three-dimensional display elements (for edges facing away from the light source).
    #[from_str = "btnShadow"]
    ButtonShadow,
    /// Specifies a grayed (disabled) text. This color is set to 0 if the current display driver does not support a
    /// solid gray color.
    #[from_str = "grayText"]
    GrayText,
    /// Specifies the color of text on push buttons.
    #[from_str = "btnText"]
    ButtonText,
    /// Specifies the color of text in an inactive caption.
    #[from_str = "inactiveCaptionText"]
    InactiveCaptionText,
    /// Specifies the highlight color for three-dimensional display elements (for edges facing the light source).
    #[from_str = "btnHighlight"]
    ButtonHighlight,
    /// Specifies a Dark shadow color for three-dimensional display elements.
    #[from_str = "3dDkShadow"]
    DarkShadow3d,
    /// Specifies a Light color for three-dimensional display elements (for edges facing the light source).
    #[from_str = "3dLight"]
    Light3d,
    /// Specifies the text color for tooltip controls.
    #[from_str = "infoText"]
    InfoText,
    /// Specifies the background color for tooltip controls.
    #[from_str = "infoBk"]
    InfoBack,
    #[from_str = "hotLight"]
    /// Specifies the color for a hyperlink or hot-tracked item.
    HotLight,
    #[from_str = "gradientActiveCaption"]
    /// Specifies the right side color in the color gradient of an active window's title bar.
    GradientActiveCaption,
    /// Specifies the right side color in the color gradient of an inactive window's title bar.
    #[from_str = "gradientInactiveCaption"]
    GradientInactiveCaption,
    /// Specifies the color used to highlight menu items when the menu appears as a flat menu.
    #[from_str = "menuHighlight"]
    MenuHighlight,
    /// Specifies the background color for the menu bar when menus appear as flat menus.
    #[from_str = "menubar"]
    MenuBar,
}

/// This simple type represents a preset color value.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetColorVal {
    /// Specifies a color with RGB value (240,248,255)
    #[from_str = "aliceBlue"]
    AliceBlue,
    /// Specifies a color with RGB value (250,235,215)
    #[from_str = "antiqueWhite"]
    AntiqueWhite,
    /// Specifies a color with RGB value (0,255,255)
    #[from_str = "aqua"]
    Aqua,
    /// Specifies a color with RGB value (127,255,212)
    #[from_str = "aquamarine"]
    Aquamarine,
    /// Specifies a color with RGB value (240,255,255)
    #[from_str = "azure"]
    Azure,
    ///Specifies a color with RGB value (245,245,220)
    #[from_str = "beige"]
    Beige,
    /// Specifies a color with RGB value (255,228,196)
    #[from_str = "bisque"]
    Bisque,
    /// Specifies a color with RGB value (0,0,0)
    #[from_str = "black"]
    Black,
    /// Specifies a color with RGB value (255,235,205)
    #[from_str = "blanchedAlmond"]
    BlanchedAlmond,
    /// Specifies a color with RGB value (0,0,255)
    #[from_str = "blue"]
    Blue,
    /// Specifies a color with RGB value (138,43,226)
    #[from_str = "blueViolet"]
    BlueViolet,
    /// Specifies a color with RGB value (165,42,42)
    #[from_str = "brown"]
    Brown,
    /// Specifies a color with RGB value (222,184,135)
    #[from_str = "burlyWood"]
    BurlyWood,
    /// Specifies a color with RGB value (95,158,160)
    #[from_str = "cadetBlue"]
    CadetBlue,
    /// Specifies a color with RGB value (127,255,0)
    #[from_str = "chartreuse"]
    Chartreuse,
    /// Specifies a color with RGB value (210,105,30)
    #[from_str = "chocolate"]
    Chocolate,
    /// Specifies a color with RGB value (255,127,80)
    #[from_str = "coral"]
    Coral,
    /// Specifies a color with RGB value (100,149,237)
    #[from_str = "cornflowerBlue"]
    CornflowerBlue,
    /// Specifies a color with RGB value (255,248,220)
    #[from_str = "cornsilk"]
    Cornsilk,
    /// Specifies a color with RGB value (220,20,60)
    #[from_str = "crimson"]
    Crimson,
    /// Specifies a color with RGB value (0,255,255)
    #[from_str = "cyan"]
    Cyan,
    /// Specifies a color with RGB value (0,0,139)
    #[from_str = "darkBlue"]
    DarkBlue,
    /// Specifies a color with RGB value (0,139,139)
    #[from_str = "darkCyan"]
    DarkCyan,
    /// Specifies a color with RGB value (184,134,11)
    #[from_str = "darkGoldenrod"]
    DarkGoldenrod,
    /// Specifies a color with RGB value (169,169,169)
    #[from_str = "darkGray"]
    DarkGray,
    /// Specifies a color with RGB value (169,169,169)
    #[from_str = "darkGrey"]
    DarkGrey,
    /// Specifies a color with RGB value (0,100,0)
    #[from_str = "darkGreen"]
    DarkGreen,
    /// Specifies a color with RGB value (189,183,107)
    #[from_str = "darkKhaki"]
    DarkKhaki,
    /// Specifies a color with RGB value (139,0,139)
    #[from_str = "darkMagenta"]
    DarkMagenta,
    /// Specifies a color with RGB value (85,107,47)
    #[from_str = "darkOliveGreen"]
    DarkOliveGreen,
    /// Specifies a color with RGB value (255,140,0)
    #[from_str = "darkOrange"]
    DarkOrange,
    /// Specifies a color with RGB value (153,50,204)
    #[from_str = "darkOrchid"]
    DarkOrchid,
    /// Specifies a color with RGB value (139,0,0)
    #[from_str = "darkRed"]
    DarkRed,
    /// Specifies a color with RGB value (233,150,122)
    #[from_str = "darkSalmon"]
    DarkSalmon,
    /// Specifies a color with RGB value (143,188,143)
    #[from_str = "darkSeaGreen"]
    DarkSeaGreen,
    /// Specifies a color with RGB value (72,61,139)
    #[from_str = "darkSlateBlue"]
    DarkSlateBlue,
    /// Specifies a color with RGB value (47,79,79)
    #[from_str = "darkSlateGray"]
    DarkSlateGray,
    /// Specifies a color with RGB value (47,79,79)
    #[from_str = "darkSlateGrey"]
    DarkSlateGrey,
    /// Specifies a color with RGB value (0,206,209)
    #[from_str = "darkTurquoise"]
    DarkTurqoise,
    /// Specifies a color with RGB value (148,0,211)
    #[from_str = "darkViolet"]
    DarkViolet,
    /// Specifies a color with RGB value (0,0,139)
    #[from_str = "dkBlue"]
    DkBlue,
    /// Specifies a color with RGB value (0,139,139)
    #[from_str = "dkCyan"]
    DkCyan,
    /// Specifies a color with RGB value (184,134,11)
    #[from_str = "dkGoldenrod"]
    DkGoldenrod,
    /// Specifies a color with RGB value (169,169,169)
    #[from_str = "dkGray"]
    DkGray,
    /// Specifies a color with RGB value (169,169,169)
    #[from_str = "dkGrey"]
    DkGrey,
    /// Specifies a color with RGB value (0,100,0)
    #[from_str = "dkGreen"]
    DkGreen,
    /// Specifies a color with RGB value (189,183,107)
    #[from_str = "dkKhaki"]
    DkKhaki,
    /// Specifies a color with RGB value (139,0,139)
    #[from_str = "dkMagenta"]
    DkMagenta,
    /// Specifies a color with RGB value (85,107,47)
    #[from_str = "dkOliveGreen"]
    DkOliveGreen,
    /// Specifies a color with RGB value (255,140,0)
    #[from_str = "dkOrange"]
    DkOrange,
    /// Specifies a color with RGB value (153,50,204)
    #[from_str = "dkOrchid"]
    DkOrchid,
    /// Specifies a color with RGB value (139,0,0)
    #[from_str = "dkRed"]
    DkRed,
    /// Specifies a color with RGB value (233,150,122)
    #[from_str = "dkSalmon"]
    DkSalmon,
    /// Specifies a color with RGB value (143,188,139)
    #[from_str = "dkSeaGreen"]
    DkSeaGreen,
    /// Specifies a color with RGB value (72,61,139)
    #[from_str = "dkSlateBlue"]
    DkSlateBlue,
    /// Specifies a color with RGB value (47,79,79)
    #[from_str = "dkSlateGray"]
    DkSlateGray,
    /// Specifies a color with RGB value (47,79,79)
    #[from_str = "dkSlateGrey"]
    DkSlateGrey,
    /// Specifies a color with RGB value (0,206,209)
    #[from_str = "dkTurquoise"]
    DkTurquoise,
    /// Specifies a color with RGB value (148,0,211)
    #[from_str = "dkViolet"]
    DkViolet,
    /// Specifies a color with RGB value (255,20,147)
    #[from_str = "deepPink"]
    DeepPink,
    /// Specifies a color with RGB value (0,191,255)
    #[from_str = "deepSkyBlue"]
    DeepSkyBlue,
    /// Specifies a color with RGB value (105,105,105)
    #[from_str = "dimGray"]
    DimGray,
    /// Specifies a color with RGB value (105,105,105)
    #[from_str = "dimGrey"]
    DimGrey,
    /// Specifies a color with RGB value (30,144,255)
    #[from_str = "dodgerBlue"]
    DodgerBluet,
    /// Specifies a color with RGB value (178,34,34)
    #[from_str = "firebrick"]
    Firebrick,
    /// Specifies a color with RGB value (255,250,240)
    #[from_str = "floralWhite"]
    FloralWhite,
    /// Specifies a color with RGB value (34,139,34)
    #[from_str = "forestGreen"]
    ForestGreen,
    /// Specifies a color with RGB value (255,0,255)
    #[from_str = "fuchsia"]
    Fuchsia,
    /// Specifies a color with RGB value (220,220,220)
    #[from_str = "gainsboro"]
    Gainsboro,
    /// Specifies a color with RGB value (248,248,255)
    #[from_str = "ghostWhite"]
    GhostWhite,
    /// Specifies a color with RGB value (255,215,0)
    #[from_str = "gold"]
    Gold,
    /// Specifies a color with RGB value (218,165,32)
    #[from_str = "goldenrod"]
    Goldenrod,
    /// Specifies a color with RGB value (128,128,128)
    #[from_str = "gray"]
    Gray,
    /// Specifies a color with RGB value (128,128,128)
    #[from_str = "grey"]
    Grey,
    /// Specifies a color with RGB value (0,128,0)
    #[from_str = "green"]
    Green,
    /// Specifies a color with RGB value (173,255,47)
    #[from_str = "greenYellow"]
    GreenYellow,
    /// Specifies a color with RGB value (240,255,240)
    #[from_str = "honeydew"]
    Honeydew,
    /// Specifies a color with RGB value (255,105,180)
    #[from_str = "hotPink"]
    HotPink,
    /// Specifies a color with RGB value (205,92,92)
    #[from_str = "indianRed"]
    IndianRed,
    /// Specifies a color with RGB value (75,0,130)
    #[from_str = "indigo"]
    Indigo,
    /// Specifies a color with RGB value (255,255,240)
    #[from_str = "ivory"]
    Ivory,
    /// Specifies a color with RGB value (240,230,140)
    #[from_str = "khaki"]
    Khaki,
    /// Specifies a color with RGB value (230,230,250)
    #[from_str = "lavender"]
    Lavender,
    /// Specifies a color with RGB value (255,240,245)
    #[from_str = "lavenderBlush"]
    LavenderBlush,
    /// Specifies a color with RGB value (124,252,0)
    #[from_str = "lawnGreen"]
    LawnGreen,
    /// Specifies a color with RGB value (255,250,205)
    #[from_str = "lemonChiffon"]
    LemonChiffon,
    /// Specifies a color with RGB value (173,216,230)
    #[from_str = "lightBlue"]
    LightBlue,
    /// Specifies a color with RGB value (240,128,128)
    #[from_str = "lightCoral"]
    LightCoral,
    /// Specifies a color with RGB value (224,255,255)
    #[from_str = "lightCyan"]
    LightCyan,
    /// Specifies a color with RGB value (250,250,210)
    #[from_str = "lightGoldenrodYellow"]
    LightGoldenrodYellow,
    /// Specifies a color with RGB value (211,211,211)
    #[from_str = "lightGray"]
    LightGray,
    /// Specifies a color with RGB value (211,211,211)
    #[from_str = "lightGrey"]
    LightGrey,
    /// Specifies a color with RGB value (144,238,144)
    #[from_str = "lightGreen"]
    LightGreen,
    /// Specifies a color with RGB value (255,182,193)
    #[from_str = "lightPink"]
    LightPink,
    /// Specifies a color with RGB value (255,160,122)
    #[from_str = "lightSalmon"]
    LightSalmon,
    /// Specifies a color with RGB value (32,178,170)
    #[from_str = "lightSeaGreen"]
    LightSeaGreen,
    /// Specifies a color with RGB value (135,206,250)
    #[from_str = "lightSkyBlue"]
    LightSkyBlue,
    /// Specifies a color with RGB value (119,136,153)
    #[from_str = "lightSlateGray"]
    LightSlateGray,
    /// Specifies a color with RGB value (119,136,153)
    #[from_str = "lightSlateGrey"]
    LightSlateGrey,
    /// Specifies a color with RGB value (176,196,222)
    #[from_str = "lightSteelBlue"]
    LightSteelBlue,
    /// Specifies a color with RGB value (255,255,224)
    #[from_str = "lightYellow"]
    LightYellow,
    /// Specifies a color with RGB value (173,216,230)
    #[from_str = "ltBlue"]
    LtBlue,
    /// Specifies a color with RGB value (240,128,128)
    #[from_str = "ltCoral"]
    LtCoral,
    /// Specifies a color with RGB value (224,255,255)
    #[from_str = "ltCyan"]
    LtCyan,
    /// Specifies a color with RGB value (250,250,120)
    #[from_str = "ltGoldenrodYellow"]
    LtGoldenrodYellow,
    /// Specifies a color with RGB value (211,211,211)
    #[from_str = "ltGray"]
    LtGray,
    /// Specifies a color with RGB value (211,211,211)
    #[from_str = "ltGrey"]
    LtGrey,
    /// Specifies a color with RGB value (144,238,144)
    #[from_str = "ltGreen"]
    LtGreen,
    /// Specifies a color with RGB value (255,182,193)
    #[from_str = "ltPink"]
    LtPink,
    /// Specifies a color with RGB value (255,160,122)
    #[from_str = "ltSalmon"]
    LtSalmon,
    /// Specifies a color with RGB value (32,178,170)
    #[from_str = "ltSeaGreen"]
    LtSeaGreen,
    /// Specifies a color with RGB value (135,206,250)
    #[from_str = "ltSkyBlue"]
    LtSkyBlue,
    /// Specifies a color with RGB value (119,136,153)
    #[from_str = "ltSlateGray"]
    LtSlateGray,
    /// Specifies a color with RGB value (119,136,153)
    #[from_str = "ltSlateGrey"]
    LtSlateGrey,
    /// Specifies a color with RGB value (176,196,222)
    #[from_str = "ltSteelBlue"]
    LtSteelBlue,
    /// Specifies a color with RGB value (255,255,224)
    #[from_str = "ltYellow"]
    LtYellow,
    /// Specifies a color with RGB value (0,255,0)
    #[from_str = "lime"]
    Lime,
    /// Specifies a color with RGB value (50,205,50)
    #[from_str = "limeGreen"]
    LimeGreen,
    /// Specifies a color with RGB value (250,240,230)
    #[from_str = "linen"]
    Linen,
    /// Specifies a color with RGB value (255,0,255)
    #[from_str = "magenta"]
    Magenta,
    /// Specifies a color with RGB value (128,0,0)
    #[from_str = "maroon"]
    Maroon,
    /// Specifies a color with RGB value (102,205,170)
    #[from_str = "medAquamarine"]
    MedAquamarine,
    /// Specifies a color with RGB value (0,0,205)
    #[from_str = "medBlue"]
    MedBlue,
    /// Specifies a color with RGB value (186,85,211)
    #[from_str = "medOrchid"]
    MedOrchid,
    /// Specifies a color with RGB value (147,112,219)
    #[from_str = "medPurple"]
    MedPurple,
    /// Specifies a color with RGB value (60,179,113)
    #[from_str = "medSeaGreen"]
    MedSeaGreen,
    /// Specifies a color with RGB value (123,104,238)
    #[from_str = "medSlateBlue"]
    MedSlateBlue,
    /// Specifies a color with RGB value (0,250,154)
    #[from_str = "medSpringGreen"]
    MedSpringGreen,
    /// Specifies a color with RGB value (72,209,204)
    #[from_str = "medTurquoise"]
    MedTurquoise,
    /// Specifies a color with RGB value (199,21,133)
    #[from_str = "medVioletRed"]
    MedVioletRed,
    /// Specifies a color with RGB value (102,205,170)
    #[from_str = "mediumAquamarine"]
    MediumAquamarine,
    /// Specifies a color with RGB value (0,0,205)
    #[from_str = "mediumBlue"]
    MediumBlue,
    /// Specifies a color with RGB value (186,85,211)
    #[from_str = "mediumOrchid"]
    MediumOrchid,
    /// Specifies a color with RGB value (147,112,219)
    #[from_str = "mediumPurple"]
    MediumPurple,
    /// Specifies a color with RGB value (60,179,113)
    #[from_str = "mediumSeaGreen"]
    MediumSeaGreen,
    /// Specifies a color with RGB value (123,104,238)
    #[from_str = "mediumSlateBlue"]
    MediumSlateBlue,
    /// Specifies a color with RGB value (0,250,154)
    #[from_str = "mediumSpringGreen"]
    MediumSpringGreen,
    /// Specifies a color with RGB value (72,209,204)
    #[from_str = "mediumTurquoise"]
    MediumTurquoise,
    /// Specifies a color with RGB value (199,21,133)
    #[from_str = "mediumVioletRed"]
    MediumVioletRed,
    /// Specifies a color with RGB value (25,25,112)
    #[from_str = "midnightBlue"]
    MidnightBlue,
    /// Specifies a color with RGB value (245,255,250)
    #[from_str = "mintCream"]
    MintCream,
    /// Specifies a color with RGB value (255,228,225)
    #[from_str = "mistyRose"]
    MistyRose,
    /// Specifies a color with RGB value (255,228,181)
    #[from_str = "moccasin"]
    Moccasin,
    /// Specifies a color with RGB value (255,222,173)
    #[from_str = "navajoWhite"]
    NavajoWhite,
    /// Specifies a color with RGB value (0,0,128)
    #[from_str = "navy"]
    Navy,
    /// Specifies a color with RGB value (253,245,230)
    #[from_str = "oldLace"]
    OldLace,
    /// Specifies a color with RGB value (128,128,0)
    #[from_str = "olive"]
    Olive,
    /// Specifies a color with RGB value (107,142,35)
    #[from_str = "oliveDrab"]
    OliveDrab,
    /// Specifies a color with RGB value (255,165,0)
    #[from_str = "orange"]
    Orange,
    /// Specifies a color with RGB value (255,69,0)
    #[from_str = "orangeRed"]
    OrangeRed,
    /// Specifies a color with RGB value (218,112,214)
    #[from_str = "orchid"]
    Orchid,
    /// Specifies a color with RGB value (238,232,170)
    #[from_str = "paleGoldenrod"]
    PaleGoldenrod,
    /// Specifies a color with RGB value (152,251,152)
    #[from_str = "paleGreen"]
    PaleGreen,
    /// Specifies a color with RGB value (175,238,238)
    #[from_str = "paleTurquoise"]
    PaleTurquoise,
    /// Specifies a color with RGB value (219,112,147)
    #[from_str = "paleVioletRed"]
    PaleVioletRed,
    /// Specifies a color with RGB value (255,239,213)
    #[from_str = "papayaWhip"]
    PapayaWhip,
    /// Specifies a color with RGB value (255,218,185)
    #[from_str = "peachPuff"]
    PeachPuff,
    /// Specifies a color with RGB value (205,133,63)
    #[from_str = "peru"]
    Peru,
    /// Specifies a color with RGB value (255,192,203)
    #[from_str = "pink"]
    Pink,
    /// Specifies a color with RGB value (221,160,221)
    #[from_str = "plum"]
    Plum,
    /// Specifies a color with RGB value (176,224,230)
    #[from_str = "powderBlue"]
    PowderBlue,
    /// Specifies a color with RGB value (128,0,128)
    #[from_str = "purple"]
    Purple,
    /// Specifies a color with RGB value (255,0,0)
    #[from_str = "red"]
    Red,
    /// Specifies a color with RGB value (188,143,143)
    #[from_str = "rosyBrown"]
    RosyBrown,
    /// Specifies a color with RGB value (65,105,225)
    #[from_str = "royalBlue"]
    RoyalBlue,
    /// Specifies a color with RGB value (139,69,19)
    #[from_str = "saddleBrown"]
    SaddleBrown,
    /// Specifies a color with RGB value (250,128,114)
    #[from_str = "salmon"]
    Salmon,
    /// Specifies a color with RGB value (244,164,96)
    #[from_str = "sandyBrown"]
    SandyBrown,
    /// Specifies a color with RGB value (46,139,87)
    #[from_str = "seaGreen"]
    SeaGreen,
    /// Specifies a color with RGB value (255,245,238)
    #[from_str = "seaShell"]
    SeaShell,
    /// Specifies a color with RGB value (160,82,45)
    #[from_str = "sienna"]
    Sienna,
    /// Specifies a color with RGB value (192,192,192)
    #[from_str = "silver"]
    Silver,
    /// Specifies a color with RGB value (135,206,235)
    #[from_str = "skyBlue"]
    SkyBlue,
    /// Specifies a color with RGB value (106,90,205)
    #[from_str = "slateBlue"]
    SlateBlue,
    /// Specifies a color with RGB value (112,128,144)
    #[from_str = "slateGray"]
    SlateGray,
    /// Specifies a color with RGB value (112,128,144)
    #[from_str = "slateGrey"]
    SlateGrey,
    /// Specifies a color with RGB value (255,250,250)
    #[from_str = "snow"]
    Snow,
    /// Specifies a color with RGB value (0,255,127)
    #[from_str = "springGreen"]
    SpringGreen,
    /// Specifies a color with RGB value (70,130,180)
    #[from_str = "steelBlue"]
    SteelBlue,
    /// Specifies a color with RGB value (210,180,140)
    #[from_str = "tan"]
    Tan,
    /// Specifies a color with RGB value (0,128,128)
    #[from_str = "teal"]
    Teal,
    /// Specifies a color with RGB value (216,191,216)
    #[from_str = "thistle"]
    Thistle,
    /// Specifies a color with RGB value (255,99,71)
    #[from_str = "tomato"]
    Tomato,
    /// Specifies a color with RGB value (64,224,208)
    #[from_str = "turquoise"]
    Turquoise,
    /// Specifies a color with RGB value (238,130,238)
    #[from_str = "violet"]
    Violet,
    /// Specifies a color with RGB value (245,222,179)
    #[from_str = "wheat"]
    Wheat,
    /// Specifies a color with RGB value (255,255,255)
    #[from_str = "white"]
    White,
    /// Specifies a color with RGB value (245,245,245)
    #[from_str = "whiteSmoke"]
    WhiteSmoke,
    /// Specifies a color with RGB value (255,255,0)
    #[from_str = "yellow"]
    Yellow,
    /// Specifies a color with RGB value (154,205,50)
    #[from_str = "yellowGreen"]
    YellowGreen,
}

/// This simple type represents a scheme color value.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum SchemeColorVal {
    #[from_str = "bg1"]
    Background1,
    #[from_str = "tx1"]
    Text1,
    #[from_str = "bg2"]
    Background2,
    #[from_str = "tx2"]
    Text2,
    #[from_str = "accent1"]
    Accent1,
    #[from_str = "accent2"]
    Accent2,
    #[from_str = "accent3"]
    Accent3,
    #[from_str = "accent4"]
    Accent4,
    #[from_str = "accent5"]
    Accent5,
    #[from_str = "hlink"]
    Hypelinglink,
    #[from_str = "folHlink"]
    FollowedHyperlink,
    /// A color used in theme definitions which means to use the color of the style.
    #[from_str = "phClr"]
    PlaceholderColor,
    #[from_str = "dk1"]
    Dark1,
    #[from_str = "lt1"]
    Light1,
    #[from_str = "dk2"]
    Dark2,
    #[from_str = "lt2"]
    Light2,
}

/// A reference to a color in the color scheme.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum ColorSchemeIndex {
    #[from_str = "dk1"]
    Dark1,
    #[from_str = "lt1"]
    Light1,
    #[from_str = "dk2"]
    Dark2,
    #[from_str = "lt2"]
    Light2,
    #[from_str = "accent1"]
    Accent1,
    #[from_str = "accent2"]
    Accent2,
    #[from_str = "accent3"]
    Accent3,
    #[from_str = "accent4"]
    Accent4,
    #[from_str = "accent5"]
    Accent5,
    #[from_str = "accent6"]
    Accent6,
    #[from_str = "hlink"]
    Hyperlink,
    #[from_str = "folHlink"]
    FollowedHyperlink,
}

/// This simple type specifies the text alignment types
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextAlignType {
    /// Align text to the left margin.
    #[from_str = "l"]
    Left,
    /// Align text in the center.
    #[from_str = "ctr"]
    Center,
    /// Align text to the right margin.
    #[from_str = "r"]
    Right,
    /// Align text so that it is justified across the whole line. It is smart in the sense that it does not justify
    /// sentences which are short.
    #[from_str = "just"]
    Justified,
    /// Aligns the text with an adjusted kashida length for Arabic text.
    #[from_str = "justLow"]
    JustifiedLow,
    /// Distributes the text words across an entire text line.
    #[from_str = "dist"]
    Distributed,
    /// Distributes Thai text specially, because each character is treated as a word.
    #[from_str = "thaiDist"]
    ThaiDistributed,
}

/// This simple type specifies the different kinds of font alignment.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextFontAlignType {
    /// When the text flow is horizontal or simple vertical same as fontBaseline but for other vertical modes
    /// same as fontCenter.
    #[from_str = "auto"]
    Auto,
    /// The letters are anchored to the top baseline of a single line.
    #[from_str = "t"]
    Top,
    /// The letters are anchored between the two baselines of a single line.
    #[from_str = "ctr"]
    Center,
    /// The letters are anchored to the bottom baseline of a single line.
    #[from_str = "base"]
    Baseline,
    /// The letters are anchored to the very bottom of a single line. This is different than the bottom baseline because
    /// of letters such as "g," "q," "y," etc.
    #[from_str = "b"]
    Bottom,
}

/// This simple type specifies a list of automatic numbering schemes.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextAutonumberScheme {
    /// (a), (b), (c), …
    #[from_str = "alphaLcParenBoth"]
    AlphaLcParenBoth,
    /// (A), (B), (C), …
    #[from_str = "alphaUcParenBoth"]
    AlphaUcParenBoth,
    /// a), b), c), …
    #[from_str = "alphaLcParenR"]
    AlphaLcParenR,
    /// A), B), C), …
    #[from_str = "alphaUcParenR"]
    AlphaUcParenR,
    /// a., b., c., …
    #[from_str = "alphaLcPeriod"]
    AlphaLcPeriod,
    /// A., B., C., …
    #[from_str = "alphaUcPeriod"]
    AlphaUcPeriod,
    /// (1), (2), (3), …
    #[from_str = "arabicParenBoth"]
    ArabicParenBoth,
    /// 1), 2), 3), …
    #[from_str = "arabicParenR"]
    ArabicParenR,
    /// 1., 2., 3., …
    #[from_str = "arabicPeriod"]
    ArabicPeriod,
    /// 1, 2, 3, …
    #[from_str = "arabicPlain"]
    ArabicPlain,
    /// (i), (ii), (iii), …
    #[from_str = "romanLcParenBoth"]
    RomanLcParenBoth,
    /// (I), (II), (III), …
    #[from_str = "romanUcParenBoth"]
    RomanUcParenBoth,
    /// i), ii), iii), …
    #[from_str = "romanLcParenR"]
    RomanLcParenR,
    /// I), II), III), …
    #[from_str = "romanUcParenR"]
    RomanUcParenR,
    /// i., ii., iii., …
    #[from_str = "romanLcPeriod"]
    RomanLcPeriod,
    /// I., II., III., …
    #[from_str = "romanUcPeriod"]
    RomanUcPeriod,
    /// Dbl-byte circle numbers (1-10 circle[0x2460-], 11-arabic numbers)
    #[from_str = "circleNumDbPlain"]
    CircleNumDbPlain,
    /// Wingdings black circle numbers
    #[from_str = "circleNumWdBlackPlain"]
    CircleNumWdBlackPlain,
    /// Wingdings white circle numbers (0-10 circle[0x0080-], 11- arabic numbers)
    #[from_str = "circleNumWdWhitePlain"]
    CircleNumWdWhitePlain,
    /// Dbl-byte Arabic numbers w/ double-byte period
    #[from_str = "arabicDbPeriod"]
    ArabicDbPeriod,
    /// Dbl-byte Arabic numbers
    #[from_str = "arabicDbPlain"]
    ArabicDbPlain,
    /// EA: Simplified Chinese w/ single-byte period
    #[from_str = "ea1ChsPeriod"]
    Ea1ChsPeriod,
    /// EA: Simplified Chinese (TypeA 1-99, TypeC 100-)
    #[from_str = "ea1ChsPlain"]
    Ea1ChsPlain,
    /// EA: Traditional Chinese w/ single-byte period
    #[from_str = "ea1ChtPeriod"]
    Ea1ChtPeriod,
    /// EA: Traditional Chinese (TypeA 1-19, TypeC 20-)
    #[from_str = "ea1ChtPlain"]
    Ea1ChtPlain,
    /// EA: Japanese w/ double-byte period
    #[from_str = "ea1JpnChsDbPeriod"]
    Ea1JpnChsDbPeriod,
    /// EA: Japanese/Korean (TypeC 1-)
    #[from_str = "ea1JpnKorPlain"]
    Ea1JpnKorPlain,
    /// EA: Japanese/Korean w/ single-byte period
    #[from_str = "ea1JpnKorPeriod"]
    Ea1JpnKorPeriod,
    /// Bidi Arabic 1 (AraAlpha) with ANSI minus symbol
    #[from_str = "arabic1Minus"]
    Arabic1Minus,
    /// Bidi Arabic 2 (AraAbjad) with ANSI minus symbol
    #[from_str = "arabic2Minus"]
    Arabic2Minus,
    /// Bidi Hebrew 2 with ANSI minus symbol
    #[from_str = "hebrew2Minus"]
    Hebrew2Minus,
    /// Thai alphabet period
    #[from_str = "thaiAlphaPeriod"]
    ThaiAlphaPeriod,
    /// Thai alphabet parentheses - right
    #[from_str = "thaiAlphaParenR"]
    ThaiAlphaParenR,
    /// Thai alphabet parentheses - both
    #[from_str = "thaiAlphaParenBoth"]
    ThaiAlphaParenBoth,
    /// Thai numerical period
    #[from_str = "thaiNumPeriod"]
    ThaiNumPeriod,
    /// Thai numerical parentheses - right
    #[from_str = "thaiNumParenR"]
    ThaiNumParenR,
    /// Thai numerical period
    #[from_str = "thaiNumParenBoth"]
    ThaiNumParenBoth,
    /// Hindi alphabet period - consonants
    #[from_str = "hindiAlphaPeriod"]
    HindiAlphaPeriod,
    /// Hindi numerical period
    #[from_str = "hindiNumPeriod"]
    HindiNumPeriod,
    /// Hindi numerical parentheses - right
    #[from_str = "hindiNumParenR"]
    HindiNumParenR,
    /// Hindi alphabet period - consonants
    #[from_str = "hindiAlpha1Period"]
    HindiAlpha1Period,
}

/// This simple type describes the shape of path to follow for a path gradient shade.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PathShadeType {
    /// Gradient follows the shape
    #[from_str = "shape"]
    Shape,
    /// Gradient follows a circular path
    #[from_str = "circle"]
    Circle,
    /// Gradient follows a rectangular pat
    #[from_str = "rect"]
    Rect,
}

/// This simple type indicates a preset type of pattern fill. The description of each value contains an illustration of
/// the fill type.
///
/// # Note
///
/// These presets correspond to members of the HatchStyle enumeration in the Microsoft .NET Framework.
/// A reference for this type can be found at http://msdn2.microsoft.com/enus/library/system.drawing.drawing2d.hatchstyle.aspx
#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetPatternVal {
    #[from_str = "pct5"]
    Percent5,
    #[from_str = "pct10"]
    Percent10,
    #[from_str = "pct20"]
    Percent20,
    #[from_str = "pct25"]
    Percent25,
    #[from_str = "pct30"]
    Percent30,
    #[from_str = "pct40"]
    Percent40,
    #[from_str = "pct50"]
    Percent50,
    #[from_str = "pct60"]
    Percent60,
    #[from_str = "pct70"]
    Percent70,
    #[from_str = "pct75"]
    Percent75,
    #[from_str = "pct80"]
    Percent80,
    #[from_str = "pct90"]
    Percent90,
    #[from_str = "horz"]
    Horizontal,
    #[from_str = "vert"]
    Vertical,
    #[from_str = "ltHorz"]
    LightHorizontal,
    #[from_str = "ltVert"]
    LightVertical,
    #[from_str = "dkHorz"]
    DarkHorizontal,
    #[from_str = "dkVert"]
    DarkVertical,
    #[from_str = "narHorz"]
    NarrowHorizontal,
    #[from_str = "narVert"]
    NarrowVertical,
    #[from_str = "dashHorz"]
    DashedHorizontal,
    #[from_str = "dashVert"]
    DashedVertical,
    #[from_str = "cross"]
    Cross,
    #[from_str = "dnDiag"]
    DownwardDiagonal,
    #[from_str = "upDiag"]
    UpwardDiagonal,
    #[from_str = "ltDnDiag"]
    LightDownwardDiagonal,
    #[from_str = "ltUpDiag"]
    LightUpwardDiagonal,
    #[from_str = "dkDnDiag"]
    DarkDownwardDiagonal,
    #[from_str = "dkUpDiag"]
    DarkUpwardDiagonal,
    #[from_str = "wdDnDiag"]
    WideDownwardDiagonal,
    #[from_str = "wdUpDiag"]
    WideUpwardDiagonal,
    #[from_str = "dashDnDiag"]
    DashedDownwardDiagonal,
    #[from_str = "dashUpDiag"]
    DashedUpwardDiagonal,
    #[from_str = "diagCross"]
    DiagonalCross,
    #[from_str = "smCheck"]
    SmallCheckerBoard,
    #[from_str = "lgCheck"]
    LargeCheckerBoard,
    #[from_str = "smGrid"]
    SmallGrid,
    #[from_str = "lgGrid"]
    LargeGrid,
    #[from_str = "dotGrid"]
    DottedGrid,
    #[from_str = "smConfetti"]
    SmallConfetti,
    #[from_str = "lgConfetti"]
    LargeConfetti,
    #[from_str = "horzBrick"]
    HorizontalBrick,
    #[from_str = "diagBrick"]
    DiagonalBrick,
    #[from_str = "solidDmnd"]
    SolidDiamond,
    #[from_str = "openDmnd"]
    OpenDiamond,
    #[from_str = "dotDmnd"]
    DottedDiamond,
    #[from_str = "plaid"]
    Plaid,
    #[from_str = "sphere"]
    Sphere,
    #[from_str = "weave"]
    Weave,
    #[from_str = "divot"]
    Divot,
    #[from_str = "shingle"]
    Shingle,
    #[from_str = "wave"]
    Wave,
    #[from_str = "trellis"]
    Trellis,
    #[from_str = "zigzag"]
    ZigZag,
}

/// This simple type describes how to render effects one on top of another.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum BlendMode {
    #[from_str = "over"]
    Overlay,
    #[from_str = "mult"]
    Multiply,
    #[from_str = "screen"]
    Screen,
    #[from_str = "lighten"]
    Lighten,
    #[from_str = "darken"]
    Darken,
}

/// This simple type specifies the text tab alignment types.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextTabAlignType {
    /// The text at this tab stop is left aligned.
    #[from_str = "l"]
    Left,
    /// The text at this tab stop is center aligned.
    #[from_str = "ctr"]
    Center,
    /// The text at this tab stop is right aligned.
    #[from_str = "r"]
    Right,
    /// At this tab stop, the decimals are lined up. From a user's point of view, the text here behaves as right
    /// aligned until the decimal, and then as left aligned after the decimal.
    #[from_str = "dec"]
    Decimal,
}

/// This simple type specifies the text underline types that is used.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextUnderlineType {
    /// The reason we cannot implicitly have noUnderline be the scenario where underline is not specified is
    /// because not being specified implies deriving from a particular style and the user might want to override
    /// that and make some text not be underlined even though the style says otherwise.
    #[from_str = "none"]
    None,
    /// Underline just the words and not the spaces between them.
    #[from_str = "words"]
    Words,
    /// Underline the text with a single line of normal thickness.
    #[from_str = "sng"]
    Single,
    /// Underline the text with two lines of normal thickness.
    #[from_str = "dbl"]
    Double,
    /// Underline the text with a single, thick line.
    #[from_str = "heavy"]
    Heavy,
    /// Underline the text with a single, dotted line of normal thickness.
    #[from_str = "dotted"]
    Dotted,
    /// Underline the text with a single, thick, dotted line.
    #[from_str = "dottedHeavy"]
    DottedHeavy,
    /// Underline the text with a single, dashed line of normal thickness.
    #[from_str = "dash"]
    Dash,
    /// Underline the text with a single, dashed, thick line.
    #[from_str = "dashHeavy"]
    DashHeavy,
    /// Underline the text with a single line consisting of long dashes of normal thickness.
    #[from_str = "dashLong"]
    DashLong,
    /// Underline the text with a single line consisting of long, thick dashes.
    #[from_str = "dashLongHeavy"]
    DashLongHeavy,
    /// Underline the text with a single line of normal thickness consisting of repeating dots and dashes.
    #[from_str = "dotDash"]
    DotDash,
    /// Underline the text with a single, thick line consisting of repeating dots and dashes.
    #[from_str = "dotDashHeavy"]
    DotDashHeavy,
    /// Underline the text with a single line of normal thickness consisting of repeating two dots and dashes.
    #[from_str = "dotDotDash"]
    DotDotDash,
    /// Underline the text with a single, thick line consisting of repeating two dots and dashes.
    #[from_str = "dotDotDashHeavy"]
    DotDotDashHeavy,
    /// Underline the text with a single wavy line of normal thickness.
    #[from_str = "wavy"]
    Wavy,
    /// Underline the text with a single, thick wavy line.
    #[from_str = "wavyHeavy"]
    WavyHeavy,
    /// Underline just the words and not the spaces between them.
    #[from_str = "wavyDbl"]
    WavyDouble,
}

/// This simple type specifies the strike type.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextStrikeType {
    #[from_str = "noStrike"]
    NoStrike,
    #[from_str = "sngStrike"]
    SingleStrike,
    #[from_str = "dblStrike"]
    DoubleStrike,
}

/// This simple type specifies the cap types of the text.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextCapsType {
    /// The reason we cannot implicitly have noCaps be the scenario where capitalization is not specified is
    /// because not being specified implies deriving from a particular style and the user might want to override
    /// that and make some text not have a capitalization scheme even though the style says otherwise.
    #[from_str = "none"]
    None,
    /// Apply small caps to the text. All letters are converted to lower case.
    #[from_str = "small"]
    Small,
    /// Apply all caps on the text. All lower case letters are converted to upper case even though they are stored
    /// differently in the backing store.
    #[from_str = "all"]
    All,
}

/// This simple type specifies the preset text shape geometry that is to be used for a shape. An enumeration of this
/// simple type is used so that a custom geometry does not have to be specified but instead can be constructed
/// automatically by the generating application. For each enumeration listed there is also the corresponding
/// DrawingML code that would be used to construct this shape were it a custom geometry. Within the construction
/// code for each of these preset text shapes there are predefined guides that the generating application shall
/// maintain for calculation purposes at all times. See [ShapeType](enum.ShapeType.html) to see the necessary guide values.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextShapeType {
    #[from_str = "textNoShape"]
    NoShape,
    #[from_str = "textPlain"]
    Plain,
    #[from_str = "textStop"]
    Stop,
    #[from_str = "textTriangle"]
    Triangle,
    #[from_str = "textTriangleInverted"]
    TriangleInverted,
    #[from_str = "textChevron"]
    Chevron,
    #[from_str = "textChevronInverted"]
    ChevronInverted,
    #[from_str = "textRingInside"]
    RingInside,
    #[from_str = "textRingOutside"]
    RingOutside,
    #[from_str = "textArchUp"]
    ArchUp,
    #[from_str = "textArchDown"]
    ArchDown,
    #[from_str = "textCircle"]
    Circle,
    #[from_str = "textButton"]
    Button,
    #[from_str = "textArchUpPour"]
    ArchUpPour,
    #[from_str = "textArchDownPour"]
    ArchDownPour,
    #[from_str = "textCirclePour"]
    CirclePour,
    #[from_str = "textButtonPour"]
    ButtonPour,
    #[from_str = "textCurveUp"]
    CurveUp,
    #[from_str = "textCurveDown"]
    CurveDown,
    #[from_str = "textCanUp"]
    CanUp,
    #[from_str = "textCanDown"]
    CanDown,
    #[from_str = "textWave1"]
    Wave1,
    #[from_str = "textWave2"]
    Wave2,
    #[from_str = "textWave4"]
    Wave4,
    #[from_str = "textDoubleWave1"]
    DoubleWave1,
    #[from_str = "textInflate"]
    Inflate,
    #[from_str = "textDeflate"]
    Deflate,
    #[from_str = "textInflateBottom"]
    InflateBottom,
    #[from_str = "textDeflateBottom"]
    DeflateBottom,
    #[from_str = "textInflateTop"]
    InflateTop,
    #[from_str = "textDeflateTop"]
    DeflateTop,
    #[from_str = "textDeflateInflate"]
    DeflateInflate,
    #[from_str = "textDeflateInflateDeflate"]
    DeflateInflateDeflate,
    #[from_str = "textFadeLeft"]
    FadeLeft,
    #[from_str = "textFadeUp"]
    FadeUp,
    #[from_str = "textFadeRight"]
    FadeRight,
    #[from_str = "textFadeDown"]
    FadeDown,
    #[from_str = "textSlantUp"]
    SlantUp,
    #[from_str = "textSlantDown"]
    SlantDown,
    #[from_str = "textCascadeUp"]
    CascadeUp,
    #[from_str = "textCascadeDown"]
    CascadeDown,
}

/// This simple type specifies the text vertical overflow.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextVertOverflowType {
    /// Overflow the text and pay no attention to top and bottom barriers.
    #[from_str = "overflow"]
    Overflow,
    /// Pay attention to top and bottom barriers. Use an ellipsis to denote that there is text which is not visible.
    #[from_str = "ellipsis"]
    Ellipsis,
    /// Pay attention to top and bottom barriers. Provide no indication that there is text which is not visible.
    #[from_str = "clip"]
    Clip,
}

/// This simple type specifies the text horizontal overflow types
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextHorizontalOverflowType {
    /// When a big character does not fit into a line, allow a horizontal overflow.
    #[from_str = "overflow"]
    Overflow,
    /// When a big character does not fit into a line, clip it at the proper horizontal overflow.
    #[from_str = "clip"]
    Clip,
}

/// If there is vertical text, determines what kind of vertical text is going to be used.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextVerticalType {
    /// Horizontal text. This should be default.
    #[from_str = "horz"]
    Horizontal,
    /// Determines if all of the text is vertical orientation (each line is 90 degrees rotated clockwise, so it goes
    /// from top to bottom; each next line is to the left from the previous one).
    #[from_str = "vert"]
    Vertical,
    /// Determines if all of the text is vertical orientation (each line is 270 degrees rotated clockwise, so it goes
    /// from bottom to top; each next line is to the right from the previous one).
    #[from_str = "vert270"]
    Vertical270,
    /// Determines if all of the text is vertical ("one letter on top of another").
    #[from_str = "wordArtVert"]
    WordArtVertical,
    /// A special version of vertical text, where some fonts are displayed as if rotated by 90 degrees while some fonts
    /// (mostly East Asian) are displayed vertical.
    #[from_str = "eaVert"]
    EastAsianVertical,
    /// A special version of vertical text, where some fonts are displayed as if rotated by 90 degrees while some fonts
    /// (mostly East Asian) are displayed vertical. The difference between this and the eastAsianVertical is
    /// the text flows top down then LEFT RIGHT, instead of RIGHT LEFT
    #[from_str = "mongolianVert"]
    MongolianVertical,
    /// Specifies that vertical WordArt should be shown from right to left rather than left to right.
    #[from_str = "wordArtVertRtl"]
    WordArtVerticalRtl,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextWrappingType {
    /// No wrapping occurs on this text body. Words spill out without paying attention to the bounding rectangle
    /// boundaries.
    #[from_str = "none"]
    None,
    /// Determines whether we wrap words within the bounding rectangle.
    #[from_str = "square"]
    Square,
}

/// This simple type specifies a list of available anchoring types for text.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextAnchoringType {
    /// Anchor the text at the top of the bounding rectangle.
    #[from_str = "t"]
    Top,
    /// Anchor the text at the middle of the bounding rectangle.
    #[from_str = "ctr"]
    Center,
    /// Anchor the text at the bottom of the bounding rectangle.
    #[from_str = "b"]
    Bottom,
    /// Anchor the text so that it is justified vertically. When text is horizontal, this spaces out the actual lines of
    /// text and is almost always identical in behavior to 'distrib' (special case: if only 1 line, then anchored at
    /// top). When text is vertical, then it justifies the letters vertically. This is different than anchorDistributed,
    /// because in some cases such as very little text in a line, it does not justify.
    #[from_str = "just"]
    Justified,
    /// Anchor the text so that it is distributed vertically.
    /// When text is horizontal, this spaces out the actual lines of text and is almost always identical in behavior to
    /// anchorJustified (special case: if only 1 line, then anchored in middle). When text is vertical, then it
    /// distributes the letters vertically. This is different than anchorJustified, because it always forces
    /// distribution of the words, even if there are only one or two words in a line.
    #[from_str = "dist"]
    Distributed,
}

/// This simple type specifies how an object should be rendered when specified to be in black and white mode.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum BlackWhiteMode {
    /// Object rendered with normal coloring
    #[from_str = "clr"]
    Color,
    /// Object rendered with automatic coloring
    #[from_str = "auto"]
    Auto,
    /// Object rendered with gray coloring
    #[from_str = "gray"]
    Gray,
    /// Object rendered with light gray coloring
    #[from_str = "ltGray"]
    LightGray,
    /// Object rendered with inverse gray coloring
    #[from_str = "invGray"]
    InverseGray,
    /// Object rendered within gray and white coloring
    #[from_str = "grayWhite"]
    GrayWhite,
    /// Object rendered with black and gray coloring
    #[from_str = "blackGray"]
    BlackGray,
    /// Object rendered within black and white coloring
    #[from_str = "blackWhite"]
    BlackWhite,
    /// Object rendered with black-only coloring
    #[from_str = "black"]
    Black,
    /// Object rendered within white coloirng
    #[from_str = "white"]
    White,
    /// Object rendered with hidden coloring
    #[from_str = "hidden"]
    Hidden,
}

/// This simple type specifies the ways that an animation can be built, or animated.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationBuildType {
    #[from_str = "allAtOnce"]
    AllAtOnce,
}

/// This simple type specifies the build options available only for animating a diagram. These options specify the
/// manner in which the objects within the chart should be grouped and animated.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationDgmOnlyBuildType {
    /// Animate the diagram by elements. For a tree diagram the animation occurs by branch within the diagram tree.
    #[from_str = "one"]
    One,
    /// Animate the diagram by the elements within a level, animating them one level element at a time.
    #[from_str = "lvlOne"]
    LvlOne,
    /// Animate the diagram one level at a time, animating the whole level as one object
    #[from_str = "lvlAtOnce"]
    LvlAtOnce,
}

/// This simple type specifies the ways that a diagram animation can be built. That is, it specifies the way in which
/// the objects within the diagram graphical object should be animated.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationDgmBuildType {
    #[from_str = "allAtOnce"]
    AllAtOnce,
    #[from_str = "one"]
    One,
    #[from_str = "lvlOne"]
    LvlOne,
    #[from_str = "lvlAtOnce"]
    LvlAtOnce,
}

/// This simple type specifies the build options available only for animating a chart. These options specify the
/// manner in which the objects within the chart should be grouped and animated.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationChartOnlyBuildType {
    /// Animate by each series
    #[from_str = "series"]
    Series,
    /// Animate by each category
    #[from_str = "category"]
    Category,
    /// Animate by each element within the series
    #[from_str = "seriesElement"]
    SeriesElement,
    /// Animate by each element within the category
    #[from_str = "categoryElement"]
    CategoryElement,
}

/// This simple type specifies the ways that a chart animation can be built. That is, it specifies the way in which the
/// objects within the chart should be animated.
#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationChartBuildType {
    #[from_str = "allAtOnce"]
    AllAtOnce,
    #[from_str = "series"]
    Series,
    #[from_str = "category"]
    Category,
    #[from_str = "seriesElement"]
    SeriesElement,
    #[from_str = "categoryElement"]
    CategoryElement,
}

/// This type specifies the amount of compression that has been used for a particular binary large image or picture
/// (blip).
#[derive(Debug, Clone, Copy, FromStr)]
pub enum BlipCompression {
    /// Compression size suitable for inclusion with email
    #[from_str = "email"]
    Email,
    /// Compression size suitable for viewing on screen
    #[from_str = "screen"]
    Screen,
    /// Compression size suitable for printing
    #[from_str = "print"]
    Print,
    /// Compression size suitable for high quality printing
    #[from_str = "hqprint"]
    HqPrint,
    /// No compression was used
    #[from_str = "none"]
    None,
}
