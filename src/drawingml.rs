use crate::error::{
    AdjustParseError, Limit, LimitViolationError, MissingAttributeError, MissingChildNodeError, NotGroupMemberError
};
use crate::relationship::RelationshipId;
use crate::xml::{parse_xml_bool, XmlNode};
use std::io::Read;
use std::str::FromStr;
use zip::read::ZipFile;
use log::{trace, error};
use enum_from_str::ParseEnumVariantError;
use enum_from_str_derive::FromStr;

pub type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub type Guid = String; // TODO: move to shared common types. pattern="\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\}"
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
pub type TextSpacingPercent = Percentage; // TODO: 0 <= n <= 13200000
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
pub type TextLanguageID = String;
pub type Panose = String; // TODO: hex, length=10
/// This simple type specifies the range that the start at number for a bullet's auto-numbering sequence can begin
/// at. When the numbering is alphabetical, then the numbers map to the appropriate letter. 1->a, 2->b, etc. If the
/// numbers go above 26, then the numbers begin to double up. For example, 27->aa and 53->aaa.
/// 
/// Values represented by this type are restricted to: 1 <= n <= 32767
pub type TextBulletStartAtNum = i32;
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

#[derive(Debug, Clone)]
pub enum ColorTransform {
    /// This element specifies a lighter version of its input color. A 10% tint is 10% of the input color combined with
    /// 90% white.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (BC, FF, BC)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:tint val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Tint(PositiveFixedPercentage),
    /// This element specifies a darker version of its input color. A 10% shade is 10% of the input color combined with
    /// 90% black.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (00, BC, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:shade val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Shade(PositiveFixedPercentage),
    /// This element specifies that the color rendered should be the complement of its input color with the complement
    /// being defined as such. Two colors are called complementary if, when mixed they produce a shade of grey. For
    /// instance, the complement of red which is RGB (255, 0, 0) is cyan which is RGB (0, 255, 255).
    /// 
    /// Primary colors and secondary colors are typically paired in this way:
    /// * red and cyan (where cyan is the mixture of green and blue)
    /// * green and magenta (where magenta is the mixture of red and blue)
    /// * blue and yellow (where yellow is the mixture of red and green)
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="FF0000">
    ///     <a:comp/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Complement,
    /// This element specifies the inverse of its input color.
    /// 
    /// # Xml example
    /// 
    /// The inverse of red (1, 0, 0) is cyan (0, 1, 1).
    /// 
    /// The following represents cyan, the inverse of red:
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="FF0000">
    ///     <a:inv/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Inverse,
    /// This element specifies a grayscale of its input color, taking into relative intensities of the red, green, and blue
    /// primaries.
    Grayscale,
    /// This element specifies its input color with the specific opacity, but with its color unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following represents a green solid fill which is 50% opaque
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:alpha val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Alpha(PositiveFixedPercentage),
    /// This element specifies a more or less opaque version of its input color. Increases or decreases the input alpha
    /// percentage by the specified percentage offset. A 10% alpha offset increases a 50% opacity to 60%. A -10% alpha
    /// offset decreases a 50% opacity to 40%. The transformed alpha values are limited to a range of 0 to 100%. A 10%
    /// alpha offset increase to a 100% opaque object still results in 100% opacity.
    /// 
    /// # Xml example
    /// 
    /// The following represents a green solid fill which is 90% opaque
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:alphaOff val="-10000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    AlphaOffset(FixedPercentage),
    /// This element specifies a more or less opaque version of its input color. An alpha modulate never increases the
    /// alpha beyond 100%. A 200% alpha modulate makes a input color twice as opaque as before. A 50% alpha
    /// modulate makes a input color half as opaque as before.
    /// 
    /// # Xml example
    /// 
    /// The following represents a green solid fill which is 50% opaque
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:alphaMod val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    AlphaModulate(PositivePercentage),
    /// This element specifies the input color with the specified hue, but with its saturation and luminance unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following two solid fills are equivalent.
    /// ```xml
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="100000" lum="50000">
    /// </a:solidFill>
    /// <a:solidFill>
    ///   <a:hslClr hue="0" sat="100000" lum="50000">
    ///     <a:hue val="14400000"/>
    ///   <a:hslClr/>
    /// </a:solidFill>
    /// ```
    Hue(PositiveFixedAngle),
    /// This element specifies the input color with its hue shifted, but with its saturation and luminance unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following increases the hue angular value by 10 degrees.
    /// ```xml
    /// <a:solidFill>
    ///   <a:hslClr hue="0" sat="100000" lum="50000"/>
    ///   <a:hueOff val="600000"/>
    /// </a:solidFill>
    /// ```
    HueOffset(Angle),
    /// This element specifies the input color with its hue modulated by the given percentage. A 50% hue modulate
    /// decreases the angular hue value by half. A 200% hue modulate doubles the angular hue value.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="100000" lum="50000">
    ///     <a:hueMod val="50000"/>
    ///   </a:hslClr>
    /// </a:solidFill>
    /// ```
    HueModulate(PositivePercentage),
    /// This element specifies the input color with the specified saturation, but with its hue and luminance unchanged.
    /// Typically saturation values fall in the range [0%, 100%].
    /// 
    /// # Xml example
    /// 
    /// The following two solid fills are equivalent:
    /// ```xml
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="100000" lum="50000">
    /// </a:solidFill>
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="0" lum="50000">
    ///     <a:sat val="100000"/>
    ///   <a:hslClr/>
    /// </a:solidFill>
    /// ```
    Saturation(Percentage),
    /// This element specifies the input color with its saturation shifted, but with its hue and luminance unchanged. A
    /// 10% offset to 20% saturation yields 30% saturation.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (19, E5, 19)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:satOff val="-20000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    SaturationOffset(Percentage),
    /// This element specifies the input color with its saturation modulated by the given percentage. A 50% saturation
    /// modulate reduces the saturation by half. A 200% saturation modulate doubles the saturation.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (66, 99, 66)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:satMod val="20000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    SaturationModulate(Percentage),
    /// This element specifies the input color with the specified luminance, but with its hue and saturation unchanged.
    /// Typically luminance values fall in the range [0%, 100%].
    /// 
    /// # Xml example
    /// 
    /// The following two solid fills are equivalent:
    /// ```xml
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="100000" lum="50000">
    /// </a:solidFill>
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="100000" lum="0">
    ///     <a:lum val="50000"/>
    ///   <a:hslClr/>
    /// </a:solidFill>
    /// ```
    Luminance(Percentage),
    /// This element specifies the input color with its luminance shifted, but with its hue and saturation unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (00, 99, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:lumOff val="-20000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    LuminanceOffset(Percentage),
    /// This element specifies the input color with its luminance modulated by the given percentage. A 50% luminance
    /// modulate reduces the luminance by half. A 200% luminance modulate doubles the luminance.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (00, 75, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:lumMod val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    LuminanceModulate(Percentage),
    /// This element specifies the input color with the specified red component, but with its green and blue color
    /// components unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (FF, FF, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:red val="100000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Red(Percentage),
    /// This element specifies the input color with its red component shifted, but with its green and blue color
    /// components unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (FF, 00, 00)
    /// to value RRGGBB= (CC, 00, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="FF0000">
    ///     <a:redOff val="-20000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    RedOffset(Percentage),
    /// This element specifies the input color with its red component modulated by the given percentage. A 50% red
    /// modulate reduces the red component by half. A 200% red modulate doubles the red component.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (FF, 00, 00)
    /// to value RRGGBB= (80, 00, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="FF0000">
    ///     <a:redMod val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    RedModulate(Percentage),
    /// This elements specifies the input color with the specified green component, but with its red and blue color
    /// components unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, 00, FF)
    /// to value RRGGBB= (00, FF, FF)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="0000FF">
    ///     <a:green val="100000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Green(Percentage),
    /// This element specifies the input color with its green component shifted, but with its red and blue color
    /// components unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (00, CC, 00).
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:greenOff val="-20000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    GreenOffset(Percentage),
    /// This element specifies the input color with its green component modulated by the given percentage. A 50%
    /// green modulate reduces the green component by half. A 200% green modulate doubles the green component.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (00, 80, 00)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:greenMod val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    GreenModulate(Percentage),
    /// This element specifies the input color with the specific blue component, but with the red and green color
    /// components unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, FF, 00)
    /// to value RRGGBB= (00, FF, FF)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:blue val="100000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    Blue(Percentage),
    /// This element specifies the input color with its blue component shifted, but with its red and green color
    /// components unchanged.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, 00, FF)
    /// to value RRGGBB= (00, 00, CC)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="00FF00">
    ///     <a:blueOff val="-20000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    BlueOffset(Percentage),
    /// This element specifies the input color with its blue component modulated by the given percentage. A 50% blue
    /// modulate reduces the blue component by half. A 200% blue modulate doubles the blue component.
    /// 
    /// # Xml example
    /// 
    /// The following manipulates the fill from having RGB value RRGGBB = (00, 00, FF)
    /// to value RRGGBB= (00, 00, 80)
    /// ```xml
    /// <a:solidFill>
    ///   <a:srgbClr val="0000FF">
    ///     <a:blueMod val="50000"/>
    ///   </a:srgbClr>
    /// </a:solidFill>
    /// ```
    BlueModulate(Percentage),
    /// This element specifies that the output color rendered by the generating application should be the sRGB gamma
    /// shift of the input color.
    Gamma,
    /// This element specifies that the output color rendered by the generating application should be the inverse sRGB
    /// gamma shift of the input color.
    InverseGamma,
}

impl ColorTransform {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "tint" | "shade" | "comp" | "inv" | "gray" | "alpha" | "alphaOff" | "alphaMod" | "hue" | "hueOff"
            | "hueMod" | "sat" | "satOff" | "satMod" | "lum" | "lumOff" | "lumMod" | "red" | "redOff" | "redMod"
            | "green" | "greenOff" | "greenMod" | "blue" | "blueOff" | "blueMod" | "gamma" | "invGamma" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<ColorTransform> {
        match xml_node.local_name() {
            "tint" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Tint(value.parse::<PositiveFixedPercentage>()?))
            }
            "shade" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Shade(value.parse::<PositiveFixedPercentage>()?))
            }
            "comp" => Ok(ColorTransform::Complement),
            "inv" => Ok(ColorTransform::Inverse),
            "gray" => Ok(ColorTransform::Grayscale),
            "alpha" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Alpha(value.parse::<PositiveFixedPercentage>()?))
            }
            "alphaOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::AlphaOffset(value.parse::<FixedPercentage>()?))
            }
            "alphaMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::AlphaModulate(value.parse::<FixedPercentage>()?))
            }
            "hue" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Hue(value.parse::<PositiveFixedAngle>()?))
            }
            "hueOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::HueOffset(value.parse::<Angle>()?))
            }
            "hueMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::HueModulate(value.parse::<PositivePercentage>()?))
            }
            "sat" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Saturation(value.parse::<Percentage>()?))
            }
            "satOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::SaturationOffset(value.parse::<Percentage>()?))
            }
            "satMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::SaturationModulate(value.parse::<Percentage>()?))
            }
            "lum" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Luminance(value.parse::<Percentage>()?))
            }
            "lumOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::LuminanceOffset(value.parse::<Percentage>()?))
            }
            "lumMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::LuminanceModulate(value.parse::<Percentage>()?))
            }
            "red" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Red(value.parse::<Percentage>()?))
            }
            "redOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::RedOffset(value.parse::<Percentage>()?))
            }
            "redMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::RedModulate(value.parse::<Percentage>()?))
            }
            "green" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Green(value.parse::<Percentage>()?))
            }
            "greenOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::GreenOffset(value.parse::<Percentage>()?))
            }
            "greenMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::GreenModulate(value.parse::<Percentage>()?))
            }
            "blue" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::Blue(value.parse::<Percentage>()?))
            }
            "blueOff" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::BlueOffset(value.parse::<Percentage>()?))
            }
            "blueMod" => {
                let value = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(ColorTransform::BlueModulate(value.parse::<Percentage>()?))
            }
            "gamma" => Ok(ColorTransform::Gamma),
            "invGamma" => Ok(ColorTransform::InverseGamma),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_ColorTransform").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScRgbColor {
    /// Specifies the percentage of red.
    pub r: Percentage,
    /// Specifies the percentage of green.
    pub g: Percentage,
    /// Specifies the percentage of blue.
    pub b: Percentage,
    pub color_transforms: Vec<ColorTransform>,
}

impl ScRgbColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<ScRgbColor> {
        let mut opt_r = None;
        let mut opt_g = None;
        let mut opt_b = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "r" => opt_r = Some(value.parse::<Percentage>()?),
                "g" => opt_g = Some(value.parse::<Percentage>()?),
                "b" => opt_b = Some(value.parse::<Percentage>()?),
                _ => (),
            }
        }

        let r = opt_r.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "r"))?;
        let g = opt_g.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "g"))?;
        let b = opt_b.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "b"))?;

        let mut color_transforms = Vec::new();

        for child_node in &xml_node.child_nodes {
            if ColorTransform::is_choice_member(child_node.local_name()) {
                color_transforms.push(ColorTransform::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            r,
            g,
            b,
            color_transforms,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SRgbColor {
    pub value: u32,
    pub color_transforms: Vec<ColorTransform>,
}

impl SRgbColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<SRgbColor> {
        let val_attr = xml_node
            .attribute("val")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
        let value = u32::from_str_radix(val_attr, 16)?;

        let mut color_transforms = Vec::new();

        for child_node in &xml_node.child_nodes {
            if ColorTransform::is_choice_member(child_node.local_name()) {
                color_transforms.push(ColorTransform::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            value,
            color_transforms,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HslColor {
    /// Specifies the angular value describing the wavelength. Expressed in 1/6000ths of a
    /// degree.
    pub hue: PositiveFixedAngle,
    /// Specifies the saturation referring to the purity of the hue. Expressed as a percentage with
    /// 0% referring to grey, 100% referring to the purest form of the hue.
    pub saturation: Percentage,
    /// Specifies the luminance referring to the lightness or darkness of the color. Expressed as a
    /// percentage with 0% referring to maximal dark (black) and 100% referring to maximal
    /// white.
    pub luminance: Percentage,
    pub color_transforms: Vec<ColorTransform>,
}

impl HslColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<HslColor> {
        let mut opt_h = None;
        let mut opt_s = None;
        let mut opt_l = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "hue" => opt_h = Some(value.parse::<PositiveFixedAngle>()?),
                "sat" => opt_s = Some(value.parse::<Percentage>()?),
                "lum" => opt_l = Some(value.parse::<Percentage>()?),
                _ => (),
            }
        }

        let hue = opt_h.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "hue"))?;
        let saturation = opt_s.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "sat"))?;
        let luminance = opt_l.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "lum"))?;

        let mut color_transforms = Vec::new();

        for child_node in &xml_node.child_nodes {
            if ColorTransform::is_choice_member(child_node.local_name()) {
                color_transforms.push(ColorTransform::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            hue,
            saturation,
            luminance,
            color_transforms,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SystemColor {
    /// Specifies the system color value.
    pub value: SystemColorVal,
    /// Specifies the color value that was last computed by the generating application.
    pub last_color: Option<HexColorRGB>,
    pub color_transforms: Vec<ColorTransform>,
}

impl SystemColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<SystemColor> {
        let mut opt_val = None;
        let mut last_color = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "val" => opt_val = Some(value.parse::<SystemColorVal>()?),
                "lastClr" => last_color = Some(value.clone()),
                _ => (),
            }
        }

        let value = opt_val.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;

        let mut color_transforms = Vec::new();

        for child_node in &xml_node.child_nodes {
            if ColorTransform::is_choice_member(child_node.local_name()) {
                color_transforms.push(ColorTransform::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            value,
            last_color,
            color_transforms,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PresetColor {
    pub value: PresetColorVal,
    pub color_transforms: Vec<ColorTransform>,
}

impl PresetColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<PresetColor> {
        let attr_val = xml_node
            .attribute("val")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
        let value = attr_val.parse::<PresetColorVal>()?;

        let mut color_transforms = Vec::new();

        for child_node in &xml_node.child_nodes {
            if ColorTransform::is_choice_member(child_node.local_name()) {
                color_transforms.push(ColorTransform::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            value,
            color_transforms,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SchemeColor {
    pub value: SchemeColorVal,
    pub color_transforms: Vec<ColorTransform>,
}

impl SchemeColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<SchemeColor> {
        let attr_val = xml_node
            .attribute("val")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
        let value = attr_val.parse::<SchemeColorVal>()?;

        let mut color_transforms = Vec::new();

        for child_node in &xml_node.child_nodes {
            if ColorTransform::is_choice_member(child_node.local_name()) {
                color_transforms.push(ColorTransform::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            value,
            color_transforms,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    /// This element specifies a color using the red, green, blue RGB color model. Each component, red, green, and blue
    /// is expressed as a percentage from 0% to 100%. A linear gamma of 1.0 is assumed.
    /// 
    /// Specifies the level of red as expressed by a percentage offset increase or decrease relative to the input color.
    /// 
    /// # Xml example
    /// 
    /// The following represent the same color
    /// ```xml
    /// <a:solidFill>
    ///   <a:scrgbClr r="50000" g="50000" b="50000"/>
    /// </a:solidFill>
    /// <a:solidFill>
    ///   <a:srgbClr val="BCBCBC"/>
    /// </a:solidFill>
    /// ```
    ScRgbColor(Box<ScRgbColor>),
    /// This element specifies a color using the red, green, blue RGB color model. Red, green, and blue is expressed as
    /// sequence of hex digits, RRGGBB. A perceptual gamma of 2.2 is used.
    /// 
    /// Specifies the level of red as expressed by a percentage offset increase or decrease relative to the input color.
    /// 
    /// # Xml example
    /// 
    /// The following represent the same color
    /// ```xml
    /// <a:solidFill>
    ///   <a:scrgbClr r="50000" g="50000" b="50000"/>
    /// </a:solidFill>
    /// <a:solidFill>
    ///   <a:srgbClr val="BCBCBC"/>
    /// </a:solidFill>
    /// ```
    SRgbColor(Box<SRgbColor>),
    /// This element specifies a color using the HSL color model. A perceptual gamma of 2.2 is assumed.
    /// 
    /// Hue refers to the dominant wavelength of color, saturation refers to the purity of its hue, and luminance refers
    /// to its lightness or darkness.
    /// 
    /// As with all colors, colors defined with the HSL color model can have color transforms applied to it.
    /// 
    /// # Xml example
    /// 
    /// The color blue having RGB value RRGGBB = (00, 00, 80) is equivalent to
    /// ```xml
    /// <a:solidFill>
    ///   <a:hslClr hue="14400000" sat="100000" lum="50000">
    /// </a:solidFill>
    /// ```
    HslColor(Box<HslColor>),
    /// This element specifies a color bound to predefined operating system elements.
    /// 
    /// # Xml example
    /// 
    /// The following represents the default color used for displaying text in a window.
    /// ```xml
    /// <a:solidFill>
    ///   <a:sysClr val="windowText"/>
    /// </a:solidFill>
    /// ```
    SystemColor(Box<SystemColor>),
    /// This element specifies a color bound to a user's theme. As with all elements which define a color, it is possible to
    /// apply a list of color transforms to the base color defined.
    /// 
    /// # Xml example
    /// 
    /// <a:solidFill>
    ///   <a:schemeClr val="lt1"/>
    /// </a:solidFill>
    SchemeColor(Box<SchemeColor>),
    /// This element specifies a color which is bound to one of a predefined collection of colors.
    /// 
    /// # Xml example
    /// 
    /// The following defines a solid fill bound to the "black" preset color.
    /// <a:solidFill>
    ///   <a:prstClr val="black">
    /// </a:solidFill>
    PresetColor(Box<PresetColor>),
}

impl Color {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "scrgbClr" | "srgbClr" | "hslClr" | "sysClr" | "schemeClr" | "prstClr" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Color> {
        match xml_node.local_name() {
            "scrgbClr" => Ok(Color::ScRgbColor(Box::new(ScRgbColor::from_xml_element(xml_node)?))),
            "srgbClr" => Ok(Color::SRgbColor(Box::new(SRgbColor::from_xml_element(xml_node)?))),
            "hslClr" => Ok(Color::HslColor(Box::new(HslColor::from_xml_element(xml_node)?))),
            "sysClr" => Ok(Color::SystemColor(Box::new(SystemColor::from_xml_element(xml_node)?))),
            "schemeClr" => Ok(Color::SchemeColor(Box::new(SchemeColor::from_xml_element(xml_node)?))),
            "prstClr" => Ok(Color::PresetColor(Box::new(PresetColor::from_xml_element(xml_node)?))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_ColorChoice").into()),
        }
    }
}

/// This element defines a custom color. The custom colors are used within a custom color list to define custom
/// colors that are extra colors that can be appended to a theme. This is useful within corporate scenarios where
/// there is a set corporate color palette from which to work.
#[derive(Debug, Clone)]
pub struct CustomColor {
    /// The name of the color shown in the color picker.
    pub name: Option<String>,
    pub color: Color,
}

impl CustomColor {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let name = xml_node.attribute("name").cloned();
        let color_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_ColorChoice"))?;
        let color = Color::from_xml_element(color_node)?;

        Ok(Self { name, color })
    }
}

#[derive(Debug, Clone)]
pub struct ColorMapping {
    /// A color defined which is associated as the first background color.
    pub background1: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the first text color.
    pub text1: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the second background color.
    pub background2: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the second text color.
    pub text2: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the accent 1 color.
    pub accent1: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the accent 2 color.
    pub accent2: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the accent 3 color.
    pub accent3: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the accent 4 color.
    pub accent4: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the accent 5 color.
    pub accent5: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the accent 6 color.
    pub accent6: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the color for a hyperlink.
    pub hyperlink: ColorSchemeIndex,
    /// Specifies a color defined which is associated as the color for a followed hyperlink.
    pub followed_hyperlink: ColorSchemeIndex,
}

impl ColorMapping {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut background1 = None;
        let mut text1 = None;
        let mut background2 = None;
        let mut text2 = None;
        let mut accent1 = None;
        let mut accent2 = None;
        let mut accent3 = None;
        let mut accent4 = None;
        let mut accent5 = None;
        let mut accent6 = None;
        let mut hyperlink = None;
        let mut followed_hyperlink = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "bg1" => background1 = Some(value.parse()?),
                "tx1" => text1 = Some(value.parse()?),
                "bg2" => background2 = Some(value.parse()?),
                "tx2" => text2 = Some(value.parse()?),
                "accent1" => accent1 = Some(value.parse()?),
                "accent2" => accent2 = Some(value.parse()?),
                "accent3" => accent3 = Some(value.parse()?),
                "accent4" => accent4 = Some(value.parse()?),
                "accent5" => accent5 = Some(value.parse()?),
                "accent6" => accent6 = Some(value.parse()?),
                "hlink" => hyperlink = Some(value.parse()?),
                "folHlink" => followed_hyperlink = Some(value.parse()?),
                _ => (),
            }
        }

        let background1 = background1.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "bg1"))?;
        let text1 = text1.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "tx1"))?;
        let background2 = background2.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "bg2"))?;
        let text2 = text2.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "tx2"))?;
        let accent1 = accent1.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "accent1"))?;
        let accent2 = accent2.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "accent2"))?;
        let accent3 = accent3.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "accent3"))?;
        let accent4 = accent4.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "accent4"))?;
        let accent5 = accent5.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "accent5"))?;
        let accent6 = accent6.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "accent6"))?;
        let hyperlink = hyperlink.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "hlink"))?;
        let followed_hyperlink =
            followed_hyperlink.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "folHlink"))?;

        Ok(Self {
            background1,
            text1,
            background2,
            text2,
            accent1,
            accent2,
            accent3,
            accent4,
            accent5,
            accent6,
            hyperlink,
            followed_hyperlink,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// The common name for this color scheme. This name can show up in the user interface in
    /// a list of color schemes.
    pub name: String,
    /// This element defines a color that happens to be the dark 1 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub dark1: Color,
    /// This element defines a color that happens to be the accent 1 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub light1: Color,
    /// This element defines a color that happens to be the dark 2 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub dark2: Color,
    /// This element defines a color that happens to be the accent 1 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub light2: Color,
    /// This element defines a color that happens to be the accent 1 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub accent1: Color,
    /// This element defines a color that happens to be the accent 2 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub accent2: Color,
    /// This element defines a color that happens to be the accent 3 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub accent3: Color,
    /// This element defines a color that happens to be the accent 4 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub accent4: Color,
    /// This element defines a color that happens to be the accent 5 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub accent5: Color,
    /// This element defines a color that happens to be the accent 6 color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub accent6: Color,
    /// This element defines a color that happens to be the hyperlink color. The set of twelve colors come together to
    /// form the color scheme for a theme.
    pub hyperlink: Color,
    /// This element defines a color that happens to be the followed hyperlink color. The set of twelve colors come
    /// together to form the color scheme for a theme.
    pub followed_hyperlink: Color,
}

impl ColorScheme {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let name_attr = xml_node
            .attribute("name")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "name"))?;
        let name = name_attr.clone();

        let mut dk1 = None;
        let mut lt1 = None;
        let mut dk2 = None;
        let mut lt2 = None;
        let mut accent1 = None;
        let mut accent2 = None;
        let mut accent3 = None;
        let mut accent4 = None;
        let mut accent5 = None;
        let mut accent6 = None;
        let mut hyperlink = None;
        let mut follow_hyperlink = None;

        for child_node in &xml_node.child_nodes {
            let scheme_node = child_node
                .child_nodes
                .get(0)
                .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "scheme color value"))?;

            match child_node.local_name() {
                "dk1" => dk1 = Some(Color::from_xml_element(&scheme_node)?),
                "lt1" => lt1 = Some(Color::from_xml_element(&scheme_node)?),
                "dk2" => dk2 = Some(Color::from_xml_element(&scheme_node)?),
                "lt2" => lt2 = Some(Color::from_xml_element(&scheme_node)?),
                "accent1" => accent1 = Some(Color::from_xml_element(&scheme_node)?),
                "accent2" => accent2 = Some(Color::from_xml_element(&scheme_node)?),
                "accent3" => accent3 = Some(Color::from_xml_element(&scheme_node)?),
                "accent4" => accent4 = Some(Color::from_xml_element(&scheme_node)?),
                "accent5" => accent5 = Some(Color::from_xml_element(&scheme_node)?),
                "accent6" => accent6 = Some(Color::from_xml_element(&scheme_node)?),
                "hlink" => hyperlink = Some(Color::from_xml_element(&scheme_node)?),
                "folHlink" => follow_hyperlink = Some(Color::from_xml_element(&scheme_node)?),
                _ => (),
            }
        }

        let dark1 = dk1.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "dk1"))?;
        let light1 = lt1.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "lt1"))?;
        let dark2 = dk2.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "dk2"))?;
        let light2 = lt2.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "lt2"))?;
        let accent1 = accent1.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "accent1"))?;
        let accent2 = accent2.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "accent2"))?;
        let accent3 = accent3.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "accent3"))?;
        let accent4 = accent4.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "accent4"))?;
        let accent5 = accent5.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "accent5"))?;
        let accent6 = accent6.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "accent6"))?;
        let hyperlink = hyperlink.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "hlink"))?;
        let followed_hyperlink =
            follow_hyperlink.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "folHlink"))?;

        Ok(Self {
            name,
            dark1,
            light1,
            dark2,
            light2,
            accent1,
            accent2,
            accent3,
            accent4,
            accent5,
            accent6,
            hyperlink,
            followed_hyperlink,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ColorMappingOverride {
    /// This element is a part of a choice for which color mapping is used within the document. 
    /// If this element is specified, then we specifically use the color mapping defined in the master.
    UseMaster,
    /// This element provides an override for the color mapping in a document. When defined, this color mapping is
    /// used in place of the already defined color mapping, or master color mapping. This color mapping is defined in
    /// the same manner as the other mappings within this document.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <overrideClrMapping bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1"
    ///   accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5"
    ///   accent6="accent6" hlink="hlink" folHlink="folHlink"/>
    /// ```
    Override(Box<ColorMapping>),
}

impl ColorMappingOverride {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "masterClrMapping" | "overrideClrMapping" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "masterClrMapping" => Ok(ColorMappingOverride::UseMaster),
            "overrideClrMapping" => Ok(ColorMappingOverride::Override(Box::new(
                ColorMapping::from_xml_element(xml_node)?,
            ))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "CT_ColorMappingOverride").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorSchemeAndMapping {
    /// This element defines a set of colors which are referred to as a color scheme. The color scheme is responsible for
    /// defining a list of twelve colors. The twelve colors consist of six accent colors, two dark colors, two light colors
    /// and a color for each of a hyperlink and followed hyperlink.
    /// 
    /// The Color Scheme Color elements appear in a sequence. The following listing shows the index value and
    /// corresponding Color Name.
    /// 
    /// | Sequence Index        | Element (Color) Name              |
    /// |-----------------------|-----------------------------------|
    /// |0                      |dark1                              |
    /// |1                      |light1                             |
    /// |2                      |dark2                              |
    /// |3                      |light2                             |
    /// |4                      |accent1                            |
    /// |5                      |accent2                            |
    /// |6                      |accent3                            |
    /// |7                      |accent4                            |
    /// |8                      |accent5                            |
    /// |9                      |accent6                            |
    /// |10                     |hyperlink                          |
    /// |11                     |followedHyperlink                  |
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <clrScheme name="sample">
    ///   <dk1>
    ///     <sysClr val="windowText"/>
    ///   </dk1>
    ///   <lt1>
    ///     <sysClr val="window"/>
    ///   </lt1>
    ///   <dk2>
    ///     <srgbClr val="04617B"/>
    ///   </dk2>
    ///   <lt2>
    ///     <srgbClr val="DBF5F9"/>
    ///   </lt2>
    ///   <accent1>
    ///     <srgbClr val="0F6FC6"/>
    ///   </accent1>
    ///   <accent2>
    ///     <srgbClr val="009DD9"/>
    ///   </accent2>
    ///   <accent3>
    ///     <srgbClr val="0BD0D9"/>
    ///   </accent3>
    ///   <accent4>
    ///     <srgbClr val="10CF9B"/>
    ///   </accent4>
    ///   <accent5>
    ///     <srgbClr val="7CCA62"/>
    ///   </accent5>
    ///   <accent6>
    ///     <srgbClr val="A5C249"/>
    ///   </accent6>
    ///   <hlink>
    ///     <srgbClr val="FF9800"/>
    ///   </hlink>
    ///   <folHlink>
    ///     <srgbClr val="F45511"/>
    ///   </folHlink>
    /// </clrScheme>
    /// ```
    pub color_scheme: Box<ColorScheme>,
    /// This element specifics the color mapping layer which allows a user to define colors for background and text.
    /// This allows for swapping out of light/dark colors for backgrounds and the text on top of the background in order
    /// to maintain readability of the text On a deeper level, this specifies exactly which colors the first 12 values refer
    /// to in the color scheme.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1"
    /// accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5"
    /// accent6="accent6" hlink="hlink" folHlink="folHlink"/>
    /// ```
    /// 
    /// In this example, we see that bg1 is mapped to lt1, tx1 is mapped to dk1, and so on.
    pub color_mapping: Option<Box<ColorMapping>>,
}

impl ColorSchemeAndMapping {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut color_scheme = None;
        let mut color_mapping = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "clrScheme" => color_scheme = Some(Box::new(ColorScheme::from_xml_element(child_node)?)),
                "clrMap" => color_mapping = Some(Box::new(ColorMapping::from_xml_element(child_node)?)),
                _ => (),
            }
        }

        let color_scheme =
            color_scheme.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "clrScheme"))?;

        Ok(Self {
            color_scheme,
            color_mapping,
        })
    }
}

/// GradientStop
#[derive(Debug, Clone)]
pub struct GradientStop {
    pub position: PositiveFixedPercentage,
    pub color: Color,
}

impl GradientStop {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let pos_attr = xml_node
            .attribute("pos")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "pos"))?;
        let position = pos_attr.parse::<PositiveFixedPercentage>()?;

        let child_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "color"))?;
        if !Color::is_choice_member(child_node.local_name()) {
            return Err(NotGroupMemberError::new(child_node.name.clone(), "EG_Color").into());
        }

        let color = Color::from_xml_element(child_node)?;
        Ok(Self { position, color })
    }
}

#[derive(Default, Debug, Clone)]
pub struct LinearShadeProperties {
    /// Specifies the direction of color change for the gradient. To define this angle, let its value
    /// be x measured clockwise. Then ( -sin x, cos x ) is a vector parallel to the line of constant
    /// color in the gradient fill.
    pub angle: Option<PositiveFixedAngle>,
    /// Whether the gradient angle scales with the fill region. Mathematically, if this flag is true,
    /// then the gradient vector ( cos x , sin x ) is scaled by the width (w) and height (h) of the fill
    /// region, so that the vector becomes ( w cos x, h sin x ) (before normalization). Observe
    /// that now if the gradient angle is 45 degrees, the gradient vector is ( w, h ), which goes
    /// from top-left to bottom-right of the fill region. If this flag is false, the gradient angle is
    /// independent of the fill region and is not scaled using the manipulation described above.
    /// So a 45-degree gradient angle always give a gradient band whose line of constant color is
    /// parallel to the vector (1, -1).
    pub scaled: Option<bool>,
}

impl LinearShadeProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut angle = None;
        let mut scaled = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "ang" => angle = Some(value.parse::<PositiveFixedAngle>()?),
                "scaled" => scaled = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        Ok(Self { angle, scaled })
    }
}

#[derive(Default, Debug, Clone)]
pub struct PathShadeProperties {
    /// Specifies the shape of the path to follow.
    pub path: Option<PathShadeType>,
    /// This element defines the "focus" rectangle for the center shade, specified relative to the fill tile rectangle. The
    /// center shade fills the entire tile except the margins specified by each attribute.
    /// 
    /// Each edge of the center shade rectangle is defined by a percentage offset from the corresponding edge of the
    /// tile rectangle. A positive percentage specifies an inset, while a negative percentage specifies an outset.
    /// 
    /// # Note
    /// 
    /// For example, a left offset of 25% specifies that the left edge of the center shade rectangle is located to the right
    /// of the tile rectangle's left edge by an amount equal to 25% of the tile rectangle's width.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:path path="rect">
    ///   <a:fillToRect l="50000" r="50000" t="50000" b="50000"/>
    /// </a:path>
    /// ```
    /// 
    /// In the above shape, the rectangle defined by fillToRect is a single point in the center of the shape. This creates
    /// the effect of the center shade focusing at a point in the center of the region.
    pub fill_to_rect: Option<RelativeRect>,
}

impl PathShadeProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let path = match xml_node.attribute("path") {
            Some(val) => Some(val.parse()?),
            None => None,
        };

        let fill_to_rect = match xml_node.child_nodes.get(0) {
            Some(node) => Some(RelativeRect::from_xml_element(node)?),
            None => None,
        };

        Ok(Self { path, fill_to_rect })
    }
}

#[derive(Debug, Clone)]
pub enum ShadeProperties {
    /// This element specifies a linear gradient.
    Linear(LinearShadeProperties),
    /// This element defines that a gradient fill follows a path vs. a linear line.
    Path(PathShadeProperties),
}

impl ShadeProperties {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "lin" | "path" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "lin" => Ok(ShadeProperties::Linear(LinearShadeProperties::from_xml_element(
                xml_node,
            )?)),
            "path" => Ok(ShadeProperties::Path(PathShadeProperties::from_xml_element(xml_node)?)),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_ShadeProperties").into()),
        }
    }
}

/// This element defines a gradient fill.
/// 
/// A gradient fill is a fill which is characterized by a smooth gradual transition from one color to the next. At its
/// simplest, it is a fill which transitions between two colors; or more generally, it can be a transition of any number
/// of colors.
/// 
/// The desired transition colors and locations are defined in the gradient stop list (gsLst) child element.
/// 
/// The other child element defines the properties of the gradient fill (there are two styles-- a linear shade style as
/// well as a path shade style)
#[derive(Default, Debug, Clone)]
pub struct GradientFillProperties {
    /// Specifies the direction(s) in which to flip the gradient while tiling.
    /// 
    /// Normally a gradient fill encompasses the entire bounding box of the shape which
    /// contains the fill. However, with the tileRect element, it is possible to define a "tile"
    /// rectangle which is smaller than the bounding box. In this situation, the gradient fill is
    /// encompassed within the tile rectangle, and the tile rectangle is tiled across the bounding
    /// box to fill the entire area.
    pub flip: Option<TileFlipMode>,
    /// Specifies if a fill rotates along with a shape when the shape is rotated.
    pub rotate_with_shape: Option<bool>,
    /// The list of gradient stops that specifies the gradient colors and their relative positions in the color band.
    pub gradient_stop_list: Option<Vec<GradientStop>>,
    pub shade_properties: Option<ShadeProperties>,
    /// This element specifies a rectangular region of the shape to which the gradient is applied. This region is then
    /// tiled across the remaining area of the shape to complete the fill. The tile rectangle is defined by percentage
    /// offsets from the sides of the shape's bounding box.
    /// 
    /// Each edge of the tile rectangle is defined by a percentage offset from the corresponding edge of the bounding
    /// box. A positive percentage specifies an inset, while a negative percentage specifies an outset.
    /// 
    /// # Note
    /// 
    /// For example, a left offset of 25% specifies that the left edge of the tile rectangle is located to the right of the
    /// bounding box's left edge by an amount equal to 25% of the bounding box's width.
    pub tile_rect: Option<RelativeRect>,
}

impl GradientFillProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut flip = None;
        let mut rotate_with_shape = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "flip" => flip = Some(value.parse::<TileFlipMode>()?),
                "rotWithShape" => rotate_with_shape = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        let mut gradient_stop_list = None;
        let mut shade_properties = None;
        let mut tile_rect = None;

        for child_node in &xml_node.child_nodes {
            let local_name = child_node.local_name();
            if ShadeProperties::is_choice_member(local_name) {
                shade_properties = Some(ShadeProperties::from_xml_element(child_node)?);
            } else {
                match child_node.local_name() {
                    "gsLst" => {
                        let mut vec = Vec::new();
                        for gs_node in &child_node.child_nodes {
                            vec.push(GradientStop::from_xml_element(gs_node)?);
                        }

                        if vec.len() < 2 {
                            return Err(Box::new(LimitViolationError::new(
                                xml_node.name.clone(), 
                                "gsLst",
                                Limit::Value(2),
                                Limit::Unbounded,
                                vec.len() as u32,
                            )));
                        }

                        gradient_stop_list = Some(vec);
                    }
                    "tileRect" => tile_rect = Some(RelativeRect::from_xml_element(child_node)?),
                    _ => (),
                }
            }
        }

        Ok(Self {
            flip,
            rotate_with_shape,
            gradient_stop_list,
            shade_properties,
            tile_rect,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct TileInfoProperties {
    /// Specifies additional horizontal offset after alignment.
    pub translate_x: Option<Coordinate>,
    /// Specifies additional vertical offset after alignment.
    pub translate_y: Option<Coordinate>,
    /// Specifies the amount to horizontally scale the srcRect.
    pub scale_x: Option<Percentage>,
    /// Specifies the amount to vertically scale the srcRect.
    pub scale_y: Option<Percentage>,
    /// Specifies the direction(s) in which to flip the source image while tiling. Images can be
    /// flipped horizontally, vertically, or in both directions to fill the entire region.
    pub flip_mode: Option<TileFlipMode>,
    /// Specifies where to align the first tile with respect to the shape. Alignment happens after
    /// the scaling, but before the additional offset.
    pub alignment: Option<RectAlignment>,
}

impl TileInfoProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "tx" => instance.translate_x = Some(value.parse()?),
                "ty" => instance.translate_y = Some(value.parse()?),
                "sx" => instance.scale_x = Some(value.parse()?),
                "sy" => instance.scale_y = Some(value.parse()?),
                "flip" => instance.flip_mode = Some(value.parse()?),
                "algn" => instance.alignment = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct StretchInfoProperties {
    /// This element specifies a fill rectangle. When stretching of an image is specified, a source rectangle, srcRect, is
    /// scaled to fit the specified fill rectangle.
    /// 
    /// Each edge of the fill rectangle is defined by a percentage offset from the corresponding edge of the shape's
    /// bounding box. A positive percentage specifies an inset, while a negative percentage specifies an outset.
    /// 
    /// # Note
    /// 
    /// For example, a left offset of 25% specifies that the left edge of the fill rectangle is located to the right of the
    /// bounding box's left edge by an amount equal to 25% of the bounding box's width.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:blipFill>
    ///   <a:blip r:embed="rId2"/>
    ///   <a:stretch>
    ///     <a:fillRect b="10000" r="25000"/>
    ///   </a:stretch>
    /// </a:blipFill>
    /// ```
    pub fill_rect: Option<RelativeRect>,
}

impl StretchInfoProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let fill_rect = match xml_node.child_nodes.get(0) {
            Some(rect_node) => Some(RelativeRect::from_xml_element(rect_node)?),
            None => None,
        };

        Ok(Self { fill_rect })
    }
}

#[derive(Debug, Clone)]
pub enum FillModeProperties {
    /// This element specifies that a BLIP should be tiled to fill the available space. This element defines a "tile"
    /// rectangle within the bounding box. The image is encompassed within the tile rectangle, and the tile rectangle is
    /// tiled across the bounding box to fill the entire area.
    Tile(Box<TileInfoProperties>),
    /// This element specifies that a BLIP should be stretched to fill the target rectangle. The other option is a tile where
    /// a BLIP is tiled to fill the available area.
    Stretch(Box<StretchInfoProperties>),
}

impl FillModeProperties {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "tile" | "stretch" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "tile" => Ok(FillModeProperties::Tile(Box::new(
                TileInfoProperties::from_xml_element(xml_node)?,
            ))),
            "stretch" => Ok(FillModeProperties::Stretch(Box::new(
                StretchInfoProperties::from_xml_element(xml_node)?,
            ))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_FillModeProperties").into()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlipFillProperties {
    /// Specifies the DPI (dots per inch) used to calculate the size of the blip. If not present or
    /// zero, the DPI in the blip is used.
    /// 
    /// # Note
    /// 
    /// This attribute is primarily used to keep track of the picture quality within a
    /// document. There are different levels of quality needed for print than on-screen viewing
    /// and thus a need to track this information.
    pub dpi: Option<u32>,
    /// Specifies that the fill should rotate with the shape. That is, when the shape that has been
    /// filled with a picture and the containing shape (say a rectangle) is transformed with a
    /// rotation then the fill is transformed with the same rotation.
    pub rotate_with_shape: Option<bool>,
    /// This element specifies the existence of an image (binary large image or picture) and contains a reference to the
    /// image data.
    pub blip: Option<Box<Blip>>,
    /// This element specifies the portion of the blip used for the fill.
    /// 
    /// Each edge of the source rectangle is defined by a percentage offset from the corresponding edge of the
    /// bounding box. A positive percentage specifies an inset, while a negative percentage specifies an outset.
    /// 
    /// # Note
    /// 
    /// For example, a left offset of 25% specifies that the left edge of the source rectangle is located to the right of the
    /// bounding box's left edge by an amount equal to 25% of the bounding box's width.
    pub source_rect: Option<RelativeRect>,
    pub fill_mode_properties: Option<FillModeProperties>,
}

impl BlipFillProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut dpi = None;
        let mut rotate_with_shape = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "dpi" => dpi = Some(value.parse()?),
                "rotWithShape" => rotate_with_shape = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        let mut blip = None;
        let mut source_rect = None;
        let mut fill_mode_properties = None;

        for child_node in &xml_node.child_nodes {
            let child_local_name = child_node.local_name();

            if FillModeProperties::is_choice_member(child_local_name) {
                fill_mode_properties = Some(FillModeProperties::from_xml_element(child_node)?);
            } else {
                match child_local_name {
                    "blip" => blip = Some(Box::new(Blip::from_xml_element(child_node)?)),
                    "srcRect" => source_rect = Some(RelativeRect::from_xml_element(child_node)?),
                    _ => (),
                }
            }
        }

        Ok(Self {
            dpi,
            rotate_with_shape,
            blip,
            source_rect,
            fill_mode_properties,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct PatternFillProperties {
    /// Specifies one of a set of preset patterns to fill the object.
    pub preset: Option<PresetPatternVal>,
    /// This element specifies the foreground color of a pattern fill.
    pub fg_color: Option<Color>,
    /// This element specifies the background color of a Pattern fill.
    pub bg_color: Option<Color>,
}

impl PatternFillProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        instance.preset = match xml_node.attribute("prst") {
            Some(val) => Some(val.parse()?),
            None => None,
        };

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "fgClr" => {
                    let fg_color_node = child_node
                        .child_nodes
                        .get(0)
                        .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "EG_Color"))?;
                    instance.fg_color = Some(Color::from_xml_element(fg_color_node)?);
                }
                "bgClr" => {
                    let bg_color_node = child_node
                        .child_nodes
                        .get(0)
                        .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "EG_Color"))?;
                    instance.bg_color = Some(Color::from_xml_element(bg_color_node)?);
                }
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub enum FillProperties {
    /// This element specifies that no fill is applied to the parent element.
    NoFill,
    /// This element specifies a solid color fill. The shape is filled entirely with the specified color.
    SolidFill(Color),
    GradientFill(Box<GradientFillProperties>),
    /// This element specifies the type of picture fill that the picture object has. Because a picture has a picture fill
    /// already by default, it is possible to have two fills specified for a picture object. An example of this is shown
    /// below.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:pic>
    ///   ...
    ///   <p:blipFill>
    ///     <a:blip r:embed="rId2"/>
    ///     <a:stretch>
    ///       <a:fillRect/>
    ///     </a:stretch>
    ///   </p:blipFill>
    ///   ...
    /// </p:pic>
    /// ```
    BlipFill(Box<BlipFillProperties>),
    /// This element specifies a pattern fill. A repeated pattern is used to fill the object.
    PatternFill(Box<PatternFillProperties>),
    /// This element specifies a group fill. When specified, this setting indicates that the parent element is part of a
    /// group and should inherit the fill properties of the group.
    GroupFill,
}

impl FillProperties {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "noFill" | "solidFill" | "gradFill" | "blipFill" | "pattFill" | "grpFill" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "noFill" => Ok(FillProperties::NoFill),
            "solidFill" => {
                let child_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "color"))?;
                Ok(FillProperties::SolidFill(Color::from_xml_element(&child_node)?))
            }
            "gradFill" => Ok(FillProperties::GradientFill(Box::new(
                GradientFillProperties::from_xml_element(xml_node)?,
            ))),
            "blipFill" => Ok(FillProperties::BlipFill(Box::new(
                BlipFillProperties::from_xml_element(xml_node)?,
            ))),
            "pattFill" => Ok(FillProperties::PatternFill(Box::new(
                PatternFillProperties::from_xml_element(xml_node)?,
            ))),
            "grpFill" => Ok(FillProperties::GroupFill),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_FillProperties").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineFillProperties {
    /// This element specifies that no fill is applied to the parent element.
    NoFill,
    /// This element specifies a solid color fill. The shape is filled entirely with the specified color.
    SolidFill(Color),
    GradientFill(Box<GradientFillProperties>),
    PatternFill(Box<PatternFillProperties>),
}

impl LineFillProperties {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "noFill" | "solidFill" | "gradFill" | "pattFill" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<LineFillProperties> {
        match xml_node.local_name() {
            "noFill" => Ok(LineFillProperties::NoFill),
            "solidFill" => {
                let child_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "color"))?;

                if !Color::is_choice_member(child_node.local_name()) {
                    return Err(NotGroupMemberError::new(child_node.name.clone(), "EG_Color").into());
                }

                Ok(LineFillProperties::SolidFill(Color::from_xml_element(child_node)?))
            }
            "gradFill" => Ok(LineFillProperties::GradientFill(Box::new(
                GradientFillProperties::from_xml_element(xml_node)?,
            ))),
            "pattFill" => Ok(LineFillProperties::PatternFill(Box::new(
                PatternFillProperties::from_xml_element(xml_node)?,
            ))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_LineFillProperties").into()),
        }
    }
}

/// This element specifies a dash stop primitive. Dashing schemes are built by specifying an ordered list of dash stop
/// primitive. A dash stop primitive consists of a dash and a space.
#[derive(Debug, Clone)]
pub struct DashStop {
    /// Specifies the length of the dash relative to the line width.
    pub dash_length: PositivePercentage,
    /// Specifies the length of the space relative to the line width.
    pub space_length: PositivePercentage,
}

impl DashStop {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<DashStop> {
        let mut opt_dash_length = None;
        let mut opt_space_length = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "d" => opt_dash_length = Some(value.parse::<PositivePercentage>()?),
                "sp" => opt_space_length = Some(value.parse::<PositivePercentage>()?),
                _ => (),
            }
        }

        let dash_length = opt_dash_length.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "d"))?;
        let space_length = opt_space_length.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "sp"))?;

        Ok(Self {
            dash_length,
            space_length,
        })
    }
}

#[derive(Debug, Clone)]
pub enum LineDashProperties {
    /// This element specifies that a preset line dashing scheme should be used.
    PresetDash(PresetLineDashVal),
    /// This element specifies a custom dashing scheme. It is a list of dash stop elements which represent building block
    /// atoms upon which the custom dashing scheme is built.
    CustomDash(Vec<DashStop>),
}

impl LineDashProperties {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "prstDash" | "custDash" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<LineDashProperties> {
        match xml_node.local_name() {
            "prstDash" => {
                let val_attr = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(LineDashProperties::PresetDash(val_attr.parse::<PresetLineDashVal>()?))
            }
            "custDash" => {
                let mut dash_vec = Vec::new();
                for child_node in &xml_node.child_nodes {
                    if child_node.local_name() == "ds" {
                        match DashStop::from_xml_element(child_node) {
                            Ok(val) => dash_vec.push(val),
                            Err(err) => error!("Failed to parse 'ds' element: {}", err),
                        }
                    }
                }

                Ok(LineDashProperties::CustomDash(dash_vec))
            }
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_LineDashProperties").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineJoinProperties {
    /// This element specifies that lines joined together have a round join.
    Round,
    /// This element specifies a Bevel Line Join.
    /// 
    /// A bevel joint specifies that an angle joint is used to connect lines.
    Bevel,
    /// This element specifies that a line join shall be mitered.
    /// 
    /// The value specifies the amount by which lines is extended to form a miter join - otherwise miter
    /// joins can extend infinitely far (for lines which are almost parallel).
    Miter(Option<PositivePercentage>),
}

impl LineJoinProperties {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "round" | "bevel" | "miter" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<LineJoinProperties> {
        match xml_node.local_name() {
            "round" => Ok(LineJoinProperties::Round),
            "bevel" => Ok(LineJoinProperties::Bevel),
            "miter" => {
                let lim = match xml_node.attribute("lim") {
                    Some(ref attr) => Some(attr.parse::<PositivePercentage>()?),
                    None => None,
                };
                Ok(LineJoinProperties::Miter(lim))
            }
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_LineJoinProperties").into()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct LineEndProperties {
    /// Specifies the line end decoration, such as a triangle or arrowhead.
    pub end_type: Option<LineEndType>,
    /// Specifies the line end width in relation to the line width.
    pub width: Option<LineEndWidth>,
    /// Specifies the line end length in relation to the line width.
    pub length: Option<LineEndLength>,
}

impl LineEndProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<LineEndProperties> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "type" => instance.end_type = Some(value.parse::<LineEndType>()?),
                "width" => instance.width = Some(value.parse::<LineEndWidth>()?),
                "length" => instance.length = Some(value.parse::<LineEndLength>()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

/// This element specifies an outline style that can be applied to a number of different objects such as shapes and
/// text. The line allows for the specifying of many different types of outlines including even line dashes and bevels.
#[derive(Default, Debug, Clone)]
pub struct LineProperties {
    /// Specifies the width to be used for the underline stroke. If this attribute is omitted, then a
    /// value of 0 is assumed.
    pub width: Option<LineWidth>,
    /// Specifies the ending caps that should be used for this line. If this attribute is omitted, than a value of
    /// square is assumed.
    /// 
    /// # Note
    /// 
    /// Examples of cap types are rounded, flat, etc.
    pub cap: Option<LineCap>,
    /// Specifies the compound line type to be used for the underline stroke. If this attribute is
    /// omitted, then a value of CompoundLine::Single is assumed.
    pub compound: Option<CompoundLine>,
    /// Specifies the alignment to be used for the underline stroke.
    pub pen_alignment: Option<PenAlignment>,
    pub fill_properties: Option<LineFillProperties>,
    pub dash_properties: Option<LineDashProperties>,
    pub join_properties: Option<LineJoinProperties>,
    /// This element specifies decorations which can be added to the head of a line.
    pub head_end: Option<LineEndProperties>,
    /// This element specifies decorations which can be added to the tail of a line.
    pub tail_end: Option<LineEndProperties>,
}

impl LineProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<LineProperties> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "w" => instance.width = Some(value.parse()?),
                "cap" => instance.cap = Some(value.parse()?),
                "cmpd" => instance.compound = Some(value.parse()?),
                "algn" => instance.pen_alignment = Some(value.parse()?),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            if LineFillProperties::is_choice_member(child_node.local_name()) {
                instance.fill_properties = Some(LineFillProperties::from_xml_element(child_node)?);
            } else if LineDashProperties::is_choice_member(child_node.local_name()) {
                instance.dash_properties = Some(LineDashProperties::from_xml_element(child_node)?);
            } else if LineJoinProperties::is_choice_member(child_node.local_name()) {
                instance.join_properties = Some(LineJoinProperties::from_xml_element(child_node)?);
            } else {
                match child_node.local_name() {
                    "headEnd" => instance.head_end = Some(LineEndProperties::from_xml_element(child_node)?),
                    "tailEnd" => instance.tail_end = Some(LineEndProperties::from_xml_element(child_node)?),
                    _ => (),
                }
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct RelativeRect {
    /// Specifies the left edge of the rectangle.
    pub left: Option<Percentage>,
    /// Specifies the top edge of the rectangle.
    pub top: Option<Percentage>,
    /// Specifies the right edge of the rectangle.
    pub right: Option<Percentage>,
    /// Specifies the bottom edge of the rectangle.
    pub bottom: Option<Percentage>,
}

impl RelativeRect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<RelativeRect> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "l" => instance.left = Some(value.parse::<Percentage>()?),
                "t" => instance.top = Some(value.parse::<Percentage>()?),
                "r" => instance.right = Some(value.parse::<Percentage>()?),
                "b" => instance.bottom = Some(value.parse::<Percentage>()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct Point2D {
    /// Specifies a coordinate on the x-axis. The origin point for this coordinate shall be specified
    /// by the parent XML element.
    pub x: Coordinate,
    /// Specifies a coordinate on the x-axis. The origin point for this coordinate shall be specified
    /// by the parent XML element.
    pub y: Coordinate,
}

impl Point2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut x = None;
        let mut y = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "x" => x = Some(value.parse()?),
                "y" => y = Some(value.parse()?),
                _ => (),
            }
        }

        let x = x.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "x"))?;
        let y = y.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "y"))?;

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct PositiveSize2D {
    /// Specifies the length of the extents rectangle in EMUs. This rectangle shall dictate the size
    /// of the object as displayed (the result of any scaling to the original object).
    pub width: PositiveCoordinate,
    /// Specifies the width of the extents rectangle in EMUs. This rectangle shall dictate the size
    /// of the object as displayed (the result of any scaling to the original object).
    pub height: PositiveCoordinate,
}

impl PositiveSize2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut opt_width = None;
        let mut opt_height = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "cx" => opt_width = Some(value.parse::<PositiveCoordinate>()?),
                "cy" => opt_height = Some(value.parse::<PositiveCoordinate>()?),
                _ => (),
            }
        }

        let width = opt_width.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "cx"))?;
        let height = opt_height.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "cy"))?;

        Ok(Self { width, height })
    }
}

#[derive(Debug, Clone)]
pub struct StyleMatrixReference {
    pub index: StyleMatrixColumnIndex,
    pub color: Option<Color>,
}

impl StyleMatrixReference {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let idx_attr = xml_node
            .attribute("idx")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "idx"))?;
        let index = idx_attr.parse()?;

        let color = match xml_node.child_nodes.get(0) {
            Some(node) => Some(Color::from_xml_element(node)?),
            None => None,
        };

        Ok(Self { index, color })
    }
}

/// This element specifies an Effect Container. It is a list of effects.
#[derive(Default, Debug, Clone)]
pub struct EffectContainer {
    /// Specifies the kind of container, either sibling or tree.
    pub container_type: Option<EffectContainerType>,
    /// Specifies an optional name for this list of effects, so that it can be referred to later. Shall
    /// be unique across all effect trees and effect containers.
    pub name: Option<String>,
    pub effects: Vec<Effect>,
}

impl EffectContainer {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<EffectContainer> {
        let mut container_type = None;
        let mut name = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "type" => container_type = Some(value.parse::<EffectContainerType>()?),
                "name" => name = Some(value.clone()),
                _ => (),
            }
        }

        let mut effects = Vec::new();
        for child_node in &xml_node.child_nodes {
            if Effect::is_choice_member(child_node.local_name()) {
                effects.push(Effect::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            container_type,
            name,
            effects,
        })
    }
}

/// This element represents an Alpha Bi-Level Effect.
/// 
/// Alpha (Opacity) values less than the threshold are changed to 0 (fully transparent) and alpha values greater than
/// or equal to the threshold are changed to 100% (fully opaque).
#[derive(Debug, Clone)]
pub struct AlphaBiLevelEffect {
    // Specifies the threshold value for the alpha bi-level effect.
    pub threshold: PositiveFixedPercentage,
}

impl AlphaBiLevelEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<AlphaBiLevelEffect> {
        let thresh_attr = xml_node
            .attribute("thresh")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "thresh"))?;
        let threshold = thresh_attr.parse::<PositiveFixedPercentage>()?;
        Ok(Self { threshold })
    }
}

/// This element represents an alpha inverse effect.
/// 
/// Alpha (opacity) values are inverted by subtracting from 100%.
#[derive(Default, Debug, Clone)]
pub struct AlphaInverseEffect {
    pub color: Option<Color>,
}

impl AlphaInverseEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<AlphaInverseEffect> {
        let color = match xml_node.child_nodes.get(0) {
            Some(child_node) => Some(Color::from_xml_element(child_node)?),
            None => None,
        };

        Ok(Self { color })
    }
}

/// This element represents an alpha modulate effect.
/// 
/// Effect alpha (opacity) values are multiplied by a fixed percentage. The effect container specifies an effect
/// containing alpha values to modulate.
#[derive(Debug, Clone)]
pub struct AlphaModulateEffect {
    pub container: EffectContainer,
}

impl AlphaModulateEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<AlphaModulateEffect> {
        let child_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "container"))?;

        let container = EffectContainer::from_xml_element(child_node)?;

        Ok(Self { container })
    }
}

/// This element represents an alpha modulate fixed effect.
/// 
/// Effect alpha (opacity) values are multiplied by a fixed percentage.
#[derive(Default, Debug, Clone)]
pub struct AlphaModulateFixedEffect {
    /// Specifies the percentage amount to scale the alpha.
    /// 
    /// Defaults to 100000
    pub amount: Option<PositivePercentage>,
}

impl AlphaModulateFixedEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let amount = match xml_node.attribute("amt") {
            Some(attr) => Some(attr.parse()?),
            None => None,
        };

        Ok(Self { amount })
    }
}

/// This element specifies an alpha outset/inset effect.
/// 
/// This is equivalent to an alpha ceiling, followed by alpha blur, followed by either an alpha ceiling (positive radius)
/// or alpha floor (negative radius).
#[derive(Default, Debug, Clone)]
pub struct AlphaOutsetEffect {
    /// Specifies the radius of outset/inset.
    pub radius: Option<Coordinate>,
}

impl AlphaOutsetEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let radius = match xml_node.attribute("rad") {
            Some(attr) => Some(attr.parse()?),
            None => None,
        };

        Ok(Self { radius })
    }
}

/// This element specifies an alpha replace effect.
/// 
/// Effect alpha (opacity) values are replaced by a fixed alpha.
#[derive(Debug, Clone)]
pub struct AlphaReplaceEffect {
    /// Specifies the new opacity value.
    pub alpha: PositiveFixedPercentage,
}

impl AlphaReplaceEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let a_attr = xml_node
            .attribute("a")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "a"))?;
        let alpha = a_attr.parse()?;

        Ok(Self { alpha })
    }
}

/// This element specifies a bi-level (black/white) effect. Input colors whose luminance is less than the specified
/// threshold value are changed to black. Input colors whose luminance are greater than or equal the specified
/// value are set to white. The alpha effect values are unaffected by this effect.
#[derive(Debug, Clone)]
pub struct BiLevelEffect {
    /// Specifies the luminance threshold for the Bi-Level effect. Values greater than or equal to
    /// the threshold are set to white. Values lesser than the threshold are set to black.
    pub threshold: PositiveFixedPercentage,
}

impl BiLevelEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let thresh_attr = xml_node
            .attribute("thresh")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "thresh"))?;
        let threshold = thresh_attr.parse()?;

        Ok(Self { threshold })
    }
}

/// This element specifies a blend of several effects. The container specifies the raw effects to blend while the blend
/// mode specifies how the effects are to be blended.
#[derive(Debug, Clone)]
pub struct BlendEffect {
    /// Specifies how to blend the two effects.
    pub blend: BlendMode,
    pub container: EffectContainer,
}

impl BlendEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let blend_attr = xml_node
            .attribute("blend")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "blend"))?;
        let blend = blend_attr.parse()?;

        let container_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "cont"))?;
        let container = EffectContainer::from_xml_element(container_node)?;

        Ok(Self { blend, container })
    }
}

/// This element specifies a blur effect that is applied to the entire shape, including its fill. All color channels,
/// including alpha, are affected.
#[derive(Default, Debug, Clone)]
pub struct BlurEffect {
    /// Specifies the radius of blur.
    /// 
    /// Defaults to 0
    pub radius: Option<PositiveCoordinate>,
    /// Specifies whether the bounds of the object should be grown as a result of the blurring.
    /// True indicates the bounds are grown while false indicates that they are not.
    /// 
    /// With grow set to false, the blur effect does not extend beyond the original bounds of the
    /// object
    /// 
    /// With grow set to true, the blur effect can extend beyond the original bounds of the
    /// object
    /// 
    /// Defaults to true
    pub grow: Option<bool>,
}

impl BlurEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut radius = None;
        let mut grow = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "rad" => radius = Some(value.parse()?),
                "grow" => grow = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        Ok(Self { radius, grow })
    }
}

/// This element specifies a Color Change Effect. Instances of clrFrom are replaced with instances of clrTo.
#[derive(Debug, Clone)]
pub struct ColorChangeEffect {
    /// Specifies whether alpha values are considered for the effect. Effect alpha values are
    /// considered if use_alpha is true, else they are ignored.
    /// 
    /// Defaults to true
    pub use_alpha: Option<bool>,
    /// This element specifies a solid color replacement value. All effect colors are changed to a fixed color. Alpha values
    /// are unaffected.
    pub color_from: Color,
    /// This element specifies the color which replaces the clrFrom in a clrChange effect. This is the "target" or "to"
    /// color in the color change effect.
    pub color_to: Color,
}

impl ColorChangeEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let use_alpha = match xml_node.attribute("useA") {
            Some(attr) => Some(parse_xml_bool(attr)?),
            None => None,
        };

        let mut color_from = None;
        let mut color_to = None;
        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "clrFrom" => {
                    let color_node = child_node
                        .child_nodes
                        .get(0)
                        .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "EG_Color"))?;
                    color_from = Some(Color::from_xml_element(color_node)?);
                }
                "clrTo" => {
                    let color_node = child_node
                        .child_nodes
                        .get(0)
                        .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "EG_Color"))?;
                    color_to = Some(Color::from_xml_element(color_node)?);
                }
                _ => (),
            }
        }

        let color_from = color_from.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "clrFrom"))?;
        let color_to = color_to.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "clrTo"))?;

        Ok(Self {
            use_alpha,
            color_from,
            color_to,
        })
    }
}

/// This element specifies a solid color replacement value. All effect colors are changed to a fixed color. Alpha values
/// are unaffected.
#[derive(Debug, Clone)]
pub struct ColorReplaceEffect {
    pub color: Color,
}

impl ColorReplaceEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let color_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_Color"))?;
        let color = Color::from_xml_element(color_node)?;

        Ok(Self { color })
    }
}

/// This element specifies a luminance effect. Brightness linearly shifts all colors closer to white or black.
/// Contrast scales all colors to be either closer or further apart.
#[derive(Default, Debug, Clone)]
pub struct LuminanceEffect {
    /// Specifies the percent to change the brightness.
    pub brightness: Option<FixedPercentage>,
    /// Specifies the percent to change the contrast.
    pub contrast: Option<FixedPercentage>,
}

impl LuminanceEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut brightness = None;
        let mut contrast = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "bright" => brightness = Some(value.parse()?),
                "contrast" => contrast = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(Self { brightness, contrast })
    }
}

/// This element specifies a duotone effect.
/// 
/// For each pixel, combines clr1 and clr2 through a linear interpolation to determine the new color for that pixel.
#[derive(Debug, Clone)]
pub struct DuotoneEffect {
    pub colors: [Color; 2],
}

impl DuotoneEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let color_1_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_Color"))?;
        let color_2_node = xml_node
            .child_nodes
            .get(1)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_Color"))?;

        let color_1 = Color::from_xml_element(color_1_node)?;
        let color_2 = Color::from_xml_element(color_2_node)?;

        Ok(Self {
            colors: [color_1, color_2],
        })
    }
}

/// This element specifies a fill which is one of blipFill, gradFill, grpFill, noFill, pattFill or solidFill.
#[derive(Debug, Clone)]
pub struct FillEffect {
    pub fill_properties: FillProperties,
}

impl FillEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let fill_properties_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_FillProperties"))?;
        let fill_properties = FillProperties::from_xml_element(fill_properties_node)?;

        Ok(Self { fill_properties })
    }
}

/// This element specifies a fill overlay effect. A fill overlay can be used to specify an additional fill for an object and
/// blend the two fills together.
#[derive(Debug, Clone)]
pub struct FillOverlayEffect {
    /// Specifies how to blend the fill with the base effect.
    pub blend_mode: BlendMode,
    pub fill: FillProperties,
}

impl FillOverlayEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let blend_mode_attr = xml_node
            .attribute("blend")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "blend"))?;
        let blend_mode = blend_mode_attr.parse()?;

        let fill_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_FillProperties"))?;
        let fill = FillProperties::from_xml_element(fill_node)?;

        Ok(Self { blend_mode, fill })
    }
}

/// This element specifies a glow effect, in which a color blurred outline is added outside the edges of the object.
#[derive(Debug, Clone)]
pub struct GlowEffect {
    /// Specifies the radius of the glow.
    /// 
    /// Defaults to 0
    pub radius: Option<PositiveCoordinate>,
    pub color: Color,
}

impl GlowEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let radius = match xml_node.attribute("rad") {
            Some(attr) => Some(attr.parse()?),
            None => None,
        };

        let color_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_ColorChoice"))?;
        let color = Color::from_xml_element(color_node)?;

        Ok(Self { radius, color })
    }
}

/// This element specifies a hue/saturation/luminance effect. The hue, saturation, and luminance can each be
/// adjusted relative to its current value.
#[derive(Default, Debug, Clone)]
pub struct HslEffect {
    /// Specifies the number of degrees by which the hue is adjusted.
    /// 
    /// Defaults to 0
    pub hue: Option<PositiveFixedAngle>,
    /// Specifies the percentage by which the saturation is adjusted.
    /// 
    /// Defaults to 0
    pub saturation: Option<FixedPercentage>,
    /// Specifies the percentage by which the luminance is adjusted.
    /// 
    /// Defaults to 0
    pub luminance: Option<FixedPercentage>,
}

impl HslEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "hue" => instance.hue = Some(value.parse()?),
                "sat" => instance.saturation = Some(value.parse()?),
                "lum" => instance.luminance = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

/// This element specifies an inner shadow effect. A shadow is applied within the edges of the object according to
/// the parameters given by the attributes.
#[derive(Debug, Clone)]
pub struct InnerShadowEffect {
    /// Specifies the blur radius.
    /// 
    /// Defaults to 0
    pub blur_radius: Option<PositiveCoordinate>,
    /// Specifies how far to offset the shadow.
    /// 
    /// Defaults to 0
    pub distance: Option<PositiveCoordinate>,
    /// Specifies the direction to offset the shadow.
    /// 
    /// Defaults to 0
    pub direction: Option<PositiveFixedAngle>,
    pub color: Color,
}

impl InnerShadowEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut blur_radius = None;
        let mut distance = None;
        let mut direction = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "blurRad" => blur_radius = Some(value.parse()?),
                "dist" => distance = Some(value.parse()?),
                "dir" => direction = Some(value.parse()?),
                _ => (),
            }
        }

        let color_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_ColorChoice"))?;
        let color = Color::from_xml_element(color_node)?;

        Ok(Self {
            blur_radius,
            distance,
            direction,
            color,
        })
    }
}

/// This element specifies an Outer Shadow Effect.
#[derive(Debug, Clone)]
pub struct OuterShadowEffect {
    /// Specifies the blur radius of the shadow.
    /// 
    /// Defaults to 0
    pub blur_radius: Option<PositiveCoordinate>,
    /// Specifies the how far to offset the shadow.
    /// 
    /// Defaults to 0
    pub distance: Option<PositiveCoordinate>,
    /// Specifies the direction to offset the shadow.
    /// 
    /// Defaults to 0
    pub direction: Option<PositiveFixedAngle>,
    /// Specifies the horizontal scaling factor; negative scaling causes a flip.
    /// 
    /// Defaults to 100_000
    pub scale_x: Option<Percentage>,
    /// Specifies the vertical scaling factor; negative scaling causes a flip.
    /// 
    /// Defaults to 100_000
    pub scale_y: Option<Percentage>,
    /// Specifies the horizontal skew angle.
    /// 
    /// Defaults to 0
    pub skew_x: Option<FixedAngle>,
    /// Specifies the vertical skew angle.
    /// 
    /// Defaults to 0
    pub skew_y: Option<FixedAngle>,
    /// Specifies shadow alignment; alignment happens first, effectively setting the origin for
    /// scale, skew, and offset.
    /// 
    /// Defaults to RectAlignment::Bottom
    pub alignment: Option<RectAlignment>,
    /// Specifies whether the shadow rotates with the shape if the shape is rotated.
    /// 
    /// Defaults to true
    pub rotate_with_shape: Option<bool>,
    pub color: Color,
}

impl OuterShadowEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut blur_radius = None;
        let mut distance = None;
        let mut direction = None;
        let mut scale_x = None;
        let mut scale_y = None;
        let mut skew_x = None;
        let mut skew_y = None;
        let mut alignment = None;
        let mut rotate_with_shape = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "blurRad" => blur_radius = Some(value.parse()?),
                "dist" => distance = Some(value.parse()?),
                "dir" => direction = Some(value.parse()?),
                "sx" => scale_x = Some(value.parse()?),
                "sy" => scale_y = Some(value.parse()?),
                "kx" => skew_x = Some(value.parse()?),
                "ky" => skew_y = Some(value.parse()?),
                "algn" => alignment = Some(value.parse()?),
                "rotWithShape" => rotate_with_shape = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        let color_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_ColorChoice"))?;
        let color = Color::from_xml_element(color_node)?;

        Ok(Self {
            blur_radius,
            distance,
            direction,
            scale_x,
            scale_y,
            skew_x,
            skew_y,
            alignment,
            rotate_with_shape,
            color,
        })
    }
}

/// This element specifies that a preset shadow is to be used. Each preset shadow is equivalent to a specific outer
/// shadow effect. For each preset shadow, the color element, direction attribute, and distance attribute represent
/// the color, direction, and distance parameters of the corresponding outer shadow. Additionally, the
/// rotateWithShape attribute of corresponding outer shadow is always false. Other non-default parameters of
/// the outer shadow are dependent on the prst attribute.
#[derive(Debug, Clone)]
pub struct PresetShadowEffect {
    /// Specifies which preset shadow to use.
    pub preset: PresetShadowVal,
    /// Specifies how far to offset the shadow.
    /// 
    /// Defaults to 0
    pub distance: Option<PositiveCoordinate>,
    /// Specifies the direction to offset the shadow.
    /// 
    /// Defaults to 0
    pub direction: Option<PositiveFixedAngle>,
    pub color: Color,
}

impl PresetShadowEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut preset = None;
        let mut distance = None;
        let mut direction = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "prst" => preset = Some(value.parse()?),
                "dist" => distance = Some(value.parse()?),
                "dir" => direction = Some(value.parse()?),
                _ => (),
            }
        }

        let color_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_ColorChoice"))?;
        let color = Color::from_xml_element(color_node)?;

        let preset = preset.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "prst"))?;

        Ok(Self {
            preset,
            distance,
            direction,
            color,
        })
    }
}

/// This element specifies a reflection effect.
#[derive(Default, Debug, Clone)]
pub struct ReflectionEffect {
    /// Specifies the blur radius.
    /// 
    /// Defaults to 0
    pub blur_radius: Option<PositiveCoordinate>,
    /// Starting reflection opacity.
    /// 
    /// Defaults to 100_000
    pub start_opacity: Option<PositiveFixedPercentage>,
    /// Specifies the start position (along the alpha gradient ramp) of the start alpha value.
    /// 
    /// Defaults to 0
    pub start_position: Option<PositiveFixedPercentage>,
    /// Specifies the ending reflection opacity.
    /// 
    /// Defaults to 0
    pub end_opacity: Option<PositiveFixedPercentage>,
    /// Specifies the end position (along the alpha gradient ramp) of the end alpha value.
    /// 
    /// Defaults to 100_000
    pub end_position: Option<PositiveFixedPercentage>,
    /// Specifies how far to distance the shadow.
    /// 
    /// Defaults to 0
    pub distance: Option<PositiveCoordinate>,
    /// Specifies the direction of the alpha gradient ramp relative to the shape itself.
    /// 
    /// Defaults to 0
    pub direction: Option<PositiveFixedAngle>,
    /// Specifies the direction to offset the reflection.
    /// 
    /// Defaults to 5_400_000
    pub fade_direction: Option<PositiveFixedAngle>,
    /// Specifies the horizontal scaling factor.
    /// 
    /// Defaults to 100_000
    pub scale_x: Option<Percentage>,
    /// Specifies the vertical scaling factor.
    /// 
    /// Defaults to 100_000
    pub scale_y: Option<Percentage>,
    /// Specifies the horizontal skew angle.
    /// 
    /// Defaults to 0
    pub skew_x: Option<FixedAngle>,
    /// Specifies the vertical skew angle.
    /// 
    /// Defaults to 0
    pub skew_y: Option<FixedAngle>,
    /// Specifies shadow alignment.
    /// 
    /// Defaults to RectAlignment::Bottom
    pub alignment: Option<RectAlignment>,
    /// Specifies if the reflection rotates with the shape.
    /// 
    /// Defaults to true
    pub rotate_with_shape: Option<bool>,
}

impl ReflectionEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "blurRad" => instance.blur_radius = Some(value.parse()?),
                "stA" => instance.start_opacity = Some(value.parse()?),
                "stPos" => instance.start_position = Some(value.parse()?),
                "endA" => instance.end_opacity = Some(value.parse()?),
                "endPos" => instance.end_position = Some(value.parse()?),
                "dist" => instance.distance = Some(value.parse()?),
                "dir" => instance.direction = Some(value.parse()?),
                "fadeDir" => instance.fade_direction = Some(value.parse()?),
                "sx" => instance.scale_x = Some(value.parse()?),
                "sy" => instance.scale_y = Some(value.parse()?),
                "kx" => instance.skew_x = Some(value.parse()?),
                "ky" => instance.skew_y = Some(value.parse()?),
                "algn" => instance.alignment = Some(value.parse()?),
                "rotWithShape" => instance.rotate_with_shape = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

/// This element specifies a relative offset effect. Sets up a new origin by offsetting relative to the size of the
/// previous effect.
#[derive(Default, Debug, Clone)]
pub struct RelativeOffsetEffect {
    /// Specifies the X offset.
    /// 
    /// Defaults to 0
    pub translate_x: Option<Percentage>,
    /// Specifies the Y offset.
    /// 
    /// Defaults to 0
    pub translate_y: Option<Percentage>,
}

impl RelativeOffsetEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut translate_x = None;
        let mut translate_y = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "tx" => translate_x = Some(value.parse()?),
                "ty" => translate_y = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(Self {
            translate_x,
            translate_y,
        })
    }
}

/// This element specifies a soft edge effect. The edges of the shape are blurred, while the fill is not affected.
#[derive(Debug, Clone)]
pub struct SoftEdgesEffect {
    /// Specifies the radius of blur to apply to the edges.
    pub radius: PositiveCoordinate,
}

impl SoftEdgesEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let radius_attr = xml_node
            .attribute("rad")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "rad"))?;

        let radius = radius_attr.parse()?;

        Ok(Self { radius })
    }
}

/// This element specifies a tint effect. Shifts effect color values towards/away from hue by the specified amount.
#[derive(Default, Debug, Clone)]
pub struct TintEffect {
    /// Specifies the hue towards which to tint.
    /// 
    /// Defaults to 0
    pub hue: Option<PositiveFixedAngle>,
    /// Specifies by how much the color value is shifted.
    /// 
    /// Defaults to 0
    pub amount: Option<FixedPercentage>,
}

impl TintEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut hue = None;
        let mut amount = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "hue" => hue = Some(value.parse()?),
                "amt" => amount = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(Self { hue, amount })
    }
}

/// This element specifies a transform effect. The transform is applied to each point in the shape's geometry using
/// the following matrix:
/// 
/// sx          tan(kx)     tx      x
/// tan(ky)     sy          ty  *   y
/// 0           0           1       1
#[derive(Default, Debug, Clone)]
pub struct TransformEffect {
    /// Specifies a percentage by which to horizontally scale the object.
    /// 
    /// Defaults to 100_000
    pub scale_x: Option<Percentage>,
    /// Specifies a percentage by which to vertically scale the object.
    /// 
    /// Defaults to 100_000
    pub scale_y: Option<Percentage>,
    /// Specifies an amount by which to shift the object along the x-axis.
    /// 
    /// Defaults to 0
    pub translate_x: Option<Coordinate>,
    /// Specifies an amount by which to shift the object along the y-axis.
    /// 
    /// Defaults to 0
    pub translate_y: Option<Coordinate>,
    /// Specifies the horizontal skew angle, defined as the angle between the top-left corner and
    /// bottom-left corner of the object's original bounding box. If positive, the bottom edge of
    /// the shape is positioned to the right relative to the top edge.
    /// 
    /// Defaults to 0
    pub skew_x: Option<FixedAngle>,
    /// Specifies the vertical skew angle, defined as the angle between the top-left corner and
    /// top-right corner of the object's original bounding box. If positive, the right edge of the
    /// object is positioned lower relative to the left edge.
    /// 
    /// Defaults to 0
    pub skew_y: Option<FixedAngle>,
}

impl TransformEffect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "sx" => instance.scale_x = Some(value.parse()?),
                "sy" => instance.scale_y = Some(value.parse()?),
                "kx" => instance.skew_x = Some(value.parse()?),
                "ky" => instance.skew_y = Some(value.parse()?),
                "tx" => instance.translate_x = Some(value.parse()?),
                "ty" => instance.translate_y = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

// TODO: maybe Box ReflectionEffect variant (sizeof==120)
#[derive(Debug, Clone)]
pub enum Effect {
    Container(EffectContainer),
    /// This element specifies a reference to an existing effect container.
    /// 
    /// Its value can be the name of an effect container, or one of four
    /// special references:
    /// * fill - refers to the fill effect
    /// * line - refers to the line effect
    /// * fillLine - refers to the combined fill and line effects
    /// * children - refers to the combined effects from logical child shapes or text
    EffectReference(String),
    AlphaBiLevel(AlphaBiLevelEffect),
    /// This element represents an alpha ceiling effect.
    ///
    /// Alpha (opacity) values greater than zero are changed to 100%. In other words, anything partially opaque
    /// becomes fully opaque.
    AlphaCeiling,
    /// This element represents an alpha floor effect.
    /// 
    /// Alpha (opacity) values less than 100% are changed to zero. In other words, anything partially transparent
    /// becomes fully transparent.
    AlphaFloor,
    AlphaInverse(AlphaInverseEffect),
    AlphaModulate(AlphaModulateEffect),
    AlphaModulateFixed(AlphaModulateFixedEffect),
    AlphaOutset(AlphaOutsetEffect),
    AlphaReplace(AlphaReplaceEffect),
    BiLevel(BiLevelEffect),
    Blend(BlendEffect),
    Blur(BlurEffect),
    ColorChange(ColorChangeEffect),
    ColorReplace(ColorReplaceEffect),
    Duotone(DuotoneEffect),
    Fill(FillEffect),
    FillOverlay(FillOverlayEffect),
    Glow(GlowEffect),
    /// This element specifies a gray scale effect. Converts all effect color values to a shade of gray, corresponding to
    /// their luminance. Effect alpha (opacity) values are unaffected.
    Grayscale,
    Hsl(HslEffect),
    InnerShadow(InnerShadowEffect),
    Luminance(LuminanceEffect),
    OuterShadow(OuterShadowEffect),
    PresetShadow(PresetShadowEffect),
    Reflection(ReflectionEffect),
    RelativeOffset(RelativeOffsetEffect),
    SoftEdges(SoftEdgesEffect),
    Tint(TintEffect),
    Transform(TransformEffect),
}

impl Effect {
    pub fn is_choice_member<T>(name: T) -> bool
    where
        T: AsRef<str>,
    {
        match name.as_ref() {
            "cont" | "effect" | "alphaBiLevel" | "alphaCeiling" | "alphaFloor" | "alphaInv" | "alphaMod"
            | "alphaModFix" | "alphaOutset" | "alphaRepl" | "biLevel" | "blend" | "blur" | "clrChange" | "clrRepl"
            | "duotone" | "fill" | "fillOverlay" | "glow" | "grayscl" | "hsl" | "innerShdw" | "lum" | "outerShdw"
            | "prstShdw" | "reflection" | "relOff" | "softEdge" | "tint" | "xfrm" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "cont" => Ok(Effect::Container(EffectContainer::from_xml_element(xml_node)?)),
            "effect" => {
                let ref_attr = xml_node
                    .attribute("ref")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "ref"))?;
                Ok(Effect::EffectReference(ref_attr.clone()))
            }
            "alphaBiLevel" => Ok(Effect::AlphaBiLevel(AlphaBiLevelEffect::from_xml_element(xml_node)?)),
            "alphaCeiling" => Ok(Effect::AlphaCeiling),
            "alphaFloor" => Ok(Effect::AlphaFloor),
            "alphaInv" => Ok(Effect::AlphaInverse(AlphaInverseEffect::from_xml_element(xml_node)?)),
            "alphaMod" => Ok(Effect::AlphaModulate(AlphaModulateEffect::from_xml_element(xml_node)?)),
            "alphaModFix" => Ok(Effect::AlphaModulateFixed(AlphaModulateFixedEffect::from_xml_element(
                xml_node,
            )?)),
            "alphaOutset" => Ok(Effect::AlphaOutset(AlphaOutsetEffect::from_xml_element(xml_node)?)),
            "alphaRepl" => Ok(Effect::AlphaReplace(AlphaReplaceEffect::from_xml_element(xml_node)?)),
            "biLevel" => Ok(Effect::BiLevel(BiLevelEffect::from_xml_element(xml_node)?)),
            "blend" => Ok(Effect::Blend(BlendEffect::from_xml_element(xml_node)?)),
            "blur" => Ok(Effect::Blur(BlurEffect::from_xml_element(xml_node)?)),
            "clrChange" => Ok(Effect::ColorChange(ColorChangeEffect::from_xml_element(xml_node)?)),
            "clrRepl" => Ok(Effect::ColorReplace(ColorReplaceEffect::from_xml_element(xml_node)?)),
            "duotone" => Ok(Effect::Duotone(DuotoneEffect::from_xml_element(xml_node)?)),
            "fill" => Ok(Effect::Fill(FillEffect::from_xml_element(xml_node)?)),
            "fillOverlay" => Ok(Effect::FillOverlay(FillOverlayEffect::from_xml_element(xml_node)?)),
            "glow" => Ok(Effect::Glow(GlowEffect::from_xml_element(xml_node)?)),
            "grayscl" => Ok(Effect::Grayscale),
            "hsl" => Ok(Effect::Hsl(HslEffect::from_xml_element(xml_node)?)),
            "innerShdw" => Ok(Effect::InnerShadow(InnerShadowEffect::from_xml_element(xml_node)?)),
            "lum" => Ok(Effect::Luminance(LuminanceEffect::from_xml_element(xml_node)?)),
            "outerShdw" => Ok(Effect::OuterShadow(OuterShadowEffect::from_xml_element(xml_node)?)),
            "prstShdw" => Ok(Effect::PresetShadow(PresetShadowEffect::from_xml_element(xml_node)?)),
            "reflection" => Ok(Effect::Reflection(ReflectionEffect::from_xml_element(xml_node)?)),
            "relOff" => Ok(Effect::RelativeOffset(RelativeOffsetEffect::from_xml_element(
                xml_node,
            )?)),
            "softEdge" => Ok(Effect::SoftEdges(SoftEdgesEffect::from_xml_element(xml_node)?)),
            "tint" => Ok(Effect::Tint(TintEffect::from_xml_element(xml_node)?)),
            "xfrm" => Ok(Effect::Transform(TransformEffect::from_xml_element(xml_node)?)),
            _ => Err(Box::new(NotGroupMemberError::new(xml_node.name.clone(), "EG_Effect"))),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct EffectList {
    pub blur: Option<BlurEffect>,
    pub fill_overlay: Option<FillOverlayEffect>,
    pub glow: Option<GlowEffect>,
    pub inner_shadow: Option<InnerShadowEffect>,
    pub outer_shadow: Option<OuterShadowEffect>,
    pub preset_shadow: Option<PresetShadowEffect>,
    pub reflection: Option<ReflectionEffect>,
    pub soft_edges: Option<SoftEdgesEffect>,
}

impl EffectList {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        trace!("parsing EffectList '{}'", xml_node.name);
        let mut instance: Self = Default::default();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "blur" => instance.blur = Some(BlurEffect::from_xml_element(child_node)?),
                "fillOverlay" => instance.fill_overlay = Some(FillOverlayEffect::from_xml_element(child_node)?),
                "glow" => instance.glow = Some(GlowEffect::from_xml_element(child_node)?),
                "innerShdw" => instance.inner_shadow = Some(InnerShadowEffect::from_xml_element(child_node)?),
                "outerShdw" => instance.outer_shadow = Some(OuterShadowEffect::from_xml_element(child_node)?),
                "prstShdw" => instance.preset_shadow = Some(PresetShadowEffect::from_xml_element(child_node)?),
                "reflection" => instance.reflection = Some(ReflectionEffect::from_xml_element(child_node)?),
                "softEdge" => instance.soft_edges = Some(SoftEdgesEffect::from_xml_element(child_node)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub enum EffectProperties {
    /// This element specifies a list of effects. Effects in an effectLst are applied in the default order by the rendering
    /// engine. The following diagrams illustrate the order in which effects are applied, both for shapes and for group
    /// shapes.
    /// 
    /// # Note
    /// 
    /// The output of many effects does not include the input shape. For effects that should be applied to the
    /// result of previous effects as well as the original shape, a container is used to group the inputs together.
    /// 
    /// # Example
    /// 
    /// Outer Shadow is applied both to the original shape and the original shape's glow. The result of blur
    /// contains the original shape, while the result of glow contains only the added glow. Therefore, a container that
    /// groups the blur result with the glow result is used as the input to Outer Shadow.
    EffectList(Box<EffectList>),
    /// This element specifies a list of effects. Effects are applied in the order specified by the container type (sibling or
    /// tree).
    /// 
    /// # Note
    /// 
    /// An effectDag element can contain multiple effect containers as child elements. Effect containers with
    /// different styles can be combined in an effectDag to define a directed acyclic graph (DAG) that specifies the order
    /// in which all effects are applied.
    EffectContainer(Box<EffectContainer>),
}

impl EffectProperties {
    pub fn is_choice_member<T>(name: T) -> bool
    where
        T: AsRef<str>,
    {
        match name.as_ref() {
            "effectLst" | "effectDag" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "effectLst" => Ok(EffectProperties::EffectList(Box::new(EffectList::from_xml_element(
                xml_node,
            )?))),
            "effectDag" => Ok(EffectProperties::EffectContainer(Box::new(
                EffectContainer::from_xml_element(xml_node)?,
            ))),
            _ => Err(Box::new(NotGroupMemberError::new(
                xml_node.name.clone(),
                "EG_EffectProperties",
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EffectStyleItem {
    pub effect_props: EffectProperties,
    // TODO implement
    //pub scene_3d: Option<Scene3D>,
    //pub shape_3d: Option<Shape3D>,
}

impl EffectStyleItem {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        trace!("parsing EffectStyleItem '{}'", xml_node.name);
        let mut effect_props = None;

        for child_node in &xml_node.child_nodes {
            let child_local_name = child_node.local_name();
            if EffectProperties::is_choice_member(child_local_name) {
                effect_props = Some(EffectProperties::from_xml_element(child_node)?);
            }
        }

        let effect_props =
            effect_props.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_EffectProperties"))?;

        Ok(Self { effect_props })
    }
}

#[derive(Debug, Clone)]
pub enum BlipEffect {
    AlphaBiLevel(AlphaBiLevelEffect),
    /// This element represents an alpha ceiling effect.
    ///
    /// Alpha (opacity) values greater than zero are changed to 100%. In other words, anything partially opaque
    /// becomes fully opaque.
    AlphaCeiling,
    /// This element represents an alpha floor effect.
    /// 
    /// Alpha (opacity) values less than 100% are changed to zero. In other words, anything partially transparent
    /// becomes fully transparent.
    AlphaFloor,
    AlphaInverse(AlphaInverseEffect),
    AlphaModulate(AlphaModulateEffect),
    AlphaModulateFixed(AlphaModulateFixedEffect),
    AlphaReplace(AlphaReplaceEffect),
    BiLevel(BiLevelEffect),
    Blur(BlurEffect),
    ColorChange(ColorChangeEffect),
    ColorReplace(ColorReplaceEffect),
    Duotone(DuotoneEffect),
    FillOverlay(FillOverlayEffect),
    /// This element specifies a gray scale effect. Converts all effect color values to a shade of gray, corresponding to
    /// their luminance. Effect alpha (opacity) values are unaffected.
    Grayscale,
    Hsl(HslEffect),
    Luminance(LuminanceEffect),
    Tint(TintEffect),
}

impl BlipEffect {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "alphaBiLevel" | "alphaCeiling" | "alphaFloor" | "alphaInv" | "alphaMod" | "alphaModFixed"
            | "alphaRepl" | "biLevel" | "blur" | "clrChange" | "clrRepl" | "duotone" | "fillOverlay" | "grayscl"
            | "hsl" | "lum" | "tint" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<BlipEffect> {
        match xml_node.local_name() {
            "alphaBiLevel" => Ok(BlipEffect::AlphaBiLevel(AlphaBiLevelEffect::from_xml_element(
                xml_node,
            )?)),
            "alphaCeiling" => Ok(BlipEffect::AlphaCeiling),
            "alphaFloor" => Ok(BlipEffect::AlphaFloor),
            "alphaInv" => Ok(BlipEffect::AlphaInverse(AlphaInverseEffect::from_xml_element(
                xml_node,
            )?)),
            "alphaMod" => Ok(BlipEffect::AlphaModulate(AlphaModulateEffect::from_xml_element(
                xml_node,
            )?)),
            "alphaModFixed" => Ok(BlipEffect::AlphaModulateFixed(
                AlphaModulateFixedEffect::from_xml_element(xml_node)?,
            )),
            "alphaRepl" => Ok(BlipEffect::AlphaReplace(AlphaReplaceEffect::from_xml_element(
                xml_node,
            )?)),
            "biLevel" => Ok(BlipEffect::BiLevel(BiLevelEffect::from_xml_element(xml_node)?)),
            "blur" => Ok(BlipEffect::Blur(BlurEffect::from_xml_element(xml_node)?)),
            "clrChange" => Ok(BlipEffect::ColorChange(ColorChangeEffect::from_xml_element(xml_node)?)),
            "clrRepl" => Ok(BlipEffect::ColorReplace(ColorReplaceEffect::from_xml_element(
                xml_node,
            )?)),
            "duotone" => Ok(BlipEffect::Duotone(DuotoneEffect::from_xml_element(xml_node)?)),
            "fillOverlay" => Ok(BlipEffect::FillOverlay(FillOverlayEffect::from_xml_element(xml_node)?)),
            "grayscl" => Ok(BlipEffect::Grayscale),
            "hsl" => Ok(BlipEffect::Hsl(HslEffect::from_xml_element(xml_node)?)),
            "lum" => Ok(BlipEffect::Luminance(LuminanceEffect::from_xml_element(xml_node)?)),
            "tint" => Ok(BlipEffect::Tint(TintEffect::from_xml_element(xml_node)?)),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_BlipEffect").into()),
        }
    }
}

/// Blip
#[derive(Default, Debug, Clone)]
pub struct Blip {
    /// Specifies the identification information for an embedded picture. This attribute is used to
    /// specify an image that resides locally within the file.
    pub embed_rel_id: Option<RelationshipId>,
    /// Specifies the identification information for a linked picture. This attribute is used to
    /// specify an image that does not reside within this file.
    pub linked_rel_id: Option<RelationshipId>,
    /// Specifies the compression state with which the picture is stored. This allows the
    /// application to specify the amount of compression that has been applied to a picture.
    pub compression: Option<BlipCompression>,
    pub effects: Vec<BlipEffect>,
}

impl Blip {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut embed_rel_id = None;
        let mut linked_rel_id = None;
        let mut compression = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "r:embed" => embed_rel_id = Some(value.clone()),
                "r:link" => linked_rel_id = Some(value.clone()),
                "cstate" => compression = Some(value.parse::<BlipCompression>()?),
                _ => (),
            }
        }

        let mut effects = Vec::new();

        for child_node in &xml_node.child_nodes {
            if BlipEffect::is_choice_member(child_node.local_name()) {
                effects.push(BlipEffect::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            embed_rel_id,
            linked_rel_id,
            compression,
            effects,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TextFont {
    pub typeface: TextTypeFace,
    pub panose: Option<Panose>,
    pub pitch_family: Option<i32>, // 0
    pub charset: Option<i32>,      // 1
}

impl TextFont {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextFont> {
        let mut typeface = None;
        let mut panose = None;
        let mut pitch_family = None;
        let mut charset = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "typeface" => typeface = Some(value.clone()),
                "panose" => panose = Some(value.clone()),
                "pitchFamily" => pitch_family = Some(value.parse::<i32>()?),
                "charset" => charset = Some(value.parse::<i32>()?),
                _ => (),
            }
        }

        let typeface = typeface.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "typeface"))?;

        Ok(Self {
            typeface,
            panose,
            pitch_family,
            charset,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SupplementalFont {
    /// Specifies the script, or language, in which the typeface is supposed to be used.
    /// 
    /// # Note
    /// 
    /// It is recommended that script names as specified in ISO 15924 are used.
    pub script: String,
    /// Specifies the font face to use.
    pub typeface: TextTypeFace,
}

impl SupplementalFont {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut script = None;
        let mut typeface = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "script" => script = Some(value.clone()),
                "typeface" => typeface = Some(value.clone()),
                _ => (),
            }
        }

        let script = script.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "script"))?;
        let typeface = typeface.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "typeface"))?;

        Ok(Self { script, typeface })
    }
}

#[derive(Debug, Clone)]
pub enum TextSpacing {
    Percent(TextSpacingPercent),
    Point(TextSpacingPoint),
}

impl TextSpacing {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextSpacing> {
        match xml_node.local_name() {
            "spcPct" => {
                let val_attr = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(TextSpacing::Percent(val_attr.parse::<TextSpacingPercent>()?))
            }
            "spcPts" => {
                let val_attr = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(TextSpacing::Point(val_attr.parse::<TextSpacingPoint>()?))
            }
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextSpacing").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextBulletColor {
    FollowText,
    Color(Color),
}

impl TextBulletColor {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "buClrTx" | "buClr" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextBulletColor> {
        match xml_node.local_name() {
            "buClrTx" => Ok(TextBulletColor::FollowText),
            "buClr" => {
                let child_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "color"))?;
                Ok(TextBulletColor::Color(Color::from_xml_element(child_node)?))
            }
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextBulletColor").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextBulletSize {
    FollowText,
    Percent(TextBulletSizePercent),
    Point(TextFontSize),
}

impl TextBulletSize {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "buSzTx" | "buSzPct" | "buSzPts" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextBulletSize> {
        match xml_node.local_name() {
            "buSzTx" => Ok(TextBulletSize::FollowText),
            "buSzPct" => {
                let val_attr = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(TextBulletSize::Percent(val_attr.parse::<TextBulletSizePercent>()?))
            }
            "buSzPts" => {
                let val_attr = xml_node
                    .attribute("val")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "val"))?;
                Ok(TextBulletSize::Point(val_attr.parse::<TextFontSize>()?))
            }
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextBulletSize").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextBulletTypeface {
    FollowText,
    Font(TextFont),
}

impl TextBulletTypeface {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "buFontTx" | "buFont" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextBulletTypeface> {
        match xml_node.local_name() {
            "buFontTx" => Ok(TextBulletTypeface::FollowText),
            "buFont" => Ok(TextBulletTypeface::Font(TextFont::from_xml_element(xml_node)?)),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextBulletTypeface").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextBullet {
    None,
    AutoNumbered(TextAutonumberedBullet),
    Character(String),
    Picture(Box<Blip>),
}

impl TextBullet {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "buNone" | "buAutoNum" | "buChar" | "buBlip" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextBullet> {
        match xml_node.local_name() {
            "buNone" => Ok(TextBullet::None),
            "buAutoNum" => Ok(TextBullet::AutoNumbered(TextAutonumberedBullet::from_xml_element(
                xml_node,
            )?)),
            "buChar" => {
                let char_attr = xml_node
                    .attribute("char")
                    .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "char"))?;
                Ok(TextBullet::Character(char_attr.clone()))
            }
            "buBlip" => match xml_node.child_nodes.get(0) {
                Some(child_node) => Ok(TextBullet::Picture(Box::new(Blip::from_xml_element(child_node)?))),
                None => Err(MissingChildNodeError::new(xml_node.name.clone(), "EG_TextBullet").into()),
            },
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextBullet").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextAutonumberedBullet {
    pub scheme: TextAutonumberScheme,
    pub start_at: Option<TextBulletStartAtNum>,
}

impl TextAutonumberedBullet {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextAutonumberedBullet> {
        let mut scheme = None;
        let mut start_at = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "type" => scheme = Some(value.parse::<TextAutonumberScheme>()?),
                "startAt" => start_at = Some(value.parse::<TextBulletStartAtNum>()?),
                _ => (),
            }
        }

        let scheme = scheme.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "type"))?;

        Ok(Self { scheme, start_at })
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextTabStop {
    pub position: Option<Coordinate32>,
    pub alignment: Option<TextTabAlignType>,
}

impl TextTabStop {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextTabStop> {
        let mut position = None;
        let mut alignment = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "pos" => position = Some(value.parse::<Coordinate32>()?),
                "algn" => alignment = Some(value.parse::<TextTabAlignType>()?),
                _ => (),
            }
        }

        Ok(Self { position, alignment })
    }
}

#[derive(Debug, Clone)]
pub enum TextUnderlineLine {
    FollowText,
    Line(Option<Box<LineProperties>>),
}

impl TextUnderlineLine {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "uLnTx" | "uLn" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "uLnTx" => Ok(TextUnderlineLine::FollowText),
            "uLn" => Ok(TextUnderlineLine::Line(match xml_node.child_nodes.get(0) {
                Some(node) => Some(Box::new(LineProperties::from_xml_element(node)?)),
                None => None,
            })),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextUnderlineLine").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextUnderlineFill {
    FollowText,
    Fill(FillProperties),
}

impl TextUnderlineFill {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "uFillTx" | "uFill" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "uFillTx" => Ok(TextUnderlineFill::FollowText),
            "uFill" => {
                let fill_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "EG_FillProperties"))?;
                Ok(TextUnderlineFill::Fill(FillProperties::from_xml_element(fill_node)?))
            }
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextUnderlineFill").into()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Hyperlink {
    /// Specifies the relationship id that when looked up in this slides relationship file contains
    /// the target of this hyperlink. This attribute cannot be omitted.
    pub relationship_id: Option<RelationshipId>,
    /// Specifies the URL when it has been determined by the generating application that the
    /// URL is invalid. That is the generating application can still store the URL but it is known
    /// that this URL is not correct.
    pub invalid_url: Option<String>,
    /// Specifies an action that is to be taken when this hyperlink is activated. This can be used to
    /// specify a slide to be navigated to or a script of code to be run.
    pub action: Option<String>,
    /// Specifies the target frame that is to be used when opening this hyperlink. When the
    /// hyperlink is activated this attribute is used to determine if a new window is launched for
    /// viewing or if an existing one can be used. If this attribute is omitted, than a new window
    /// is opened.
    pub target_frame: Option<String>,
    /// Specifies the tooltip that should be displayed when the hyperlink text is hovered over
    /// with the mouse. If this attribute is omitted, than the hyperlink text itself can be
    /// displayed.
    pub tooltip: Option<String>,
    /// Specifies whether to add this URI to the history when navigating to it. This allows for the
    /// viewing of this presentation without the storing of history information on the viewing
    /// machine. If this attribute is omitted, then a value of 1 or true is assumed.
    /// 
    /// Defaults to true
    pub history: Option<bool>,
    /// Specifies if this attribute has already been used within this document. That is when a
    /// hyperlink has already been visited that this attribute would be utilized so the generating
    /// application can determine the color of this text. If this attribute is omitted, then a value
    /// of 0 or false is implied.
    /// 
    /// Defaults to false
    pub highlight_click: Option<bool>,
    /// Specifies if the URL in question should stop all sounds that are playing when it is clicked.
    /// 
    /// Defaults to false
    pub end_sound: Option<bool>,
    pub sound: Option<EmbeddedWAVAudioFile>,
}

impl Hyperlink {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "r:id" => instance.relationship_id = Some(value.clone()),
                "invalidUrl" => instance.invalid_url = Some(value.clone()),
                "action" => instance.action = Some(value.clone()),
                "tgtFrame" => instance.target_frame = Some(value.clone()),
                "tooltip" => instance.tooltip = Some(value.clone()),
                "history" => instance.history = Some(parse_xml_bool(value)?),
                "highlightClick" => instance.highlight_click = Some(parse_xml_bool(value)?),
                "endSnd" => instance.end_sound = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        instance.sound = match xml_node.child_nodes.get(0) {
            Some(node) => Some(EmbeddedWAVAudioFile::from_xml_element(node)?),
            None => None,
        };

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextCharacterProperties {
    /// Specifies whether the numbers contained within vertical text continue vertically with the
    /// text or whether they are to be displayed horizontally while the surrounding characters
    /// continue in a vertical fashion. If this attribute is omitted, than a value of 0, or false is
    /// assumed.
    pub kumimoji: Option<bool>,

    /// Specifies the language to be used when the generating application is displaying the user
    /// interface controls. If this attribute is omitted, than the generating application can select a
    /// language of its choice.
    pub language: Option<TextLanguageID>,

    /// Specifies the alternate language to use when the generating application is displaying the
    /// user interface controls. If this attribute is omitted, than the lang attribute is used here.
    pub alternative_language: Option<TextLanguageID>,

    /// Specifies the size of text within a text run. Whole points are specified in increments of
    /// 100 starting with 100 being a point size of 1. For instance a font point size of 12 would be
    /// 1200 and a font point size of 12.5 would be 1250. If this attribute is omitted, than the
    /// value in defRPr should be used.
    pub font_size: Option<TextFontSize>,

    /// Specifies whether a run of text is formatted as bold text. If this attribute is omitted, than
    /// a value of 0, or false is assumed.
    pub bold: Option<bool>,

    /// Specifies whether a run of text is formatted as italic text. If this attribute is omitted, than
    /// a value of 0, or false is assumed.
    pub italic: Option<bool>,

    /// Specifies whether a run of text is formatted as underlined text. If this attribute is omitted,
    /// than no underline is assumed.
    pub underline: Option<TextUnderlineType>,

    /// Specifies whether a run of text is formatted as strikethrough text. If this attribute is
    /// omitted, than no strikethrough is assumed.
    pub strikethrough: Option<TextStrikeType>,

    /// Specifies the minimum font size at which character kerning occurs for this text run.
    /// Whole points are specified in increments of 100 starting with 100 being a point size of 1.
    /// For instance a font point size of 12 would be 1200 and a font point size of 12.5 would be
    /// 1250. If this attribute is omitted, than kerning occurs for all font sizes down to a 0 point
    /// font.
    pub kerning: Option<TextNonNegativePoint>,

    /// Specifies the capitalization that is to be applied to the text run. This is a render-only
    /// modification and does not affect the actual characters stored in the text run. This
    /// attribute is also distinct from the toggle function where the actual characters stored in
    /// the text run are changed.
    pub capitalization: Option<TextCapsType>,

    /// Specifies the spacing between characters within a text run. This spacing is specified
    /// numerically and should be consistently applied across the entire run of text by the
    /// generating application. Whole points are specified in increments of 100 starting with 100
    /// being a point size of 1. For instance a font point size of 12 would be 1200 and a font point
    /// size of 12.5 would be 1250. If this attribute is omitted than a value of 0 or no adjustment
    /// is assumed.
    pub spacing: Option<TextPoint>,

    /// Specifies the normalization of height that is to be applied to the text run. This is a renderonly
    /// modification and does not affect the actual characters stored in the text run. This
    /// attribute is also distinct from the toggle function where the actual characters stored in
    /// the text run are changed. If this attribute is omitted, than a value of 0, or false is
    /// assumed.
    pub normalize_heights: Option<bool>,

    /// Specifies the baseline for both the superscript and subscript fonts. The size is specified
    /// using a percentage where 1% is equal to 1 percent of the font size and 100% is equal to
    /// 100 percent font of the font size.
    pub baseline: Option<Percentage>,

    /// Specifies that a run of text has been selected by the user to not be checked for mistakes.
    /// Therefore if there are spelling, grammar, etc mistakes within this text the generating
    /// application should ignore them.
    pub no_proofing: Option<bool>,

    /// Specifies that the content of a text run has changed since the proofing tools have last
    /// been run. Effectively this flags text that is to be checked again by the generating
    /// application for mistakes such as spelling, grammar, etc.
    /// 
    /// Defaults to true
    pub dirty: Option<bool>,

    /// Specifies that when this run of text was checked for spelling, grammar, etc. that a
    /// mistake was indeed found. This allows the generating application to effectively save the
    /// state of the mistakes within the document instead of having to perform a full pass check
    /// upon opening the document.
    /// 
    /// Defaults to false
    pub spelling_error: Option<bool>,

    /// Specifies whether or not a text run has been checked for smart tags. This attribute acts
    /// much like the dirty attribute dose for the checking of spelling, grammar, etc. A value of
    /// true here indicates to the generating application that this text run should be checked for
    /// smart tags. If this attribute is omitted, than a value of 0, or false is assumed.
    pub smarttag_clean: Option<bool>,

    /// Specifies a smart tag identifier for a run of text. This ID is unique throughout the
    /// presentation and is used to reference corresponding auxiliary information about the
    /// smart tag.
    /// 
    /// Defaults to 0
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr/>
    ///   <a:lstStyle/>
    ///   <a:p>
    ///     <a:r>
    ///       <a:rPr lang="en-US" dirty="0" smtId="1"/>
    ///       <a:t>CNTS</a:t>
    ///     </a:r>
    ///     <a:endParaRPr lang="en-US" dirty="0"/>
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// The text run has a smtId attribute value of 1, which denotes that the text should be
    /// inspected for smart tag information, which in this case maps to a stock ticker symbol.
    pub smarttag_id: Option<u32>,

    /// Specifies the link target name that is used to reference to the proper link properties in a
    /// custom XML part within the document.
    pub bookmark_link_target: Option<String>,
    pub line_properties: Option<Box<LineProperties>>,
    pub fill_properties: Option<FillProperties>,
    pub effect_properties: Option<EffectProperties>,
    pub highlight_color: Option<Color>,
    pub text_underline_line: Option<TextUnderlineLine>,
    pub text_underline_fill: Option<TextUnderlineFill>,
    pub latin_font: Option<TextFont>,
    pub east_asian_font: Option<TextFont>,
    pub complex_script_font: Option<TextFont>,
    pub symbol_font: Option<TextFont>,
    pub hyperlink_click: Option<Box<Hyperlink>>,
    pub hyperlink_mouse_over: Option<Box<Hyperlink>>,

    /// This element specifies whether the contents of this run shall have right-to-left characteristics. Specifically, the
    /// following behaviors are applied when this element’s val attribute is true (or an equivalent):
    /// 
    /// * Formatting – When the contents of this run are displayed, all characters shall be treated as complex
    ///   script characters. This means that the values of the cs element (§21.1.2.3.1) shall be used to determine
    ///   the font face.
    /// 
    /// * Character Directionality Override – When the contents of this run are displayed, this property acts as a
    ///   right-to-left override for characters which are classified as follows (using the Unicode Character
    ///   Database):
    /// 
    ///   * Weak types except European Number, European Number Terminator, Common Number Separator,
    ///     Arabic Number and (for Hebrew text) European Number Separator when constituting part of a
    ///     number
    /// 
    ///   * Neutral types
    /// 
    /// * This element provides information used to resolve the (Unicode) classifications of individual characters
    ///   as either L, R, AN or EN. Once this is determined, the line should be displayed subject to the
    ///   recommendation of the Unicode Bidirectional Algorithm in reordering resolved levels.
    /// 
    ///   # Rationale
    /// 
    ///   This override allows applications to store and utilize higher-level information beyond that
    ///   implicitly derived from the Unicode Bidirectional algorithm. For example, if the string “first second”
    ///   appears in a right-to-left paragraph inside a document, the Unicode algorithm would always result in
    ///   “first second” at display time (since the neutral character is surrounded by strongly classified
    ///   characters). However, if the whitespace was entered using a right-to-left input method (e.g. a Hebrew
    ///   keyboard), then that character could be classified as RTL using this property, allowing the display of
    ///   “second first” in a right-to-left paragraph, since the user explicitly asked for the space in a right-to-left
    ///   context.
    /// 
    /// This property shall not be used with strong left-to-right text. Any behavior under that condition is unspecified.
    /// This property, when off, should not be used with strong right-to-left text. Any behavior under that condition is
    /// unspecified.
    /// 
    /// If this element is not present, the default value is to leave the formatting applied at previous level in the style
    /// hierarchy. If this element is never applied in the style hierarchy, then right to left characteristics shall not be
    /// applied to the contents of this run.
    /// 
    /// # Xml example
    /// 
    /// Consider the following DrawingML visual content: “first second, أولى ثاني ”. This content might
    /// appear as follows within its parent paragraph:
    /// ```xml
    /// <a:p>
    ///   <a:r>
    ///     <a:t>first second, </w:t>
    ///   </a:r>
    ///   <a:r>
    ///     <a:rPr>
    ///       <a:rtl/>
    ///     </a:rPr>
    ///     <a:t> أولى </a:t>
    ///   </a:r>
    ///   <a:r>
    ///     <a:rPr>
    ///       <a:rtl/>
    ///     </a:rPr>
    ///     <a:t> </a:t>
    ///   </a:r>
    ///   <a:r>
    ///     <a:rPr>
    ///       <a:rtl/>
    ///     </a:rPr>
    ///     <a:t> ثاني </a:t>
    ///   </a:r>
    /// </a:p>
    /// ```
    /// 
    /// The presence of the rtl element on the second, third, and fourth runs specifies that:
    /// 
    /// * The formatting on those runs is specified using the complex-script property variants.
    /// * The whitespace character is treated as right-to-left.
    /// 
    /// Note that the second, third and fourth runs could be joined as one run with the rtl element specified.
    pub rtl: Option<bool>,
}

impl TextCharacterProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextCharacterProperties> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "kumimoji" => instance.kumimoji = Some(parse_xml_bool(value)?),
                "lang" => instance.language = Some(value.clone()),
                "altLang" => instance.alternative_language = Some(value.clone()),
                "sz" => instance.font_size = Some(value.parse()?),
                "b" => instance.bold = Some(parse_xml_bool(value)?),
                "i" => instance.italic = Some(parse_xml_bool(value)?),
                "u" => instance.underline = Some(value.parse()?),
                "strike" => instance.strikethrough = Some(value.parse()?),
                "kern" => instance.kerning = Some(value.parse()?),
                "cap" => instance.capitalization = Some(value.parse()?),
                "spc" => instance.spacing = Some(value.parse()?),
                "normalizeH" => instance.normalize_heights = Some(parse_xml_bool(value)?),
                "baseline" => instance.baseline = Some(value.parse()?),
                "noProof" => instance.no_proofing = Some(parse_xml_bool(value)?),
                "dirty" => instance.dirty = Some(parse_xml_bool(value)?),
                "err" => instance.spelling_error = Some(parse_xml_bool(value)?),
                "smtClean" => instance.smarttag_clean = Some(parse_xml_bool(value)?),
                "smtId" => instance.smarttag_id = Some(value.parse()?),
                "bmk" => instance.bookmark_link_target = Some(value.clone()),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            let child_local_name = child_node.local_name();
            if FillProperties::is_choice_member(child_local_name) {
                instance.fill_properties = Some(FillProperties::from_xml_element(child_node)?);
            } else if EffectProperties::is_choice_member(child_local_name) {
                instance.effect_properties = Some(EffectProperties::from_xml_element(child_node)?);
            } else if TextUnderlineLine::is_choice_member(child_local_name) {
                instance.text_underline_line = Some(TextUnderlineLine::from_xml_element(child_node)?);
            } else if TextUnderlineFill::is_choice_member(child_local_name) {
                instance.text_underline_fill = Some(TextUnderlineFill::from_xml_element(child_node)?);
            } else {
                match child_local_name {
                    "ln" => instance.line_properties = Some(Box::new(LineProperties::from_xml_element(child_node)?)),
                    "highlight" => {
                        let color_node = child_node
                            .child_nodes
                            .get(0)
                            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "CT_Color"))?;
                        instance.highlight_color = Some(Color::from_xml_element(color_node)?);
                    }
                    "latin" => instance.latin_font = Some(TextFont::from_xml_element(child_node)?),
                    "ea" => instance.east_asian_font = Some(TextFont::from_xml_element(child_node)?),
                    "cs" => instance.complex_script_font = Some(TextFont::from_xml_element(child_node)?),
                    "sym" => instance.symbol_font = Some(TextFont::from_xml_element(child_node)?),
                    "hlinkClick" => instance.hyperlink_click = Some(Box::new(Hyperlink::from_xml_element(child_node)?)),
                    "hlinkMouseOver" => {
                        instance.hyperlink_mouse_over = Some(Box::new(Hyperlink::from_xml_element(child_node)?))
                    }
                    "rtl" => {
                        instance.rtl = match child_node.text {
                            Some(ref s) => Some(parse_xml_bool(s)?),
                            None => None,
                        }
                    }
                    _ => (),
                }
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextParagraphProperties {
    /// Specifies the left margin of the paragraph. This is specified in addition to the text body
    /// inset and applies only to this text paragraph. That is the text body inset and the marL
    /// attributes are additive with respect to the text position. If this attribute is omitted, then a
    /// value of 347663 is implied.
    pub margin_left: Option<TextMargin>,

    /// Specifies the right margin of the paragraph. This is specified in addition to the text body
    /// inset and applies only to this text paragraph. That is the text body inset and the marR
    /// attributes are additive with respect to the text position. If this attribute is omitted, then a
    /// value of 0 is implied.
    pub margin_right: Option<TextMargin>,

    /// Specifies the particular level text properties that this paragraph follows. The value for this
    /// attribute is numerical and formats the text according to the corresponding level
    /// paragraph properties that are listed within the lstStyle element. Since there are nine
    /// separate level properties defined, this tag has an effective range of 0-8 = 9 available
    /// values.
    /// 
    /// # Xml example
    /// 
    /// Consider the following DrawingML. This would specify that this paragraph
    /// should follow the lvl2pPr formatting style because once again lvl="1" is considered to be
    /// level 2.
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:pPr lvl="1" …/>
    ///     …
    ///     <a:t>Sample text</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// # Note
    /// 
    /// To resolve conflicting paragraph properties the linear hierarchy of paragraph
    /// properties should be examined starting first with the pPr element. The rule here is that
    /// properties that are defined at a level closer to the actual text should take precedence.
    /// That is if there is a conflicting property between the pPr and lvl1pPr elements then the
    /// pPr property should take precedence because in the property hierarchy it is closer to the
    /// actual text being represented.
    pub level: Option<TextIndentLevelType>,

    /// Specifies the indent size that is applied to the first line of text in the paragraph. An
    /// indentation of 0 is considered to be at the same location as marL attribute. If this
    /// attribute is omitted, then a value of -342900 is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the scenario where the user now wanted to add a paragraph
    /// indentation to the first line of text in their two column format book.
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr numCol="2" spcCol="914400"…/>
    ///     <a:normAutofit/>
    ///   </a:bodyPr>
    ///   …
    ///   <a:p>
    ///     <a:pPr marL="0" indent="571500" algn="just">
    ///       <a:buNone/>
    ///     </a:pPr>
    ///     …
    ///     <a:t>Here is some…</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// By adding the indent attribute the user has effectively added a first line indent to this
    /// paragraph of text.
    pub indent: Option<TextIndent>,

    /// Specifies the alignment that is to be applied to the paragraph. Possible values for this
    /// include left, right, centered, justified and distributed. If this attribute is omitted, then a
    /// value of left is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the user wishes to have two columns of text that
    /// have a justified alignment, much like text within a book. The following DrawingML could
    /// describe this.
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr numCol="2" spcCol="914400"…/>
    ///     <a:normAutofit/>
    ///   </a:bodyPr>
    ///   …
    ///   <a:p>
    ///     <a:pPr marL="0" algn="just">
    ///       <a:buNone/>
    ///     </a:pPr>
    ///     …
    ///     <a:t>Sample Text …</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub align: Option<TextAlignType>,

    /// Specifies the default size for a tab character within this paragraph. This attribute should
    /// be used to describe the spacing of tabs within the paragraph instead of a leading
    /// indentation tab. For indentation tabs there are the marL and indent attributes to assist
    /// with this.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where a paragraph contains numerous tabs that need to be
    /// of a specific size. The following DrawingML would describe this.
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:pPr defTabSz="376300" …/>
    ///     …
    ///     <a:t>Sample Text …</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub default_tab_size: Option<Coordinate32>,

    /// Specifies whether the text is right-to-left or left-to-right in its flow direction. If this
    /// attribute is omitted, then a value of 0, or left-to-right is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the following example of a text body with two lines of text. In this
    /// example, both lines contain English and Arabic text, however, the second line has the
    /// rtl attribute set to true whereas the first line does not set the rtl attribute.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:r>
    ///       <a:t>Test </a:t>
    ///     </a:r>
    ///     <a:r>
    ///       <a:rPr>
    ///         <a:rtl w:val="1"/>
    ///       </a:rPr>
    ///       <a:t> تجربة </a:t>
    ///     </a:r>
    ///   </a:p>
    ///   <a:p>
    ///     <a:pPr rtl="1"/>
    ///     <a:r>
    ///       <a:rPr>
    ///         <a:rtl w:val="0"/>
    ///       </a:rPr>
    ///       <a:t>Test </a:t>
    ///     </a:r>
    ///     <a:r>
    ///       <a:t> تجربة </a:t>
    ///     </a:r>
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub rtl: Option<bool>,

    /// Specifies whether an East Asian word can be broken in half and wrapped onto the next
    /// line without a hyphen being added. To determine whether an East Asian word can be
    /// broken the presentation application would use the kinsoku settings here. This attribute
    /// is to be used specifically when there is a word that cannot be broken into multiple pieces
    /// without a hyphen. That is it is not present within the existence of normal breakable East
    /// Asian words but is when a special case word arises that should not be broken for a line
    /// break. If this attribute is omitted, then a value of 1 or true is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the presentation contains a long word that must not
    /// be divided with a line break. Instead it should be placed, in whole on a new line so that it
    /// can fit. The picture below shows a normal paragraph where a long word has been broken
    /// for a line break. The second picture shown below shows that same paragraph with the
    /// long word specified to not allow a line break. The resulting DrawingML is as follows.
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:pPr eaLnBrk="0" …/>
    ///     …
    ///     <a:t>Sample text (Long word)</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub east_asian_line_break: Option<bool>,

    /// Determines where vertically on a line of text the actual words are positioned. This deals
    /// with vertical placement of the characters with respect to the baselines. For instance
    /// having text anchored to the top baseline, anchored to the bottom baseline, centered in
    /// between, etc. To understand this attribute and it's use it is helpful to understand what
    /// baselines are. A diagram describing these different cases is shown below. If this attribute
    /// is omitted, then a value of base is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the user wishes to represent the chemical compound
    /// of a water molecule. For this they need to make sure the H, the 2, and the O are all in the
    /// correct position and are of the correct size. The results below can be achieved through
    /// the DrawingML shown below.
    /// 
    /// ```xml
    /// <a:txtBody>
    ///   …
    ///   <a:pPr fontAlgn="b" …/>
    ///   …
    ///   <a:r>
    ///     <a:rPr …/>
    ///     <a:t>H </a:t>
    ///   </a:r>
    ///   <a:r>
    ///     <a:rPr sz="1200" …/>
    ///     <a:t>2</a:t>
    ///   </a:r>
    ///   <a:r>
    ///     <a:rPr …/>
    ///     <a:t>O</a:t>
    ///   </a:r>
    ///   …
    /// </p:txBody>
    /// ```
    pub font_align: Option<TextFontAlignType>,

    /// Specifies whether a Latin word can be broken in half and wrapped onto the next line
    /// without a hyphen being added. This attribute is to be used specifically when there is a
    /// word that cannot be broken into multiple pieces without a hyphen. It is not present
    /// within the existence of normal breakable Latin words but is when a special case word
    /// arises that should not be broken for a line break. If this attribute is omitted, then a value
    /// of 1 or true is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the presentation contains a long word that must not
    /// be divided with a line break. Instead it should be placed, in whole on a new line so that it
    /// can fit. The picture below shows a normal paragraph where a long word has been broken
    /// for a line break. The second picture shown below shows that same paragraph with the
    /// long word specified to not allow a line break. The resulting DrawingML is as follows.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:pPr latinLnBrk="0" …/>
    ///     …
    ///     <a:t>Sample text (Long word)</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub latin_line_break: Option<bool>,

    /// Specifies whether punctuation is to be forcefully laid out on a line of text or put on a
    /// different line of text. That is, if there is punctuation at the end of a run of text that should
    /// be carried over to a separate line does it actually get carried over. A true value allows for
    /// hanging punctuation forcing the punctuation to not be carried over and a value of false
    /// allows the punctuation to be carried onto the next text line. If this attribute is omitted,
    /// then a value of 0, or false is implied.
    pub hanging_punctuations: Option<bool>,

    /// This element specifies the vertical line spacing that is to be used within a paragraph. This can be specified in two
    /// different ways, percentage spacing and font point spacing. If this element is omitted then the spacing between
    /// two lines of text should be determined by the point size of the largest piece of text within a line.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:p>
    ///     <a:pPr>
    ///       <a:lnSpc>
    ///         <a:spcPct val="200%"/>
    ///       </a:lnSpc>
    ///     </a:pPr>
    ///     <a:r>
    ///       <a:rPr lang="en-US" dirty="0" smtClean="0"/>
    ///       <a:t>Some</a:t>
    ///     </a:r>
    ///     <a:br>
    ///       <a:rPr lang="en-US" smtClean="0"/>
    ///     </a:br>
    ///     <a:r>
    ///      <a:rPr lang="en-US" dirty="0" smtClean="0"/>
    ///      <a:t>Text</a:t>
    ///     </a:r>
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// This paragraph has two lines of text that have percentage based vertical spacing. This kind of spacing should
    /// change based on the size of the text involved as its size is calculated as a percentage of this.
    pub line_spacing: Option<TextSpacing>,
    pub space_before: Option<TextSpacing>,
    pub space_after: Option<TextSpacing>,
    pub bullet_color: Option<TextBulletColor>,
    pub bullet_size: Option<TextBulletSize>,
    pub bullet_typeface: Option<TextBulletTypeface>,
    pub bullet: Option<TextBullet>,
    pub tab_stop_list: Vec<TextTabStop>,
    pub default_run_properties: Option<Box<TextCharacterProperties>>,
}

impl TextParagraphProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<TextParagraphProperties> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "marL" => instance.margin_left = Some(value.parse()?),
                "marR" => instance.margin_right = Some(value.parse()?),
                "lvl" => instance.level = Some(value.parse()?),
                "indent" => instance.indent = Some(value.parse()?),
                "algn" => instance.align = Some(value.parse()?),
                "defTabSz" => instance.default_tab_size = Some(value.parse()?),
                "rtl" => instance.rtl = Some(parse_xml_bool(value)?),
                "eaLnBrk" => instance.east_asian_line_break = Some(parse_xml_bool(value)?),
                "fontAlgn" => instance.font_align = Some(value.parse()?),
                "latinLnBrk" => instance.latin_line_break = Some(parse_xml_bool(value)?),
                "hangingPunct" => instance.hanging_punctuations = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            if TextBulletColor::is_choice_member(child_node.local_name()) {
                instance.bullet_color = Some(TextBulletColor::from_xml_element(child_node)?);
            } else if TextBulletColor::is_choice_member(child_node.local_name()) {
                instance.bullet_size = Some(TextBulletSize::from_xml_element(child_node)?);
            } else if TextBulletTypeface::is_choice_member(child_node.local_name()) {
                instance.bullet_typeface = Some(TextBulletTypeface::from_xml_element(child_node)?);
            } else if TextBullet::is_choice_member(child_node.local_name()) {
                instance.bullet = Some(TextBullet::from_xml_element(child_node)?);
            } else {
                match child_node.local_name() {
                    "lnSpc" => {
                        let line_spacing_node = child_node
                            .child_nodes
                            .get(0)
                            .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "lnSpc child"))?;
                        instance.line_spacing = Some(TextSpacing::from_xml_element(line_spacing_node)?);
                    }
                    "spcBef" => {
                        let space_before_node = child_node
                            .child_nodes
                            .get(0)
                            .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "spcBef child"))?;
                        instance.space_before = Some(TextSpacing::from_xml_element(space_before_node)?);
                    }
                    "spcAft" => {
                        let space_after_node = child_node
                            .child_nodes
                            .get(0)
                            .ok_or_else(|| MissingChildNodeError::new(child_node.name.clone(), "spcAft child"))?;
                        instance.space_after = Some(TextSpacing::from_xml_element(space_after_node)?);
                    }
                    "tabLst" => instance.tab_stop_list.push(TextTabStop::from_xml_element(child_node)?),
                    "defRPr" => {
                        instance.default_run_properties =
                            Some(Box::new(TextCharacterProperties::from_xml_element(child_node)?))
                    }
                    _ => (),
                }
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextParagraph {
    /// This element contains all paragraph level text properties for the containing paragraph. These paragraph
    /// properties should override any and all conflicting properties that are associated with the paragraph in question.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:p>
    ///   <a:pPr marL="0" algn="ctr">
    ///     <a:buNone/>
    ///   </a:pPr>
    ///   …
    ///   <a:t>Some Text</a:t>
    ///   …
    /// </a:p>
    /// ```
    /// 
    /// The paragraph described above is formatting with a left margin of 0 and has all of text runs contained within it
    /// centered about the horizontal median of the bounding box for the text body.
    /// 
    /// # Note
    /// 
    /// To resolve conflicting paragraph properties the linear hierarchy of paragraph properties should be
    /// examined starting first with the pPr element. The rule here is that properties that are defined at a level closer to
    /// the actual text should take precedence. That is if there is a conflicting property between the pPr and lvl1pPr
    /// elements then the pPr property should take precedence because in the property hierarchy it is closer to the
    /// actual text being represented.
    pub properties: Option<Box<TextParagraphProperties>>,

    pub text_run_list: Vec<TextRun>,

    /// This element specifies the text run properties that are to be used if another run is inserted after the last run
    /// specified. This effectively saves the run property state so that it can be applied when the user enters additional
    /// text. If this element is omitted, then the application can determine which default properties to apply. It is
    /// recommended that this element be specified at the end of the list of text runs within the paragraph so that an
    /// orderly list is maintained.
    pub end_paragraph_char_properties: Option<Box<TextCharacterProperties>>,
}

impl TextParagraph {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for child_node in &xml_node.child_nodes {
            let local_name = child_node.local_name();
            if TextRun::is_choice_member(local_name) {
                instance.text_run_list.push(TextRun::from_xml_element(child_node)?);
            } else {
                match child_node.local_name() {
                    "pPr" => {
                        instance.properties = Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                    }
                    "endParaRPr" => {
                        instance.end_paragraph_char_properties =
                            Some(Box::new(TextCharacterProperties::from_xml_element(child_node)?))
                    }
                    _ => (),
                }
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub enum TextRun {
    RegularTextRun(Box<RegularTextRun>),

    /// This element specifies the existence of a vertical line break between two runs of text within a paragraph. In
    /// addition to specifying a vertical space between two runs of text, this element can also have run properties
    /// specified via the rPr child element. This sets the formatting of text for the line break so that if text is later
    /// inserted there that a new run can be generated with the correct formatting.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:r>
    ///       …
    ///       <a:t>Text Run 1.</a:t>
    ///       …
    ///     </a:r>
    ///     <a:br/>
    ///       <a:r>
    ///       …
    ///       <a:t>Text Run 2.</a:t>
    ///       …
    ///     </a:r>
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// This paragraph has two runs of text laid out in a vertical fashion with a line break in between them. This line
    /// break acts much like a carriage return would within a normal run of text.
    LineBreak(Box<TextLineBreak>),

    /// This element specifies a text field which contains generated text that the application should update periodically.
    /// Each piece of text when it is generated is given a unique identification number that is used to refer to a specific
    /// field. At the time of creation the text field indicates the kind of text that should be used to update this field. This
    /// update type is used so that all applications that did not create this text field can still know what kind of text it
    /// should be updated with. Thus the new application can then attach an update type to the text field id for
    /// continual updating.
    /// 
    /// # Xml example
    /// 
    /// Consider a slide within a presentation that needs to have the slide number placed on the slide. The
    /// following DrawingML can be used to describe such a situation.
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr/>
    ///   <a:lstStyle/>
    ///   <a:p>
    ///     <a:fld id="{424CEEAC-8F67-4238-9622-1B74DC6E8318}" type="slidenum">
    ///       <a:rPr lang="en-US" smtClean="0"/>
    ///       <a:pPr/>
    ///       <a:t>3</a:t>
    ///     </a:fld>
    ///     <a:endParaRPr lang="en-US"/>
    ///   </a:p>
    /// </p:txBody>
    /// ```
    TextField(Box<TextField>),
}

impl TextRun {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "r" | "br" | "fld" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "r" => Ok(TextRun::RegularTextRun(Box::new(RegularTextRun::from_xml_element(
                xml_node,
            )?))),
            "br" => Ok(TextRun::LineBreak(Box::new(TextLineBreak::from_xml_element(xml_node)?))),
            "fld" => Ok(TextRun::TextField(Box::new(TextField::from_xml_element(xml_node)?))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextRun").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegularTextRun {
    pub char_properties: Option<Box<TextCharacterProperties>>,
    pub text: String,
}

impl RegularTextRun {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut char_properties = None;
        let mut text = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "rPr" => char_properties = Some(Box::new(TextCharacterProperties::from_xml_element(child_node)?)),
                "t" => text = child_node.text.clone(),
                _ => (),
            }
        }

        let text = text.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "t"))?;
        Ok(Self { char_properties, text })
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextLineBreak {
    pub char_properties: Option<Box<TextCharacterProperties>>,
}

impl TextLineBreak {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let char_properties = match xml_node.child_nodes.get(0) {
            Some(node) => Some(Box::new(TextCharacterProperties::from_xml_element(node)?)),
            None => None,
        };

        Ok(Self { char_properties })
    }
}

#[derive(Debug, Clone)]
pub struct TextField {
    /// Specifies the unique to this document, host specified token that is used to identify the
    /// field. This token is generated when the text field is created and persists in the file as the
    /// same token until the text field is removed. Any application should check the document
    /// for conflicting tokens before assigning a new token to a text field.
    pub id: Guid,

    /// Specifies the type of text that should be used to update this text field. This is used to
    /// inform the rendering application what text it should use to update this text field. There
    /// are no specific syntax restrictions placed on this attribute. The generating application can
    /// use it to represent any text that should be updated before rendering the presentation.
    /// 
    /// Reserved values:
    /// 
    /// |Value          |Description                                            |
    /// |---------------|-------------------------------------------------------|
    /// |slidenum       |presentation slide number                              |
    /// |datetime       |default date time format for the rendering application |
    /// |datetime1      |MM/DD/YYYY date time format                            |
    /// |datetime2      |Day, Month DD, YYYY date time format                   |
    /// |datetime3      |DD Month YYYY date time format                         |
    /// |datetime4      |Month DD, YYYY date time format                        |
    /// |datetime5      |DD-Mon-YY date time format                             |
    /// |datetime6      |Month YY date time format                              |
    /// |datetime7      |Mon-YY date time format                                |
    /// |datetime8      |MM/DD/YYYY hh:mm AM/PM date time format                |
    /// |datetime9      |MM/DD/YYYY hh:mm:ss AM/PM date time format             |
    /// |datetime10     |hh:mm date time format                                 |
    /// |datetime11     |hh:mm:ss date time format                              |
    /// |datetime12     |hh:mm AM/PM date time format                           |
    /// |datetime13     |hh:mm:ss: AM/PM date time format                       |
    pub field_type: Option<String>,
    pub char_properties: Option<Box<TextCharacterProperties>>,
    pub paragraph_properties: Option<Box<TextParagraph>>,
    pub text: Option<String>,
}

impl TextField {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut id = None;
        let mut field_type = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "id" => id = Some(value.clone()),
                "type" => field_type = Some(value.clone()),
                _ => (),
            }
        }

        let id = id.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "id"))?;

        let mut char_properties = None;
        let mut paragraph_properties = None;
        let mut text = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "rPr" => char_properties = Some(Box::new(TextCharacterProperties::from_xml_element(child_node)?)),
                "pPr" => paragraph_properties = Some(Box::new(TextParagraph::from_xml_element(child_node)?)),
                "t" => text = child_node.text.clone(),
                _ => (),
            }
        }

        Ok(Self {
            id,
            field_type,
            char_properties,
            paragraph_properties,
            text,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextListStyle {
    /// This element specifies the paragraph properties that are to be applied when no other paragraph properties have
    /// been specified. If this attribute is omitted, then it is left to the application to decide the set of default paragraph
    /// properties that should be applied.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:lstStyle>
    ///     <a:defPPr>
    ///       <a:buNone/>
    ///     </a:defPPr>
    ///   </a:lstStyle>
    ///   <a:p>
    ///     …
    ///     <a:t>Sample Text</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// The above paragraph follows the properties described in defPPr if no overriding properties are specified within
    /// the pPr element.
    pub def_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl1_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl2_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl3_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl4_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl5_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl6_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl7_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl8_paragraph_props: Option<Box<TextParagraphProperties>>,
    pub lvl9_paragraph_props: Option<Box<TextParagraphProperties>>,
}

impl TextListStyle {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "defPPr" => {
                    instance.def_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl1pPr" => {
                    instance.lvl1_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl2pPr" => {
                    instance.lvl2_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl3pPr" => {
                    instance.lvl3_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl4pPr" => {
                    instance.lvl4_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl5pPr" => {
                    instance.lvl5_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl6pPr" => {
                    instance.lvl6_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl7pPr" => {
                    instance.lvl7_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl8pPr" => {
                    instance.lvl8_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                "lvl9pPr" => {
                    instance.lvl9_paragraph_props =
                        Some(Box::new(TextParagraphProperties::from_xml_element(child_node)?))
                }
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct TextBody {
    pub body_properties: Box<TextBodyProperties>,
    pub list_style: Option<Box<TextListStyle>>,

    /// This element specifies the presence of a paragraph of text within the containing text body. The paragraph is the
    /// highest level text separation mechanism within a text body. A paragraph can contain text paragraph properties
    /// associated with the paragraph. If no properties are listed then properties specified in the defPPr element are
    /// used.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the user would like to describe a text body that contains two paragraphs.
    /// The requirement for these paragraphs is that one be right aligned and the other left aligned. The following
    /// DrawingML would specify a text body such as this.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   …
    ///   <a:p>
    ///     <a:pPr algn="r">
    ///     </a:pPr>
    ///     …
    ///     <a:t>Some text</a:t>
    ///     …
    ///   </a:p>
    ///   <a:p>
    ///     <a:pPr algn="l">
    ///     </a:pPr>
    ///     …
    ///     <a:t>Some text</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub paragraph_array: Vec<Box<TextParagraph>>,
}

impl TextBody {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut body_properties = None;
        let mut list_style = None;
        let mut paragraph_array = Vec::new();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "bodyPr" => body_properties = Some(Box::new(TextBodyProperties::from_xml_element(child_node)?)),
                "lstStyle" => list_style = Some(Box::new(TextListStyle::from_xml_element(child_node)?)),
                "p" => paragraph_array.push(Box::new(TextParagraph::from_xml_element(child_node)?)),
                _ => (),
            }
        }

        let body_properties =
            body_properties.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "bodyPr"))?;

        Ok(Self {
            body_properties,
            list_style,
            paragraph_array,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextBodyProperties {
    /// Specifies the rotation that is being applied to the text within the bounding box. If it not
    /// specified, the rotation of the accompanying shape is used. If it is specified, then this is
    /// applied independently from the shape. That is the shape can have a rotation applied in
    /// addition to the text itself having a rotation applied to it. If this attribute is omitted, then a
    /// value of 0, is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where a shape has a rotation of 5400000, or 90 degrees
    /// clockwise applied to it. In addition to this, the text body itself has a rotation of -5400000,
    /// or 90 degrees counter-clockwise applied to it. Then the resulting shape would appear to
    /// be rotated but the text within it would appear as though it had not been rotated at all.
    /// The DrawingML specifying this would look like the following:
    /// 
    /// ```xml
    /// <p:sp>
    ///   <p:spPr>
    ///     <a:xfrm rot="5400000">
    ///     …
    ///     </a:xfrm>
    ///   </p:spPr>
    ///   …
    ///   <p:txBody>
    ///     <a:bodyPr rot="-5400000" … />
    ///     …
    ///     (Some text)
    ///     …
    ///   </p:txBody>
    /// </p:sp>
    /// ```
    pub rotate_angle: Option<Angle>,
    /// Specifies whether the before and after paragraph spacing defined by the user is to be
    /// respected. While the spacing between paragraphs is helpful, it is additionally useful to be
    /// able to set a flag as to whether this spacing is to be followed at the edges of the text
    /// body, in other words the first and last paragraphs in the text body. More precisely since
    /// this is a text body level property it should only effect the before paragraph spacing of the
    /// first paragraph and the after paragraph spacing of the last paragraph for a given text
    /// body. If this attribute is omitted, then a value of 0, or false is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where spacing has been defined between multiple
    /// paragraphs within a text body using the spcBef and spcAft paragraph spacing attributes.
    /// For this text body however the user would like to not have this followed for the edge
    /// paragraphs and thus we have the following DrawingML.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr spcFirstLastPara="0" … />
    ///   …
    ///   <a:p>
    ///     <a:pPr>
    ///       <a:spcBef>
    ///         <a:spcPts val="1800"/>
    ///       </a:spcBef>
    ///       <a:spcAft>
    ///         <a:spcPts val="600"/>
    ///       </a:spcAft>
    ///     </a:pPr>
    ///     …
    ///     (Some text)
    ///     …
    ///   </a:p>
    ///   <a:p>
    ///     <a:pPr>
    ///       <a:spcBef>
    ///         <a:spcPts val="1800"/>
    ///       </a:spcBef>
    ///       <a:spcAft>
    ///         <a:spcPts val="600"/>
    ///       </a:spcAft>
    ///     </a:pPr>
    ///     …
    ///     (Some text)
    ///     …
    ///   </a:p>
    ///   …
    /// </p:txBody>
    /// ```
    pub paragraph_spacing: Option<bool>,
    /// Determines whether the text can flow out of the bounding box vertically. This is used to
    /// determine what happens in the event that the text within a shape is too large for the
    /// bounding box it is contained within. If this attribute is omitted, then a value of overflow
    /// is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where we have multiply paragraphs within a shape and the
    /// second causes text to flow outside the shape. By applying the clip value of the
    /// vertOverflow attribute as a body property this overflowing text is now cut off instead of
    /// extending beyond the bounds of the shape.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr vertOverflow="clip" … />
    ///   …
    ///   <a:p>
    ///     …
    ///     (Some text)
    ///     …
    ///   </a:p>
    ///   <a:p>
    ///     …
    ///     (Some longer text)
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub vertical_overflow: Option<TextVertOverflowType>,
    /// Determines whether the text can flow out of the bounding box horizontally. This is used
    /// to determine what happens in the event that the text within a shape is too large for the
    /// bounding box it is contained within. If this attribute is omitted, then a value of overflow
    /// is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where we have multiply paragraphs within a shape and the
    /// second is greater in length and causes text to flow outside the shape. By applying the clip
    /// value of the horzOverflow attribute as a body property this overflowing text now is cut
    /// off instead of extending beyond the bounds of the shape.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr horzOverflow="clip" … />
    ///   …
    ///   <a:p>
    ///   …
    ///   (Some text)
    ///   …
    ///   </a:p>
    ///   <a:p>
    ///   …
    ///   (Some more text)
    ///   …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    pub horizontal_overflow: Option<TextHorizontalOverflowType>,
    /// Determines if the text within the given text body should be displayed vertically. If this
    /// attribute is omitted, then a value of horz, or no vertical text is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the user needs to display text that appears vertical
    /// and has a right to left flow with respect to its columns.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr vert="wordArtVertRtl" … />
    ///   …
    ///   <a:p>
    ///     …
    ///     <a:t>This is</a:t>
    ///     …
    ///   </a:p>
    ///   <a:p>
    ///     …
    ///     <a:t>some text.</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    /// 
    /// In the above sample DrawingML there are two paragraphs denoting a separation
    /// between the text otherwise which are known as either a line or paragraph break.
    /// Because wordArtVertRtl is used here this text is not only displayed in a stacked manner
    /// flowing from top to bottom but also have the first paragraph be displayed to the right of
    /// the second. This is because it is both vertical text and right to left.
    pub vertical_type: Option<TextVerticalType>,
    /// Specifies the wrapping options to be used for this text body. If this attribute is omitted,
    /// then a value of square is implied which wraps the text using the bounding text box.
    pub wrap_type: Option<TextWrappingType>,
    /// Specifies the left inset of the bounding rectangle. Insets are used just as internal margins
    /// for text boxes within shapes. If this attribute is omitted, then a value of 91440 or 0.1
    /// inches is implied.
    pub left_inset: Option<Coordinate32>,
    /// Specifies the top inset of the bounding rectangle. Insets are used just as internal margins
    /// for text boxes within shapes. If this attribute is omitted, then a value of 45720 or 0.05
    /// inches is implied.
    pub top_inset: Option<Coordinate32>,
    /// Specifies the right inset of the bounding rectangle. Insets are used just as internal
    /// margins for text boxes within shapes. If this attribute is omitted, then a value of 91440 or
    /// 0.1 inches is implied.
    pub right_inset: Option<Coordinate32>,
    /// Specifies the bottom inset of the bounding rectangle. Insets are used just as internal
    /// margins for text boxes within shapes. If this attribute is omitted, a value of 45720 or 0.05
    /// inches is implied.
    pub bottom_inset: Option<Coordinate32>,
    /// Specifies the number of columns of text in the bounding rectangle. When applied to a
    /// text run this property takes the width of the bounding box for the text and divides it by
    /// the number of columns specified. These columns are then treated as overflow containers
    /// in that when the previous column has been filled with text the next column acts as the
    /// repository for additional text. When all columns have been filled and text still remains
    /// then the overflow properties set for this text body are used and the text is reflowed to
    /// make room for additional text. If this attribute is omitted, then a value of 1 is implied.
    pub column_count: Option<TextColumnCount>,
    /// Specifies the space between text columns in the text area. This should only apply when
    /// there is more than 1 column present. If this attribute is omitted, then a value of 0 is
    /// implied.
    pub space_between_columns: Option<PositiveCoordinate32>,
    /// Specifies whether columns are used in a right-to-left or left-to-right order. The usage of
    /// this attribute only sets the column order that is used to determine which column
    /// overflow text should go to next. If this attribute is omitted, then a value of 0 or falseis
    /// implied in which case text starts in the leftmost column and flow to the right.
    /// 
    /// # Note
    /// 
    /// This attribute in no way determines the direction of text but merely the direction
    /// in which multiple columns are used.
    pub rtl_columns: Option<bool>,
    /// Specifies that text within this textbox is converted text from a WordArt object. This is
    /// more of a backwards compatibility attribute that is useful to the application from a
    /// tracking perspective. WordArt was the former way to apply text effects and therefore
    /// this attribute is useful in document conversion scenarios. If this attribute is omitted, then
    /// a value of 0 or false is implied.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr wrap="none" fromWordArt="1" …/>
    ///   …
    /// </p:txBody>
    /// ```
    /// 
    /// Because of the presence of the fromWordArt attribute the text within this shape can be
    /// mapped back to the corresponding WordArt during document conversion.
    pub is_from_word_art: Option<bool>,
    /// Specifies the anchoring position of the txBody within the shape. If this attribute is
    /// omitted, then a value of t, or top is implied.
    pub anchor: Option<TextAnchoringType>,
    /// Specifies the centering of the text box. The way it works fundamentally is to determine
    /// the smallest possible "bounds box" for the text and then to center that "bounds box"
    /// accordingly. This is different than paragraph alignment, which aligns the text within the
    /// "bounds box" for the text. This flag is compatible with all of the different kinds of
    /// anchoring. If this attribute is omitted, then a value of 0 or false is implied.
    /// 
    /// # Example
    /// 
    /// The text within this shape has been both vertically centered with the anchor
    /// attribute and horizontally centered with the anchorCtr attribute.
    /// 
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr anchor="ctr" anchorCtr="1" … />
    ///   …
    /// </p:txBody>
    /// ```
    pub anchor_center: Option<bool>,
    /// Forces the text to be rendered anti-aliased regardless of the font size. Certain fonts can
    /// appear grainy around their edges unless they are anti-aliased. Therefore this attribute
    /// allows for the specifying of which bodies of text should always be anti-aliased and which
    /// ones should not. If this attribute is omitted, then a value of 0 or false is implied.
    pub force_antialias: Option<bool>,
    /// Specifies whether text should remain upright, regardless of the transform applied to it
    /// and the accompanying shape transform. If this attribute is omitted, then a value of 0, or
    /// false is implied.
    pub upright: Option<bool>,
    /// Specifies that the line spacing for this text body is decided in a simplistic manner using
    /// the font scene. If this attribute is omitted, a value of 0 or false is implied.
    pub compatible_line_spacing: Option<bool>,
    /// This element specifies when a preset geometric shape should be used to transform a piece of text. This
    /// operation is known formally as a text warp. The generating application should be able to render all preset
    /// geometries enumerated in the TextShapeType list.
    /// 
    /// Using any of the presets listed under the ST_TextShapeType list below it is possible to apply a text warp to a run
    /// of DrawingML text via the following steps.
    /// 
    /// If you look at any of the text warps in the file format you notice that each consists of two paths. This
    /// corresponds to a top path (first one specified) and a bottom path (second one specified). Now the top path and
    /// the bottom path represent the top line and base line that the text needs to be warped to. This is done in the
    /// following way:
    /// 
    /// 1. Compute the rectangle that the unwarped text resides in. (tightest possible rectangle around text, no
    ///    white space except for “space characters”)
    /// 2. Take each of the quadratic and cubic Bezier curves that are used to calculate the original character and
    ///    change their end points and control points by the following method…
    /// 3. Move a vertical line horizontally along the original text rectangle and find the horizontal percentage that
    ///    a given end point or control point lives at. (.5 for the middle for instance)
    /// 4. Now do the same thing for this point vertically. Find the vertical percentage that this point lives at with
    ///    the top and bottom of this text rectangle being the respective top and bottom bounds. (0.0 and 1.0
    ///    respectively)
    /// 5. Now that we have the percentages for a given point in a Bezier equation we can map that to the new
    ///    point in the warped text environment.
    /// 6. Going back to the top and bottom paths specified in the file format we can take these and flatten them
    ///    out to a straight arc (top and bottom might be different lengths)
    /// 7. After they are straight we can measure them both horizontally to find the same percentage point that
    ///    we found within the original text rectangle. (0.5 let’s say)
    /// 8. So then we measure 50% along the top path and 50% along the bottom path, putting the paths back to
    ///    their original curvy shapes.
    /// 9. Once we have these two points we can draw a line between them that serves as our vertical line in the
    ///    original text rectangle (This might not be truly vertical as 50% on the top does not always line up
    ///    with 50% on the bottom. end)
    /// 10. Taking this new line we then follow it from top to bottom the vertical percentage amount that we got
    ///     from step 4.
    /// 11. This is then the new point that should be used in place of the old point in the original text rectangle.
    /// 12. We then continue doing these same steps for each of the end points and control points within the body
    ///     of text. (is applied to a whole body of text only)
    /// 
    /// # Xml example
    /// 
    /// Consider the case where the user wishes to accent a piece of text by warping it's shape. For this to
    /// occur a preset shape is chosen from the TextShapeType list and applied to the entire body of text.
    /// 
    /// ```xml
    /// <p:sp>
    ///   <p:txBody>
    ///     <a:bodyPr wrap="none" rtlCol="0">
    ///       <a:prstTxWarp prst="textInflate">
    ///       </a:prstTxWarp>
    ///       <a:spAutoFit/>
    ///     </a:bodyPr>
    ///     <a:lstStyle/>
    ///     <a:p>
    ///       …
    ///       <a:t>Sample Text</a:t>
    ///       …
    ///     </a:p>
    ///   </p:txBody>
    /// </p:sp>
    /// ```
    /// 
    /// # Note
    /// 
    /// Horizontal percentages begin at 0.0 and continue to 1.0, left to right. Vertical percentages begin at 0.0
    /// and continue to 1.0, top to bottom.
    /// 
    /// Since this is a shape it does have both a shape coordinate system and a path coordinate system.
    pub preset_text_warp: Option<Box<PresetTextShape>>,
    pub auto_fit_type: Option<TextAutoFit>,
    // TODO implement
    //pub scene_3d: Option<Scene3D>,
    //pub text_3d: Option<Text3D>,
}

impl TextBodyProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "rot" => instance.rotate_angle = Some(value.parse()?),
                "spcFirstLastPara" => instance.paragraph_spacing = Some(parse_xml_bool(value)?),
                "vertOverflow" => instance.vertical_overflow = Some(value.parse()?),
                "horzOverflow" => instance.horizontal_overflow = Some(value.parse()?),
                "vert" => instance.vertical_type = Some(value.parse()?),
                "wrap" => instance.wrap_type = Some(value.parse()?),
                "lIns" => instance.left_inset = Some(value.parse()?),
                "tIns" => instance.top_inset = Some(value.parse()?),
                "rIns" => instance.right_inset = Some(value.parse()?),
                "bIns" => instance.bottom_inset = Some(value.parse()?),
                "numCol" => instance.column_count = Some(value.parse()?),
                "spcCol" => instance.space_between_columns = Some(value.parse()?),
                "rtlCol" => instance.rtl_columns = Some(parse_xml_bool(value)?),
                "fromWordArt" => instance.is_from_word_art = Some(parse_xml_bool(value)?),
                "anchor" => instance.anchor = Some(value.parse()?),
                "anchorCtr" => instance.anchor_center = Some(parse_xml_bool(value)?),
                "forceAA" => instance.force_antialias = Some(parse_xml_bool(value)?),
                "upright" => instance.upright = Some(parse_xml_bool(value)?),
                "compatLnSpc" => instance.compatible_line_spacing = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            let child_local_name = child_node.local_name();
            if TextAutoFit::is_choice_member(child_local_name) {
                instance.auto_fit_type = Some(TextAutoFit::from_xml_element(child_node)?);
            } else if child_local_name == "prstTxWarp" {
                instance.preset_text_warp = Some(Box::new(PresetTextShape::from_xml_element(child_node)?));
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub enum TextAutoFit {
    /// This element specifies that text within the text body should not be auto-fit to the bounding box. Auto-fitting is
    /// when text within a text box is scaled in order to remain inside the text box. If this element is omitted, then
    /// noAutofit or auto-fit off is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider a text box where the user wishes to have the text extend outside the bounding box. The
    /// following DrawingML would describe this.
    /// ```xml
    /// <p:txBody>
    ///   <a:bodyPr wrap="none" rtlCol="0">
    ///     <a:noAutofit/>
    ///   </a:bodyPr>
    ///   <a:p>
    ///     …
    ///     <a:t>Some text</a:t>
    ///     …
    ///   </a:p>
    /// </p:txBody>
    /// ```
    NoAutoFit,

    /// This element specifies that text within the text body should be normally auto-fit to the bounding box. Autofitting
    /// is when text within a text box is scaled in order to remain inside the text box. If this element is omitted,
    /// then noAutofit or auto-fit off is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the situation where a user is building a diagram and needs to have the text for each shape
    /// that they are using stay within the bounds of the shape. An easy way this might be done is by using
    /// normAutofit. The following DrawingML illustrates how this might be accomplished.
    /// ```xml
    /// <p:sp>
    ///   <p:txBody>
    ///     <a:bodyPr rtlCol="0" anchor="ctr">
    ///       <a:normAutofit fontScale="92.000%" lnSpcReduction="20.000%"/>
    ///     </a:bodyPr>
    ///     …
    ///     <a:p>
    ///       …
    ///       <a:t>Diagram Object 1</a:t>
    ///       …
    ///     </a:p>
    ///   </p:txBody>
    /// </p:sp>
    /// <p:sp>
    ///   <p:txBody>
    ///     <a:bodyPr rtlCol="0" anchor="ctr">
    ///       <a:normAutofit fontScale="92.000%" lnSpcReduction="20.000%"/>
    ///     </a:bodyPr>
    ///     …
    ///     <a:p>
    ///       …
    ///       <a:t>Diagram Object 2</a:t>
    ///       …
    ///     </a:p>
    ///   </p:txBody>
    /// </p:sp>
    /// ```
    /// 
    /// In the above example there are two shapes that have normAutofit turned on so that when the user types more
    /// text within the shape that the text actually resizes to accommodate the new data. For the application to know
    /// how and to what degree the text should be resized two attributes are set for the auto-fit resize logic.
    NormalAutoFit(TextNormalAutoFit),

    /// This element specifies that a shape should be auto-fit to fully contain the text described within it. Auto-fitting is
    /// when text within a shape is scaled in order to contain all the text inside. If this element is omitted, then
    /// NoAutoFit or auto-fit off is implied.
    /// 
    /// # Xml example
    /// 
    /// Consider the situation where a user is building a diagram and needs to have the text for each shape
    /// that they are using stay within the bounds of the shape. An easy way this might be done is by using ShapeAutoFit.
    /// The following DrawingML illustrates how this might be accomplished.
    /// 
    /// ```xml
    /// <p:sp>
    ///   <p:txBody>
    ///     <a:bodyPr rtlCol="0" anchor="ctr">
    ///       <a:spAutoFit/>
    ///     </a:bodyPr>
    ///     …
    ///     <a:p>
    ///       …
    ///       <a:t>Diagram Object 1</a:t>
    ///       …
    ///     </a:p>
    ///   </p:txBody>
    /// </p:sp>
    /// <p:sp>
    ///   <p:txBody>
    ///     <a:bodyPr rtlCol="0" anchor="ctr">
    ///       <a:spAutoFit/>
    ///     </a:bodyPr>
    ///     …
    ///     <a:p>
    ///       …
    ///       <a:t>Diagram Object 2</a:t>
    ///       …
    ///     </a:p>
    ///   </p:txBody>
    /// </p:sp>
    /// ```
    /// 
    /// In the above example there are two shapes that have ShapeAutoFit turned on so that when the user types more
    /// text within the shape that the shape actually resizes to accommodate the new data.
    ShapeAutoFit,
}

impl TextAutoFit {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "noAutofit" | "normAutofit" | "spAutoFit" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "noAutofit" => Ok(TextAutoFit::NoAutoFit),
            "normAutofit" => Ok(TextAutoFit::NormalAutoFit(TextNormalAutoFit::from_xml_element(
                xml_node,
            )?)),
            "spAutoFit" => Ok(TextAutoFit::ShapeAutoFit),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_TextAutofit").into()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextNormalAutoFit {
    /// Specifies the percentage of the original font size to which each run in the text body is
    /// scaled. In order to auto-fit text within a bounding box it is sometimes necessary to
    /// decrease the font size by a certain percentage. Using this attribute the font within a text
    /// box can be scaled based on the value provided. A value of 100% scales the text to 100%,
    /// while a value of 1% scales the text to 1%. If this attribute is omitted, then a value of 100%
    /// is implied.
    /// 
    /// Defaults to 100000
    pub font_scale: Option<TextFontScalePercent>,

    /// Specifies the percentage amount by which the line spacing of each paragraph in the text
    /// body is reduced. The reduction is applied by subtracting it from the original line spacing
    /// value. Using this attribute the vertical spacing between the lines of text can be scaled by
    /// a percent amount. A value of 100% reduces the line spacing by 100%, while a value of 1%
    /// reduces the line spacing by one percent. If this attribute is omitted, then a value of 0% is
    /// implied.
    /// 
    /// Defaults to 0
    /// 
    /// # Note
    /// 
    /// This attribute applies only to paragraphs with percentage line spacing.
    pub line_spacing_reduction: Option<TextSpacingPercent>,
}

impl TextNormalAutoFit {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut font_scale = None;
        let mut line_spacing_reduction = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "fontScale" => font_scale = Some(value.parse::<TextFontScalePercent>()?),
                "lnSpcReduction" => line_spacing_reduction = Some(value.parse::<TextSpacingPercent>()?),
                _ => (),
            }
        }

        Ok(Self {
            font_scale,
            line_spacing_reduction,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PresetTextShape {
    /// Specifies the preset geometry that is used for a shape warp on a piece of text. This preset
    /// can have any of the values in the enumerated list for TextShapeType. This attribute
    /// is required in order for a text warp to be rendered.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:sp>
    ///   <p:txBody>
    ///     <a:bodyPr wrap="none" rtlCol="0">
    ///       <a:prstTxWarp prst="textInflate">
    ///         </a:prstTxWarp>
    ///       <a:spAutoFit/>
    ///     </a:bodyPr>
    ///     <a:lstStyle/>
    ///     <a:p>
    ///       …
    ///       <a:t>Sample Text</a:t>
    ///       …
    ///     </a:p>
    ///   </p:txBody>
    /// </p:sp>
    /// ```
    /// 
    /// In the above example a preset text shape geometry has been used to define the warping
    /// shape. The shape utilized here is the sun shape.
    pub preset: TextShapeType,
    pub adjust_value_list: Vec<GeomGuide>,
}

impl PresetTextShape {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let preset_attr = xml_node
            .attribute("prst")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "prst"))?;
        let preset = preset_attr.parse()?;

        let mut adjust_value_list = Vec::new();
        if let Some(node) = xml_node.child_nodes.get(0) {
            for av_node in &node.child_nodes {
                adjust_value_list.push(GeomGuide::from_xml_element(av_node)?);
            }
        }

        Ok(Self {
            preset,
            adjust_value_list,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FontScheme {
    /// The name of the font scheme shown in the user interface.
    pub name: String,
    /// This element defines the set of major fonts which are to be used under different languages or locals.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <majorFont>
    /// <latin typeface="Calibri"/>
    ///   <ea typeface="Arial"/>
    ///   <cs typeface="Arial"/>
    ///   <font script="Jpan" typeface="MS Pゴシック "/>
    ///   <font script="Hang" typeface="HY중고딕"/>
    ///   <font script="Hans" typeface="隶 书"/>
    ///   <font script="Hant" typeface="微軟正黑體 "/>
    ///   <font script="Arab" typeface="Traditional Arabic"/>
    ///   <font script="Hebr" typeface="Arial"/>
    ///   <font script="Thai" typeface="Cordia New"/>
    ///   <font script="Ethi" typeface="Nyala"/>
    ///   <font script="Beng" typeface="Vrinda"/>
    ///   <font script="Gujr" typeface="Shruti"/>
    ///   <font script="Khmr" typeface="DaunPenh"/>
    ///   <font script="Knda" typeface="Tunga"/>
    /// </majorFont>
    /// ```
    /// 
    /// In this example, we see the latin, east asian, and complex script fonts defined along with many fonts for
    /// different locals.
    pub major_font: Box<FontCollection>,
    /// This element defines the set of minor fonts that are to be used under different languages or locals.
    /// 
    /// ```xml
    /// <minorFont>
    ///   <latin typeface="Calibri"/>
    ///   <ea typeface="Arial"/>
    ///   <cs typeface="Arial"/>
    ///   <font script="Jpan" typeface="MS Pゴシック "/>
    ///   <font script="Hang" typeface="HY중고딕"/>
    ///   <font script="Hans" typeface="隶 书"/>
    ///   <font script="Hant" typeface="微軟正黑體 "/>
    ///   <font script="Arab" typeface="Traditional Arabic"/>
    ///   <font script="Hebr" typeface="Arial"/>
    ///   <font script="Thai" typeface="Cordia New"/>
    ///   <font script="Ethi" typeface="Nyala"/>
    ///   <font script="Beng" typeface="Vrinda"/>
    ///   <font script="Gujr" typeface="Shruti"/>
    ///   <font script="Khmr" typeface="DaunPenh"/>
    ///   <font script="Knda" typeface="Tunga"/>
    /// </minorFont>
    /// ```
    /// 
    /// In this example, we see the latin, east asian, and complex script fonts defined along with many fonts for
    /// different locals.
    pub minor_font: Box<FontCollection>,
}

impl FontScheme {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let name_attr = xml_node
            .attribute("name")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "name"))?;
        let name = name_attr.clone();
        let mut major_font = None;
        let mut minor_font = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "majorFont" => major_font = Some(Box::new(FontCollection::from_xml_element(child_node)?)),
                "minorFont" => minor_font = Some(Box::new(FontCollection::from_xml_element(child_node)?)),
                _ => (),
            }
        }

        let major_font = major_font.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "majorFont"))?;
        let minor_font = minor_font.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "minorFont"))?;

        Ok(Self {
            name,
            major_font,
            minor_font,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FontCollection {
    pub latin: TextFont,
    pub east_asian: TextFont,
    pub complex_script: TextFont,
    /// This element defines a list of font within the styles area of DrawingML. A font is defined by a script along
    /// with a typeface.
    pub supplemental_font_list: Vec<SupplementalFont>,
}

impl FontCollection {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut opt_latin = None;
        let mut opt_ea = None;
        let mut opt_cs = None;
        let mut supplemental_font_list = Vec::new();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "latin" => opt_latin = Some(TextFont::from_xml_element(child_node)?),
                "ea" => opt_ea = Some(TextFont::from_xml_element(child_node)?),
                "cs" => opt_cs = Some(TextFont::from_xml_element(child_node)?),
                "font" => supplemental_font_list.push(SupplementalFont::from_xml_element(child_node)?),
                _ => (),
            }
        }

        let latin = opt_latin.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "latin"))?;
        let east_asian = opt_ea.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "ea"))?;
        let complex_script = opt_cs.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "cs"))?;

        Ok(Self {
            latin,
            east_asian,
            complex_script,
            supplemental_font_list,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NonVisualDrawingProps {
    /// Specifies a unique identifier for the current DrawingML object within the current
    /// document. This ID can be used to assist in uniquely identifying this object so that it can
    /// be referred to by other parts of the document.
    /// 
    /// If multiple objects within the same document share the same id attribute value, then the
    /// document shall be considered non-conformant.
    /// 
    /// # Example
    /// 
    /// Consider a DrawingML object defined as follows:
    /// 
    /// <… id="10" … >
    /// 
    /// The id attribute has a value of 10, which is the unique identifier for this DrawingML
    /// object.
    pub id: DrawingElementId,
    /// Specifies the name of the object.
    /// 
    /// # Note
    /// 
    /// Typically, this is used to store the original file name of a picture object.
    /// 
    /// # Example
    /// 
    /// Consider a DrawingML object defined as follows:
    /// 
    /// < … name="foo.jpg" >
    /// 
    /// The name attribute has a value of foo.jpg, which is the name of this DrawingML object.
    pub name: String,
    /// Specifies alternative text for the current DrawingML object, for use by assistive
    /// technologies or applications which do not display the current object.
    /// 
    /// If this element is omitted, then no alternative text is present for the parent object.
    /// 
    /// # Example
    /// 
    /// Consider a DrawingML object defined as follows:
    /// 
    /// <… descr="A picture of a bowl of fruit">
    /// 
    /// The descr attribute contains alternative text which can be used in place of the actual
    /// DrawingML object.
    pub description: Option<String>,
    /// Specifies whether this DrawingML object is displayed. When a DrawingML object is
    /// displayed within a document, that object can be hidden (i.e., present, but not visible).
    /// This attribute determines whether the object is rendered or made hidden. [Note: An
    /// application can have settings which allow this object to be viewed. end note]
    /// 
    /// If this attribute is omitted, then the parent DrawingML object shall be displayed (i.e., not
    /// hidden).
    /// 
    /// Defaults to false
    /// 
    /// # Example
    /// 
    /// Consider an inline DrawingML object which must be hidden within the
    /// document's content. This setting would be specified as follows:
    /// 
    /// <… hidden="true" />
    /// 
    /// The hidden attribute has a value of true, which specifies that the DrawingML object is
    /// hidden and not displayed when the document is displayed.
    pub hidden: Option<bool>,
    /// Specifies the title (caption) of the current DrawingML object.
    /// 
    /// If this attribute is omitted, then no title text is present for the parent object.
    /// 
    /// # Example
    /// 
    /// Consider a DrawingML object defined as follows:
    /// 
    /// <… title="Process Flow Diagram">
    pub title: Option<String>,
    pub hyperlink_click: Option<Box<Hyperlink>>,
    /// This element specifies the hyperlink information to be activated when the user's mouse is hovered over the
    /// corresponding object. The operation of the hyperlink is to have the specified action be activated when the
    /// mouse of the user hovers over the object. When this action is activated then additional attributes can be used to
    /// specify other tasks that should be performed along with the action.
    pub hyperlink_hover: Option<Box<Hyperlink>>,
}

impl NonVisualDrawingProps {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut opt_id = None;
        let mut opt_name = None;
        let mut description = None;
        let mut hidden = None;
        let mut title = None;
        let mut hyperlink_click = None;
        let mut hyperlink_hover = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "id" => opt_id = Some(value.parse::<DrawingElementId>()?),
                "name" => opt_name = Some(value.clone()),
                "descr" => description = Some(value.clone()),
                "hidden" => hidden = Some(parse_xml_bool(value)?),
                "title" => title = Some(value.clone()),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "hlinkClick" => hyperlink_click = Some(Box::new(Hyperlink::from_xml_element(child_node)?)),
                "hlinkHover" => hyperlink_hover = Some(Box::new(Hyperlink::from_xml_element(child_node)?)),
                _ => (),
            }
        }

        let id = opt_id.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "id"))?;
        let name = opt_name.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "name"))?;

        Ok(Self {
            id,
            name,
            description,
            hidden,
            title,
            hyperlink_click,
            hyperlink_hover,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct Locking {
    /// Specifies that the generating application should not allow shape grouping for the
    /// corresponding connection shape. That is it cannot be combined within other shapes to
    /// form a group of shapes. If this attribute is not specified, then a value of false is assumed.
    /// 
    /// Defaults to false
    pub no_grouping: Option<bool>,
    /// Specifies that the generating application should not allow selecting of the corresponding
    /// connection shape. That means also that no picture, shapes or text attached to this
    /// connection shape can be selected if this attribute has been specified. If this attribute is
    /// not specified, then a value of false is assumed.
    /// 
    /// Defaults to false
    pub no_select: Option<bool>,
    /// Specifies that the generating application should not allow shape rotation changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_rotate: Option<bool>,
    /// Specifies that the generating application should not allow aspect ratio changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_change_aspect_ratio: Option<bool>,
    /// Specifies that the generating application should not allow position changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_move: Option<bool>,
    /// Specifies that the generating application should not allow size changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_resize: Option<bool>,
    /// Specifies that the generating application should not allow shape point changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_edit_points: Option<bool>,
    /// Specifies that the generating application should not show adjust handles for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_adjust_handles: Option<bool>,
    /// Specifies that the generating application should not allow arrowhead changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_change_arrowheads: Option<bool>,
    /// Specifies that the generating application should not allow shape type changes for the
    /// corresponding connection shape. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_change_shape_type: Option<bool>,
}

impl Locking {
    pub fn try_attribute_parse(&mut self, attr: &str, value: &str) -> Result<()> {
        match attr {
            "noGrp" => self.no_grouping = Some(parse_xml_bool(value)?),
            "noSelect" => self.no_select = Some(parse_xml_bool(value)?),
            "noRot" => self.no_rotate = Some(parse_xml_bool(value)?),
            "noChangeAspect" => self.no_change_aspect_ratio = Some(parse_xml_bool(value)?),
            "noMove" => self.no_move = Some(parse_xml_bool(value)?),
            "noResize" => self.no_resize = Some(parse_xml_bool(value)?),
            "noEditPoints" => self.no_edit_points = Some(parse_xml_bool(value)?),
            "noAdjustHandles" => self.no_adjust_handles = Some(parse_xml_bool(value)?),
            "noChangeArrowheads" => self.no_change_arrowheads = Some(parse_xml_bool(value)?),
            "noChangeShapeType" => self.no_change_shape_type = Some(parse_xml_bool(value)?),
            _ => (),
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ShapeLocking {
    pub locking: Locking,
    pub no_text_edit: Option<bool>, // false
}

impl ShapeLocking {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut locking: Locking = Default::default();
        let mut no_text_edit = None;

        for (attr, value) in &xml_node.attributes {
            if attr.as_str() == "noTextEdit" {
                no_text_edit = Some(parse_xml_bool(value)?);
            } else {
                locking.try_attribute_parse(attr, value)?;
            }
        }

        Ok(Self { locking, no_text_edit })
    }
}

#[derive(Default, Debug, Clone)]
pub struct GroupLocking {
    pub no_grouping: Option<bool>,            // false
    pub no_ungrouping: Option<bool>,          // false
    pub no_select: Option<bool>,              // false
    pub no_rotate: Option<bool>,              // false
    pub no_change_aspect_ratio: Option<bool>, // false
    pub no_move: Option<bool>,                // false
    pub no_resize: Option<bool>,              // false
}

impl GroupLocking {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "noGrp" => instance.no_grouping = Some(parse_xml_bool(value)?),
                "noUngrp" => instance.no_ungrouping = Some(parse_xml_bool(value)?),
                "noSelect" => instance.no_select = Some(parse_xml_bool(value)?),
                "noRot" => instance.no_rotate = Some(parse_xml_bool(value)?),
                "noChangeAspect" => instance.no_change_aspect_ratio = Some(parse_xml_bool(value)?),
                "noMove" => instance.no_move = Some(parse_xml_bool(value)?),
                "noResize" => instance.no_resize = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct GraphicalObjectFrameLocking {
    /// Specifies that the generating application should not allow shape grouping for the
    /// corresponding graphic frame. That is it cannot be combined within other shapes to form
    /// a group of shapes. If this attribute is not specified, then a value of false is assumed.
    /// 
    /// Defaults to false
    pub no_grouping: Option<bool>,
    /// Specifies that the generating application should not allow selecting of objects within the
    /// corresponding graphic frame but allow selecting of the graphic frame itself. If this
    /// attribute is not specified, then a value of false is assumed.
    /// 
    /// Defaults to false
    pub no_drilldown: Option<bool>,
    /// Specifies that the generating application should not allow selecting of the corresponding
    /// picture. That means also that no picture, shapes or text attached to this picture can be
    /// selected if this attribute has been specified. If this attribute is not specified, then a value
    /// of false is assumed.
    /// 
    /// Defaults to false
    /// 
    /// # Note
    /// 
    /// If this attribute is specified to be true then the graphic frame cannot be selected
    /// and the objects within the graphic frame cannot be selected as well. That is the entire
    /// graphic frame including all sub-parts are considered un-selectable.
    pub no_select: Option<bool>,
    /// Specifies that the generating application should not allow aspect ratio changes for the
    /// corresponding graphic frame. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_change_aspect: Option<bool>,
    /// Specifies that the corresponding graphic frame cannot be moved. Objects that reside
    /// within the graphic frame can still be moved unless they also have been locked. If this
    /// attribute is not specified, then a value of false is assumed.
    /// 
    /// Defaults to false
    pub no_move: Option<bool>,
    /// Specifies that the generating application should not allow size changes for the
    /// corresponding graphic frame. If this attribute is not specified, then a value of false is
    /// assumed.
    /// 
    /// Defaults to false
    pub no_resize: Option<bool>,
}

impl GraphicalObjectFrameLocking {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "noGrp" => instance.no_grouping = Some(parse_xml_bool(value)?),
                "noDrilldown" => instance.no_drilldown = Some(parse_xml_bool(value)?),
                "noSelect" => instance.no_select = Some(parse_xml_bool(value)?),
                "noChangeAspect" => instance.no_change_aspect = Some(parse_xml_bool(value)?),
                "noMove" => instance.no_move = Some(parse_xml_bool(value)?),
                "noResize" => instance.no_resize = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct ConnectorLocking {
    pub locking: Locking,
}

impl ConnectorLocking {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut locking: Locking = Default::default();

        for (attr, value) in &xml_node.attributes {
            locking.try_attribute_parse(attr, value)?;
        }

        Ok(Self { locking })
    }
}

#[derive(Debug, Clone)]
pub struct PictureLocking {
    pub locking: Locking,
    pub no_crop: Option<bool>, // false
}

impl PictureLocking {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut locking: Locking = Default::default();
        let mut no_crop = None;
        for (attr, value) in &xml_node.attributes {
            if attr.as_str() == "noCrop" {
                no_crop = Some(parse_xml_bool(value)?);
            } else {
                locking.try_attribute_parse(attr, value)?;
            }
        }

        Ok(Self { locking, no_crop })
    }
}

#[derive(Default, Debug, Clone)]
pub struct NonVisualDrawingShapeProps {
    pub is_text_box: Option<bool>, // false
    pub shape_locks: Option<ShapeLocking>,
}

impl NonVisualDrawingShapeProps {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let is_text_box = match xml_node.attribute("txBox") {
            Some(attr) => Some(parse_xml_bool(attr)?),
            None => None,
        };

        let shape_locks = match xml_node.child_nodes.get(0) {
            Some(sp_lock_node) => Some(ShapeLocking::from_xml_element(sp_lock_node)?),
            None => None,
        };

        Ok(Self {
            is_text_box,
            shape_locks,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct NonVisualGroupDrawingShapeProps {
    pub locks: Option<GroupLocking>,
}

impl NonVisualGroupDrawingShapeProps {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let locks = match xml_node.child_nodes.get(0) {
            Some(node) => Some(GroupLocking::from_xml_element(node)?),
            None => None,
        };

        Ok(Self { locks })
    }
}

#[derive(Default, Debug, Clone)]
pub struct NonVisualGraphicFrameProperties {
    /// This element specifies all locking properties for a graphic frame. These properties inform the generating
    /// application about specific properties that have been previously locked and thus should not be changed.
    pub graphic_frame_locks: Option<GraphicalObjectFrameLocking>,
}

impl NonVisualGraphicFrameProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let graphic_frame_locks = match xml_node.child_nodes.get(0) {
            Some(node) => {
                if node.local_name() == "graphicFrameLocks" {
                    Some(GraphicalObjectFrameLocking::from_xml_element(node)?)
                } else {
                    None
                }
            }
            None => None,
        };

        Ok(Self { graphic_frame_locks })
    }
}

#[derive(Default, Debug, Clone)]
pub struct NonVisualConnectorProperties {
    /// This element specifies all locking properties for a connection shape. These properties inform the generating
    /// application about specific properties that have been previously locked and thus should not be changed.
    pub connector_locks: Option<ConnectorLocking>,
    /// This element specifies the starting connection that should be made by the corresponding connector shape. This
    /// connects the head of the connector to the first shape.
    pub start_connection: Option<Connection>,
    /// This element specifies the ending connection that should be made by the corresponding connector shape. This
    /// connects the end tail of the connector to the final destination shape.
    pub end_connection: Option<Connection>,
}

impl NonVisualConnectorProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "cxnSpLocks" => instance.connector_locks = Some(ConnectorLocking::from_xml_element(child_node)?),
                "stCxn" => instance.start_connection = Some(Connection::from_xml_element(child_node)?),
                "endCxn" => instance.end_connection = Some(Connection::from_xml_element(child_node)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct NonVisualPictureProperties {
    /// Specifies if the user interface should show the resizing of the picture based on the
    /// picture's current size or its original size. If this attribute is set to true, then scaling is
    /// relative to the original picture size as opposed to the current picture size.
    /// 
    /// Defaults to true
    /// 
    /// # Example
    /// 
    /// Consider the case where a picture has been resized within a document and is
    /// now 50% of the originally inserted picture size. Now if the user chooses to make a later
    /// adjustment to the size of this picture within the generating application, then the value of
    /// this attribute should be checked.
    /// 
    /// If this attribute is set to true then a value of 50% is shown. Similarly, if this attribute is set
    /// to false, then a value of 100% should be shown because the picture has not yet been
    /// resized from its current (smaller) size.
    pub prefer_relative_resize: Option<bool>,
    pub picture_locks: Option<PictureLocking>,
}

impl NonVisualPictureProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let prefer_relative_resize = match xml_node.attribute("preferRelativeResize") {
            Some(attr) => Some(parse_xml_bool(attr)?),
            None => None,
        };

        let picture_locks = match xml_node.child_nodes.get(0) {
            Some(node) => Some(PictureLocking::from_xml_element(node)?),
            None => None,
        };

        Ok(Self {
            prefer_relative_resize,
            picture_locks,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    /// Specifies the id of the shape to make the final connection to.
    pub id: DrawingElementId,
    /// Specifies the index into the connection site table of the final connection shape. That is
    /// there are many connection sites on a shape and it shall be specified which connection
    /// site the corresponding connector shape should connect to.
    pub shape_index: u32,
}

impl Connection {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut id = None;
        let mut shape_index = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "id" => id = Some(value.parse()?),
                "idx" => shape_index = Some(value.parse()?),
                _ => (),
            }
        }

        let id = id.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "id"))?;
        let shape_index = shape_index.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "idx"))?;

        Ok(Self { id, shape_index })
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedWAVAudioFile {
    /// Specifies the identification information for an embedded audio file. This attribute is used
    /// to specify the location of an object that resides locally within the file.
    pub embed_rel_id: RelationshipId,
    /// Specifies the original name or given short name for the corresponding sound. This is used
    /// to distinguish this sound from others by providing a human readable name for the
    /// attached sound should the user need to identify the sound among others within the UI.
    pub name: Option<String>,
    //pub built_in: Option<bool>, // false
}

impl EmbeddedWAVAudioFile {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut embed_rel_id = None;
        let mut name = None;
        //let mut built_in = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "r:embed" => embed_rel_id = Some(value.clone()),
                "name" => name = Some(value.clone()),
                _ => (),
            }
        }

        let embed_rel_id = embed_rel_id.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "r:embed"))?;

        Ok(Self { embed_rel_id, name })
    }
}

#[derive(Debug, Clone)]
pub struct AudioCDTime {
    /// Specifies which track of the CD this Audio begins playing on. This attribute is required and
    /// cannot be omitted.
    pub track: u8,
    /// Specifies the time in seconds that the CD Audio should be started at.
    /// 
    /// Defaults to 0
    pub time: Option<u32>,
}

impl AudioCDTime {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut track = None;
        let mut time = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "track" => track = Some(value.parse()?),
                "time" => time = Some(value.parse()?),
                _ => (),
            }
        }

        let track = track.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "track"))?;

        Ok(Self { track, time })
    }
}

#[derive(Debug, Clone)]
pub struct AudioCD {
    /// This element specifies the start point for a CD Audio sound element. Encompassed within this element are the
    /// time and track at which the sound should begin its playback. This element is used in conjunction with an Audio
    /// End Time element to specify the time span for an entire audioCD sound element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:audioCd>
    ///   <a:st track="1" time="2"/>
    ///   <a:end track="3" time="65"/>
    /// </a:audioCd>
    /// ```
    /// 
    /// In the above example, the audioCD sound element shown specifies for a portion of audio spanning from 2
    /// seconds into the first track to 1 minute, 5 seconds into the third track.
    pub start_time: AudioCDTime,
    /// This element specifies the end point for a CD Audio sound element. Encompassed within this element are the
    /// time and track at which the sound should halt its playback. This element is used in conjunction with an Audio
    /// Start Time element to specify the time span for an entire audioCD sound element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:audioCd>
    ///   <a:st track="1" time="2"/>
    ///   <a:end track="3" time="65"/>
    /// </a:audioCd>
    /// ```
    /// 
    /// In the above example, the audioCD sound element shown specifies for a portion of audio spanning from 2
    /// seconds into the first track to 1 minute, 5 seconds into the third track.
    pub end_time: AudioCDTime,
}

impl AudioCD {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut start_time = None;
        let mut end_time = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "st" => start_time = Some(AudioCDTime::from_xml_element(child_node)?),
                "end" => end_time = Some(AudioCDTime::from_xml_element(child_node)?),
                _ => (),
            }
        }

        let start_time = start_time.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "st"))?;
        let end_time = end_time.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "end"))?;

        Ok(Self { start_time, end_time })
    }
}

#[derive(Debug, Clone)]
pub struct AudioFile {
    /// Specifies the identification information for a linked object. This attribute is used to
    /// specify the location of an object that does not reside within this file.
    pub link: RelationshipId,
    /// Specifies the content type for the external file that is referenced by this element. Content
    /// types define a media type, a subtype, and an optional set of parameters, as defined in
    /// Part 2. If a rendering application cannot process external content of the content type
    /// specified, then the specified content can be ignored.
    /// 
    /// If this attribute is omitted, application should attempt to determine the content type by
    /// reading the contents of the relationship’s target.
    /// 
    /// Suggested audio types:
    /// * aiff
    /// * midi
    /// * ogg
    /// * mpeg
    /// 
    /// A producer that wants interoperability should use the following standard format:
    /// * audio
    /// * mpeg ISO
    /// * IEC 11172-3
    pub content_type: Option<String>,
}

impl AudioFile {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut link = None;
        let mut content_type = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "r:link" => link = Some(value.clone()),
                "contentType" => content_type = Some(value.clone()),
                _ => (),
            }
        }

        let link = link.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "r:link"))?;

        Ok(Self { link, content_type })
    }
}

#[derive(Debug, Clone)]
pub struct VideoFile {
    /// Specifies the identification information for a linked video file. This attribute is used to
    /// specify the location of an object that does not reside within this file.
    pub link: RelationshipId,
    /// Specifies the content type for the external file that is referenced by this element. Content
    /// types define a media type, a subtype, and an optional set of parameters, as defined in
    /// Part 2. If a rendering application cannot process external content of the content type
    /// specified, then the specified content can be ignored.
    /// 
    /// Suggested video formats:
    /// * avi
    /// * mpg
    /// * mpeg
    /// * ogg
    /// * quicktime
    /// * vc1
    /// 
    /// If this attribute is omitted, application should attempt to determine the content type by
    /// reading the contents of the relationship’s target.
    pub content_type: Option<String>,
}

impl VideoFile {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut link = None;
        let mut content_type = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "r:link" => link = Some(value.clone()),
                "contentType" => content_type = Some(value.clone()),
                _ => (),
            }
        }

        let link = link.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "r:link"))?;

        Ok(Self { link, content_type })
    }
}

#[derive(Debug, Clone)]
pub struct QuickTimeFile {
    /// Specifies the identification information for a linked object. This attribute is used to
    /// specify the location of an object that does not reside within this file.
    pub link: RelationshipId,
}

impl QuickTimeFile {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let link_attr = xml_node
            .attribute("r:link")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "r:link"))?;
        let link = link_attr.clone();

        Ok(Self { link })
    }
}

#[derive(Debug, Clone)]
pub enum Media {
    /// This element specifies the existence of Audio from a CD. This element is specified within the non-visual
    /// properties of an object. The audio shall be attached to an object as this is how it is represented within the
    /// document. The actual playing of the sound however is done within the timing node list that is specified under
    /// the timing element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:pic>
    ///   <p:nvPicPr>
    ///     <p:cNvPr id="7" name="Rectangle 6">
    ///       <a:hlinkClick r:id="" action="ppaction://media"/>
    ///     </p:cNvPr>
    ///     <p:cNvPicPr>
    ///       <a:picLocks noRot="1"/>
    ///     </p:cNvPicPr>
    ///     <p:nvPr>
    ///       <a:audioCd>
    ///         <a:st track="1"/>
    ///         <a:end track="3" time="65"/>
    ///       </a:audioCd>
    ///     </p:nvPr>
    ///   </p:nvPicPr>
    ///   ...
    /// </p:pic>
    /// ```
    /// 
    /// In the above example, we see that there is a single audioCD element attached to this picture. This picture is
    /// placed within the document just as a normal picture or shape would be. The id of this picture, namely 7 in this
    /// case, is used to refer to this audioCD element from within the timing node list. For this example we see that the
    /// audio for this CD starts playing at the 0 second mark on the first track and ends on the 1 minute 5 second mark
    /// of the third track.
    AudioCd(AudioCD),
    /// This element specifies the existence of an audio WAV file. This element is specified within the non-visual
    /// properties of an object. The audio shall be attached to an object as this is how it is represented within the
    /// document. The actual playing of the audio however is done within the timing node list that is specified under the
    /// timing element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:pic>
    ///   <p:nvPicPr>
    ///     <p:cNvPr id="7" name="Rectangle 6">
    ///       <a:hlinkClick r:id="" action="ppaction://media"/>
    ///     </p:cNvPr>
    ///     <p:cNvPicPr>
    ///       <a:picLocks noRot="1"/>
    ///     </p:cNvPicPr>
    ///     <p:nvPr>
    ///       <a:wavAudioFile r:embed="rId2"/>
    ///     </p:nvPr>
    ///   </p:nvPicPr>
    ///   ...
    /// </p:pic>
    /// ```
    /// 
    /// In the above example, we see that there is a single wavAudioFile element attached to this picture. This picture
    /// is placed within the document just as a normal picture or shape would be. The id of this picture, namely 7 in this
    /// case, is used to refer to this wavAudioFile element from within the timing node list. The Embedded relationship
    /// id is used to retrieve the actual audio file for playback purposes.
    WavAudioFile(EmbeddedWAVAudioFile),
    /// This element specifies the existence of an audio file. This element is specified within the non-visual properties of
    /// an object. The audio shall be attached to an object as this is how it is represented within the document. The
    /// actual playing of the audio however is done within the timing node list that is specified under the timing
    /// element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:pic>
    ///   <p:nvPicPr>
    ///     <p:cNvPr id="7" name="Rectangle 6">
    ///       <a:hlinkClick r:id="" action="ppaction://media"/>
    ///     </p:cNvPr>
    ///     <p:cNvPicPr>
    ///       <a:picLocks noRot="1"/>
    ///     </p:cNvPicPr>
    ///     <p:nvPr>
    ///       <a:audioFile r:link="rId1"/>
    ///     </p:nvPr>
    ///   </p:nvPicPr>
    ///   ...
    /// </p:pic>
    /// ```
    /// 
    /// In the above example, we see that there is a single audioFile element attached to this picture. This picture is
    /// placed within the document just as a normal picture or shape would be. The id of this picture, namely 7 in this
    /// case, is used to refer to this audioFile element from within the timing node list. The Linked relationship id is
    /// used to retrieve the actual audio file for playback purposes.
    AudioFile(AudioFile),
    /// This element specifies the existence of a video file. This element is specified within the non-visual properties of
    /// an object. The video shall be attached to an object as this is how it is represented within the document. The
    /// actual playing of the video however is done within the timing node list that is specified under the timing
    /// element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:pic>
    ///   <p:nvPicPr>
    ///     <p:cNvPr id="7" name="Rectangle 6">
    ///       <a:hlinkClick r:id="" action="ppaction://media"/>
    ///     </p:cNvPr>
    ///     <p:cNvPicPr>
    ///       <a:picLocks noRot="1"/>
    ///     </p:cNvPicPr>
    ///     <p:nvPr>
    ///       <a:videoFile r:link="rId1"/>
    ///     </p:nvPr>
    ///   </p:nvPicPr>
    ///   ...
    /// </p:pic>
    /// ```
    /// 
    /// In the above example, we see that there is a single videoFile element attached to this picture. This picture is
    /// placed within the document just as a normal picture or shape would be. The id of this picture, namely 7 in this
    /// case, is used to refer to this videoFile element from within the timing node list. The Linked relationship id is
    /// used to retrieve the actual video file for playback purposes.
    VideoFile(VideoFile),
    /// This element specifies the existence of a QuickTime file. This element is specified within the non-visual
    /// properties of an object. The QuickTime file shall be attached to an object as this is how it is represented
    /// within the document. The actual playing of the QuickTime however is done within the timing node list that is
    /// specified under the timing element.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:pic>
    ///   <p:nvPicPr>
    ///     <p:cNvPr id="7" name="Rectangle 6">
    ///       <a:hlinkClick r:id="" action="ppaction://media"/>
    ///     </p:cNvPr>
    ///     <p:cNvPicPr>
    ///       <a:picLocks noRot="1"/>
    ///     </p:cNvPicPr>
    ///     <p:nvPr>
    ///       <a:quickTimeFile r:link="rId1"/>
    ///     </p:nvPr>
    ///   </p:nvPicPr>
    ///   ...
    /// </p:pic>
    /// ```
    /// 
    /// In the above example, we see that there is a single quickTimeFile element attached to this picture. This picture
    /// is placed within the document just as a normal picture or shape would be. The id of this picture, namely 7 in this
    /// case, is used to refer to this quickTimeFile element from within the timing node list. The Linked relationship id
    /// is used to retrieve the actual video file for playback purposes.
    QuickTimeFile(QuickTimeFile),
}

impl Media {
    pub fn is_choice_member<T>(name: T) -> bool
    where
        T: AsRef<str>,
    {
        match name.as_ref() {
            "audioCd" | "wavAudioFile" | "audioFile" | "videoFile" | "quickTimeFile" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "audioCd" => Ok(Media::AudioCd(AudioCD::from_xml_element(xml_node)?)),
            "wavAudioFile" => Ok(Media::WavAudioFile(EmbeddedWAVAudioFile::from_xml_element(xml_node)?)),
            "audioFile" => Ok(Media::AudioFile(AudioFile::from_xml_element(xml_node)?)),
            "videoFile" => Ok(Media::VideoFile(VideoFile::from_xml_element(xml_node)?)),
            "quickTimeFile" => Ok(Media::QuickTimeFile(QuickTimeFile::from_xml_element(xml_node)?)),
            _ => Err(Box::new(NotGroupMemberError::new(xml_node.name.clone(), "EG_Media"))),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Transform2D {
    /// Specifies the rotation of the Graphic Frame. The units for which this attribute is specified
    /// in reside within the simple type definition referenced below.
    pub rotate_angle: Option<Angle>,
    /// Specifies a horizontal flip. When true, this attribute defines that the shape is flipped
    /// horizontally about the center of its bounding box.
    /// 
    /// Defaults to false
    pub flip_horizontal: Option<bool>,
    /// Specifies a vertical flip. When true, this attribute defines that the group is flipped
    /// vertically about the center of its bounding box.
    pub flip_vertical: Option<bool>,
    /// This element specifies the location of the bounding box of an object. Effects on an object are not included in this
    /// bounding box.
    pub offset: Option<Point2D>,
    /// This element specifies the size of the bounding box enclosing the referenced object.
    pub extents: Option<PositiveSize2D>,
}

impl Transform2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "rot" => instance.rotate_angle = Some(value.parse()?),
                "flipH" => instance.flip_horizontal = Some(parse_xml_bool(value)?),
                "flipV" => instance.flip_vertical = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "off" => instance.offset = Some(Point2D::from_xml_element(child_node)?),
                "ext" => instance.extents = Some(PositiveSize2D::from_xml_element(child_node)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct GroupTransform2D {
    pub rotate_angle: Option<Angle>,   // 0
    pub flip_horizontal: Option<bool>, // false
    pub flip_vertical: Option<bool>,   // false
    /// This element specifies the location of the bounding box of an object. Effects on an object are not included in this
    /// bounding box.
    pub offset: Option<Point2D>,
    /// This element specifies the size of the bounding box enclosing the referenced object.
    pub extents: Option<PositiveSize2D>,
    /// This element specifies the location of the child extents rectangle and is used for calculations of grouping, scaling,
    /// and rotation behavior of shapes placed within a group.
    pub child_offset: Option<Point2D>,
    /// This element specifies the size dimensions of the child extents rectangle and is used for calculations of grouping,
    /// scaling, and rotation behavior of shapes placed within a group.
    pub child_extents: Option<PositiveSize2D>,
}

impl GroupTransform2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "rot" => instance.rotate_angle = Some(value.parse()?),
                "flipH" => instance.flip_horizontal = Some(parse_xml_bool(value)?),
                "flipV" => instance.flip_vertical = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "off" => instance.offset = Some(Point2D::from_xml_element(child_node)?),
                "ext" => instance.extents = Some(PositiveSize2D::from_xml_element(child_node)?),
                "chOff" => instance.child_offset = Some(Point2D::from_xml_element(child_node)?),
                "chExt" => instance.child_extents = Some(PositiveSize2D::from_xml_element(child_node)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct GroupShapeProperties {
    pub black_and_white_mode: Option<BlackWhiteMode>,
    /// This element is nearly identical to the representation of 2-D transforms for ordinary shapes. The only
    /// addition is a member to represent the Child offset and the Child extents.
    pub transform: Option<Box<GroupTransform2D>>,
    pub fill_properties: Option<FillProperties>,
    pub effect_properties: Option<EffectProperties>,
    // TODO implement
    //pub scene_3d: Option<Scene3D>,
}

impl GroupShapeProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let black_and_white_mode = match xml_node.attribute("bwMode") {
            Some(attr) => Some(attr.parse()?),
            None => None,
        };

        let mut transform = None;
        let mut fill_properties = None;
        let mut effect_properties = None;

        for child_node in &xml_node.child_nodes {
            let child_local_name = child_node.local_name();
            if child_local_name == "xfrm" {
                transform = Some(Box::new(GroupTransform2D::from_xml_element(child_node)?));
            } else if FillProperties::is_choice_member(child_local_name) {
                fill_properties = Some(FillProperties::from_xml_element(child_node)?);
            } else if EffectProperties::is_choice_member(child_local_name) {
                effect_properties = Some(EffectProperties::from_xml_element(child_node)?);
            }
        }

        Ok(Self {
            black_and_white_mode,
            transform,
            fill_properties,
            effect_properties,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Geometry {
    /// This element specifies the existence of a custom geometric shape. This shape consists of a series of lines and
    /// curves described within a creation path. In addition to this there can also be adjust values, guides, adjust
    /// handles, connection sites and an inscribed rectangle specified for this custom geometric shape.
    /// 
    /// # Xml example
    /// 
    /// Consider the scenario when a preset geometry does not accurately depict what must be displayed in
    /// the document. For this a custom geometry can be used to define most any 2-dimensional geometric shape.
    /// 
    /// ```xml
    /// <a:custGeom>
    ///   <a:avLst/>
    ///   <a:gdLst/>
    ///   <a:ahLst/>
    ///   <a:cxnLst/>
    ///   <a:rect l="0" t="0" r="0" b="0"/>
    ///   <a:pathLst>
    ///     <a:path w="2650602" h="1261641">
    ///       <a:moveTo>
    ///         <a:pt x="0" y="1261641"/>
    ///       </a:moveTo>
    ///       <a:lnTo>
    ///         <a:pt x="2650602" y="1261641"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="1226916" y="0"/>
    ///       </a:lnTo>
    ///       <a:close/>
    ///     </a:path>
    ///   </a:pathLst>
    /// </a:custGeom>
    /// ```
    Custom(Box<CustomGeometry2D>),
    /// This element specifies when a preset geometric shape should be used instead of a custom geometric shape. The
    /// generating application should be able to render all preset geometries enumerated in the ShapeType enum.
    /// 
    /// # Xml example
    /// 
    /// Consider the scenario when a user does not wish to specify all the lines and curves that make up the
    /// desired shape but instead chooses to use a preset geometry. The following DrawingML would specify such a
    /// case.
    /// 
    /// ```xml
    /// <p:sp>
    ///   <p:nvSpPr>
    ///     <p:cNvPr id="4" name="My Preset Shape"/>
    ///     <p:cNvSpPr/>
    ///     <p:nvPr/>
    ///   </p:nvSpPr>
    ///   <p:spPr>
    ///     <a:xfrm>
    ///       <a:off x="1981200" y="533400"/>
    ///       <a:ext cx="1143000" cy="1066800"/>
    ///     </a:xfrm>
    ///     <a:prstGeom prst="heart">
    ///     </a:prstGeom>
    ///   </p:spPr>
    /// </p:sp>
    /// ```
    Preset(Box<PresetGeometry2D>),
}

impl Geometry {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "custGeom" | "prstGeom" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "custGeom" => Ok(Geometry::Custom(Box::new(CustomGeometry2D::from_xml_element(
                xml_node,
            )?))),
            "prstGeom" => Ok(Geometry::Preset(Box::new(PresetGeometry2D::from_xml_element(
                xml_node,
            )?))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "EG_Geometry").into()),
        }
    }
}

/// This element specifies the precense of a shape guide that is used to govern the geometry of the specified shape.
/// A shape guide consists of a formula and a name that the result of the formula is assigned to. Recognized
/// formulas are listed with the fmla attribute documentation for this element.
/// 
/// # Note
/// 
/// The order in which guides are specified determines the order in which their values are calculated. For
/// instance it is not possible to specify a guide that uses another guides result when that guide has not yet been
/// calculated.
/// 
/// # Example
/// 
/// Consider the case where the user would like to specify a triangle with it's bottom edge defined not by
/// static points but by using a varying parameter, namely an guide. Consider the diagrams and DrawingML shown
/// below. This first triangle has been drawn with a bottom edge that is equal to the 2/3 the value of the shape
/// height. Thus we see in the figure below that the triangle appears to occupy 2/3 of the vertical space within the
/// shape bounding box.
/// 
/// ```xml
/// <a:xfrm>
///   <a:off x="3200400" y="1600200"/>
///   <a:ext cx="1705233" cy="679622"/>
/// </a:xfrm>
/// <a:custGeom>
///   <a:avLst/>
///   <a:gdLst>
///     <a:gd name="myGuide" fmla="*/ h 2 3"/>
///   </a:gdLst>
///   <a:ahLst/>
///   <a:cxnLst/>
///   <a:rect l="0" t="0" r="0" b="0"/>
///   <a:pathLst>
///     <a:path w="1705233" h="679622">
///       <a:moveTo>
///         <a:pt x="0" y="myGuide"/>
///       </a:moveTo>
///       <a:lnTo>
///         <a:pt x="1705233" y="myGuide"/>
///       </a:lnTo>
///       <a:lnTo>
///         <a:pt x="852616" y="0"/>
///       </a:lnTo>
///       <a:close/>
///     </a:path>
///   </a:pathLst>
/// </a:custGeom>
/// ```
/// 
/// If however we change the guide to half that, namely 1/3. Then we see the entire bottom edge of the triangle
/// move to now only occupy 1/3 of the toal space within the shape bounding box. This is because both of the
/// bottom points in this triangle depend on this guide for their coordinate positions.
/// 
/// ```xml
/// <a:gdLst>
///   <a:gd name="myGuide" fmla="*/ h 1 3"/>
/// </a:gdLst>
/// ```
#[derive(Debug, Clone)]
pub struct GeomGuide {
    /// Specifies the name that is used to reference to this guide. This name can be used just as a
    /// variable would within an equation. That is this name can be substituted for literal values
    /// within other guides or the specification of the shape path.
    pub name: GeomGuideName,
    /// Specifies the formula that is used to calculate the value for a guide. Each formula has a
    /// certain number of arguments and a specific set of operations to perform on these
    /// arguments in order to generate a value for a guide. There are a total of 17 different
    /// formulas available. These are shown below with the usage for each defined.
    /// 
    /// * **('\*/') - Multiply Divide Formula**
    /// 
    ///     Arguments: 3 (fmla="*/ x y z")
    /// 
    ///     Usage: "*/ x y z" = ((x * y) / z) = value of this guide
    /// * **('+-') - Add Subtract Formula**
    /// 
    ///     Arguments: 3 (fmla="+- x y z")
    /// 
    ///     Usage: "+- x y z" = ((x + y) - z) = value of this guide
    /// 
    /// * **('+/') - Add Divide Formula**
    /// 
    ///     Arguments: 3 (fmla="+/ x y z")
    /// 
    ///     Usage: "+/ x y z" = ((x + y) / z) = value of this guide
    /// 
    /// * **('?:') - If Else Formula**
    /// 
    ///     Arguments: 3 (fmla="?: x y z")
    /// 
    ///     Usage: "?: x y z" = if (x > 0), then y = value of this guide,  
    ///     else z = value of this guide
    /// 
    /// * **('abs') - Absolute Value Formula**
    /// 
    ///     Arguments: 1 (fmla="abs x")
    /// 
    ///     Usage: "abs x" = if (x < 0), then (-1) * x = value of this guide  
    ///     else x = value of this guide
    /// 
    /// * **('at2') - ArcTan Formula**
    /// 
    ///     Arguments: 2 (fmla="at2 x y")
    /// 
    ///     Usage: "at2 x y" = arctan(y / x) = value of this guide
    /// 
    /// * **('cat2') - Cosine ArcTan Formula**
    /// 
    ///     Arguments: 3 (fmla="cat2 x y z")
    /// 
    ///     Usage: "cat2 x y z" = (x*(cos(arctan(z / y))) = value of this guide
    /// 
    /// * **('cos') - Cosine Formula**
    /// 
    ///     Arguments: 2 (fmla="cos x y")
    /// 
    ///     Usage: "cos x y" = (x * cos( y )) = value of this guide
    /// 
    /// * **('max') - Maximum Value Formula**
    /// 
    ///     Arguments: 2 (fmla="max x y")
    /// 
    ///     Usage: "max x y" = if (x > y), then x = value of this guide  
    ///     else y = value of this guide
    /// 
    /// * **('min') - Minimum Value Formula**
    /// 
    ///     Arguments: 2 (fmla="min x y")
    /// 
    ///     Usage: "min x y" = if (x < y), then x = value of this guide  
    ///     else y = value of this guide
    /// 
    /// * **('mod') - Modulo Formula**
    /// 
    ///     Arguments: 3 (fmla="mod x y z")
    /// 
    ///     Usage: "mod x y z" = sqrt(x^2 + b^2 + c^2) = value of this guide
    /// 
    /// * **('pin') - Pin To Formula**
    /// 
    ///     Arguments: 3 (fmla="pin x y z")
    /// 
    ///     Usage: "pin x y z" = if (y < x), then x = value of this guide  
    ///     else if (y > z), then z = value of this guide  
    ///     else y = value of this guide
    /// 
    /// * **('sat2') - Sine ArcTan Formula**
    /// 
    ///     Arguments: 3 (fmla="sat2 x y z")
    /// 
    ///     Usage: "sat2 x y z" = (x*sin(arctan(z / y))) = value of this guide
    /// 
    /// * **('sin') - Sine Formula**
    /// 
    ///     Arguments: 2 (fmla="sin x y")
    /// 
    ///     Usage: "sin x y" = (x * sin( y )) = value of this guide
    /// 
    /// * **('sqrt') - Square Root Formula**
    /// 
    ///     Arguments: 1 (fmla="sqrt x")
    /// 
    ///     Usage: "sqrt x" = sqrt(x) = value of this guide
    /// 
    /// * **('tan') - Tangent Formula**
    /// 
    ///     Arguments: 2 (fmla="tan x y")
    /// 
    ///     Usage: "tan x y" = (x * tan( y )) = value of this guide
    /// 
    /// * **('val') - Literal Value Formula**
    /// 
    ///     Arguments: 1 (fmla="val x")
    /// 
    ///     Usage: "val x" = x = value of this guide
    /// 
    /// # Note
    /// 
    /// Guides that have a literal value formula specified via fmla="val x" above should
    /// only be used within the avLst as an adjust value for the shape. This however is not
    /// strictly enforced.
    pub formula: GeomGuideFormula,
}

impl GeomGuide {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut name = None;
        let mut formula = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "name" => name = Some(value.clone()),
                "fmla" => formula = Some(value.clone()),
                _ => (),
            }
        }

        let name = name.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "name"))?;
        let formula = formula.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "fmla"))?;
        Ok(Self { name, formula })
    }
}

#[derive(Debug, Clone)]
pub enum AdjustHandle {
    /// This element specifies an XY-based adjust handle for a custom shape. The position of this adjust handle is
    /// specified by the corresponding pos child element. The allowed adjustment of this adjust handle are specified via
    /// it's min and max type attributes. Based on the adjustment of this adjust handle certain corresponding guides are
    /// updated to contain these values.
    XY(Box<XYAdjustHandle>),
    /// This element specifies a polar adjust handle for a custom shape. The position of this adjust handle is specified by
    /// the corresponding pos child element. The allowed adjustment of this adjust handle are specified via it's min and
    /// max attributes. Based on the adjustment of this adjust handle certain corresponding guides are updated to
    /// contain these values.
    Polar(Box<PolarAdjustHandle>),
}

impl AdjustHandle {
    pub fn is_choice_member(name: &str) -> bool {
        match name {
            "ahXY" | "ahPolar" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "ahXY" => Ok(AdjustHandle::XY(Box::new(XYAdjustHandle::from_xml_element(xml_node)?))),
            "ahPolar" => Ok(AdjustHandle::Polar(Box::new(PolarAdjustHandle::from_xml_element(
                xml_node,
            )?))),
            _ => Err(NotGroupMemberError::new(xml_node.name.clone(), "AdjustHandle").into()),
        }
    }
}

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

/// This element specifies an x-y coordinate within the path coordinate space. This coordinate space is determined
/// by the width and height attributes defined within the path element. A point is utilized by one of it's parent
/// elements to specify the next point of interest in custom geometry shape. Depending on the parent element used
/// the point can either have a line drawn to it or the cursor can simply be moved to this new location.
/// 
/// Specifies a position coordinate within the shape bounding box. It should be noted that this coordinate is placed
/// within the shape bounding box using the transform coordinate system which is also called the shape coordinate
/// system, as it encompasses the entire shape. The width and height for this coordinate system are specified within
/// the ext transform element.
/// 
/// # Note
/// 
/// When specifying a point coordinate in path coordinate space it should be noted that the top left of the
/// coordinate space is x=0, y=0 and the coordinate points for x grow to the right and for y grow down.
/// 
/// # Xml example
/// 
/// To highlight the differences in the coordinate systems consider the drawing of the following triangle.
/// Notice that the dimensions of the triangle are specified using the shape coordinate system with EMUs as the
/// units via the ext transform element. Thus we see this shape is 1705233 EMUs wide by 679622 EMUs tall.
/// However when looking at how the path for this shape is drawn we see that the x and y values fall between 0 and
/// 2. This is because the path coordinate system has the arbitrary dimensions of 2 for the width and 2 for the
/// height. Thus we see that a y coordinate of 2 within the path coordinate system specifies a y coordinate of
/// 679622 within the shape coordinate system for this particular case.
/// 
/// ```xml
/// <a:xfrm>
///   <a:off x="3200400" y="1600200"/>
///   <a:ext cx="1705233" cy="679622"/>
/// </a:xfrm>
/// <a:custGeom>
///   <a:avLst/>
///   <a:gdLst/>
///   <a:ahLst/>
///   <a:cxnLst/>
///   <a:rect l="0" t="0" r="0" b="0"/>
///   <a:pathLst>
///     <a:path w="2" h="2">
///       <a:moveTo>
///         <a:pt x="0" y="2"/>
///       </a:moveTo>
///       <a:lnTo>
///         <a:pt x="2" y="2"/>
///       </a:lnTo>
///       <a:lnTo>
///         <a:pt x="1" y="0"/>
///       </a:lnTo>
///       <a:close/>
///     </a:path>
///   </a:pathLst>
/// </a:custGeom>
/// ```
#[derive(Debug, Clone)]
pub struct AdjPoint2D {
    /// Specifies the x coordinate for this position coordinate. The units for this coordinate space
    /// are defined by the width of the path coordinate system. This coordinate system is
    /// overlayed on top of the shape coordinate system thus occupying the entire shape
    /// bounding box. Because the units for within this coordinate space are determined by the
    /// path width and height an exact measurement unit cannot be specified here.
    pub x: AdjCoordinate,
    /// Specifies the y coordinate for this position coordinate. The units for this coordinate space
    /// are defined by the height of the path coordinate system. This coordinate system is
    /// overlayed on top of the shape coordinate system thus occupying the entire shape
    /// bounding box. Because the units for within this coordinate space are determined by the
    /// path width and height an exact measurement unit cannot be specified here.
    pub y: AdjCoordinate,
}

impl AdjPoint2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut x = None;
        let mut y = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "x" => x = Some(value.parse()?),
                "y" => y = Some(value.parse()?),
                _ => (),
            }
        }

        let x = x.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "x"))?;
        let y = y.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "y"))?;

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct GeomRect {
    /// Specifies the x coordinate of the left edge for a shape text rectangle. The units for this
    /// edge is specified in EMUs as the positioning here is based on the shape coordinate
    /// system. The width and height for this coordinate system are specified within the ext
    /// transform element.
    pub left: AdjCoordinate,
    /// Specifies the y coordinate of the top edge for a shape text rectangle. The units for this
    /// edge is specified in EMUs as the positioning here is based on the shape coordinate
    /// system. The width and height for this coordinate system are specified within the ext
    /// transform element.
    pub top: AdjCoordinate,
    /// Specifies the x coordinate of the right edge for a shape text rectangle. The units for this
    /// edge is specified in EMUs as the positioning here is based on the shape coordinate
    /// system. The width and height for this coordinate system are specified within the ext
    /// transform element.
    pub right: AdjCoordinate,
    /// Specifies the y coordinate of the bottom edge for a shape text rectangle. The units for
    /// this edge is specified in EMUs as the positioning here is based on the shape coordinate
    /// system. The width and height for this coordinate system are specified within the ext
    /// transform element.
    pub bottom: AdjCoordinate,
}

impl GeomRect {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut left = None;
        let mut top = None;
        let mut right = None;
        let mut bottom = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "l" => left = Some(value.parse()?),
                "t" => top = Some(value.parse()?),
                "r" => right = Some(value.parse()?),
                "b" => bottom = Some(value.parse()?),
                _ => (),
            }
        }

        let left = left.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "l"))?;
        let top = top.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "t"))?;
        let right = right.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "r"))?;
        let bottom = bottom.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "b"))?;

        Ok(Self {
            left,
            top,
            right,
            bottom,
        })
    }
}

#[derive(Debug, Clone)]
pub struct XYAdjustHandle {
    /// Specifies the name of the guide that is updated with the adjustment x position from this
    /// adjust handle.
    pub guide_reference_x: Option<GeomGuideName>,
    /// Specifies the name of the guide that is updated with the adjustment y position from this
    /// adjust handle.
    pub guide_reference_y: Option<GeomGuideName>,
    /// Specifies the minimum horizontal position that is allowed for this adjustment handle. If
    /// this attribute is omitted, then it is assumed that this adjust handle cannot move in the x
    /// direction. That is the maxX and minX are equal.
    pub min_x: Option<AdjCoordinate>,
    /// Specifies the maximum horizontal position that is allowed for this adjustment handle. If
    /// this attribute is omitted, then it is assumed that this adjust handle cannot move in the x
    /// direction. That is the maxX and minX are equal.
    pub max_x: Option<AdjCoordinate>,
    /// Specifies the minimum vertical position that is allowed for this adjustment handle. If this
    /// attribute is omitted, then it is assumed that this adjust handle cannot move in the y
    /// direction. That is the maxY and minY are equal.
    pub min_y: Option<AdjCoordinate>,
    /// Specifies the maximum vertical position that is allowed for this adjustment handle. If this
    /// attribute is omitted, then it is assumed that this adjust handle cannot move in the y
    /// direction. That is the maxY and minY are equal.
    pub max_y: Option<AdjCoordinate>,
    /// Specifies a position coordinate within the shape bounding box. It should be noted that this coordinate is placed
    /// within the shape bounding box using the transform coordinate system which is also called the shape coordinate
    /// system, as it encompasses the entire shape. The width and height for this coordinate system are specified within
    /// the ext transform element.
    /// 
    /// # Note
    /// 
    /// When specifying a point coordinate in path coordinate space it should be noted that the top left of the
    /// coordinate space is x=0, y=0 and the coordinate points for x grow to the right and for y grow down.
    /// 
    /// # Xml example
    /// 
    /// To highlight the differences in the coordinate systems consider the drawing of the following triangle.
    /// Notice that the dimensions of the triangle are specified using the shape coordinate system with EMUs as the
    /// units via the ext transform element. Thus we see this shape is 1705233 EMUs wide by 679622 EMUs tall.
    /// However when looking at how the path for this shape is drawn we see that the x and y values fall between 0 and
    /// 2. This is because the path coordinate system has the arbitrary dimensions of 2 for the width and 2 for the
    /// height. Thus we see that a y coordinate of 2 within the path coordinate system specifies a y coordinate of
    /// 679622 within the shape coordinate system for this particular case.
    /// 
    /// ```xml
    /// <a:xfrm>
    ///   <a:off x="3200400" y="1600200"/>
    ///   <a:ext cx="1705233" cy="679622"/>
    /// </a:xfrm>
    /// <a:custGeom>
    ///   <a:avLst/>
    ///   <a:gdLst/>
    ///   <a:ahLst/>
    ///   <a:cxnLst/>
    ///   <a:rect l="0" t="0" r="0" b="0"/>
    ///   <a:pathLst>
    ///     <a:path w="2" h="2">
    ///       <a:moveTo>
    ///         <a:pt x="0" y="2"/>
    ///       </a:moveTo>
    ///       <a:lnTo>
    ///         <a:pt x="2" y="2"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="1" y="0"/>
    ///       </a:lnTo>
    ///       <a:close/>
    ///     </a:path>
    ///   </a:pathLst>
    /// </a:custGeom>
    /// ```
    pub position: AdjPoint2D,
}

impl XYAdjustHandle {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut guide_reference_x = None;
        let mut guide_reference_y = None;
        let mut min_x = None;
        let mut max_x = None;
        let mut min_y = None;
        let mut max_y = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "gdRefX" => guide_reference_x = Some(value.clone()),
                "gdRefY" => guide_reference_y = Some(value.clone()),
                "minX" => min_x = Some(value.parse()?),
                "maxX" => max_x = Some(value.parse()?),
                "minY" => min_y = Some(value.parse()?),
                "maxY" => max_y = Some(value.parse()?),
                _ => (),
            }
        }

        let pos_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pos"))?;
        let position = AdjPoint2D::from_xml_element(pos_node)?;

        Ok(Self {
            guide_reference_x,
            guide_reference_y,
            min_x,
            max_x,
            min_y,
            max_y,
            position,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PolarAdjustHandle {
    /// Specifies the name of the guide that is updated with the adjustment radius from this
    /// adjust handle.
    pub guide_reference_radial: Option<GeomGuideName>,
    /// Specifies the name of the guide that is updated with the adjustment angle from this
    /// adjust handle.
    pub guide_reference_angle: Option<GeomGuideName>,
    /// Specifies the minimum radial position that is allowed for this adjustment handle. If this
    /// attribute is omitted, then it is assumed that this adjust handle cannot move radially. That
    /// is the maxR and minR are equal.
    pub min_radial: Option<AdjCoordinate>,
    /// Specifies the maximum radial position that is allowed for this adjustment handle. If this
    /// attribute is omitted, then it is assumed that this adjust handle cannot move radially. That
    /// is the maxR and minR are equal.
    pub max_radial: Option<AdjCoordinate>,
    /// Specifies the minimum angle position that is allowed for this adjustment handle. If this
    /// attribute is omitted, then it is assumed that this adjust handle cannot move angularly.
    /// That is the maxAng and minAng are equal.
    pub min_angle: Option<AdjAngle>,
    /// Specifies the maximum angle position that is allowed for this adjustment handle. If this
    /// attribute is omitted, then it is assumed that this adjust handle cannot move angularly.
    /// That is the maxAng and minAng are equal.
    pub max_angle: Option<AdjAngle>,
    pub position: AdjPoint2D,
}

impl PolarAdjustHandle {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut guide_reference_radial = None;
        let mut guide_reference_angle = None;
        let mut min_radial = None;
        let mut max_radial = None;
        let mut min_angle = None;
        let mut max_angle = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "gdRefR" => guide_reference_radial = Some(value.clone()),
                "gdRefAng" => guide_reference_angle = Some(value.clone()),
                "minR" => min_radial = Some(value.parse()?),
                "maxR" => max_radial = Some(value.parse()?),
                "minAng" => min_angle = Some(value.parse()?),
                "maxAng" => max_angle = Some(value.parse()?),
                _ => (),
            }
        }

        let pos_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pos"))?;
        let position = AdjPoint2D::from_xml_element(pos_node)?;

        Ok(Self {
            guide_reference_radial,
            guide_reference_angle,
            min_radial,
            max_radial,
            min_angle,
            max_angle,
            position,
        })
    }
}

/// This element specifies the existence of a connection site on a custom shape. A connection site allows a cxnSp to
/// be attached to this shape. This connection is maintained when the shape is repositioned within the document. It
/// should be noted that this connection is placed within the shape bounding box using the transform coordinate
/// system which is also called the shape coordinate system, as it encompasses the entire shape. The width and
/// height for this coordinate system are specified within the ext transform element.
/// 
/// # Note
/// 
/// The transform coordinate system is different from a path coordinate system as it is per shape instead of
/// per path within the shape.
/// 
/// # Xml example
/// 
/// Consider the following custom geometry that has two connection sites specified. One connection is
/// located at the bottom left of the shape and the other at the bottom right. The following DrawingML would
/// describe such a custom geometry.
/// 
/// ```xml
/// <a:xfrm>
///   <a:off x="3200400" y="1600200"/>
///   <a:ext cx="1705233" cy="679622"/>
/// </a:xfrm>
/// <a:custGeom>
///   <a:avLst/>
///   <a:gdLst/>
///   <a:ahLst/>
///   <a:cxnLst>
///     <a:cxn ang="0">
///       <a:pos x="0" y="679622"/>
///     </a:cxn>
///     <a:cxn ang="0">
///       <a:pos x="1705233" y="679622"/>
///     </a:cxn>
///   </a:cxnLst>
///   <a:rect l="0" t="0" r="0" b="0"/>
///   <a:pathLst>
///     <a:path w="2" h="2">
///       <a:moveTo>
///         <a:pt x="0" y="2"/>
///       </a:moveTo>
///       <a:lnTo>
///         <a:pt x="2" y="2"/>
///       </a:lnTo>
///       <a:lnTo>
///         <a:pt x="1" y="0"/>
///       </a:lnTo>
///       <a:close/>
///     </a:path>
///   </a:pathLst>
/// </a:custGeom>
/// ```
#[derive(Debug, Clone)]
pub struct ConnectionSite {
    /// Specifies the incoming connector angle. This angle is the angle around the connection
    /// site that an incoming connector tries to be routed to. This allows connectors to know
    /// where the shape is in relation to the connection site and route connectors so as to avoid
    /// any overlap with the shape.
    pub angle: AdjAngle,
    pub position: AdjPoint2D,
}

impl ConnectionSite {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let angle_attr = xml_node
            .attribute("ang")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "ang"))?;
        let angle = angle_attr.parse()?;

        let pos_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pos"))?;
        let position = AdjPoint2D::from_xml_element(pos_node)?;

        Ok(Self { angle, position })
    }
}

#[derive(Debug, Clone)]
pub enum Path2DCommand {
    /// This element specifies the ending of a series of lines and curves in the creation path of a custom geometric
    /// shape. When this element is encountered, the generating application should consider the corresponding path
    /// closed. That is, any further lines or curves that follow this element should be ignored.
    /// 
    /// # Note
    /// 
    /// A path can be specified and not closed. A path such as this cannot however have any fill associated with it
    /// as it has not been considered a closed geometric path.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:custGeom>
    ///   <a:pathLst>
    ///     <a:path w="2824222" h="590309">
    ///       <a:moveTo>
    ///         <a:pt x="0" y="428263"/>
    ///       </a:moveTo>
    ///       <a:lnTo>
    ///         <a:pt x="1620455" y="590309"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="2824222" y="173620"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="1562582" y="0"/>
    ///       </a:lnTo>
    ///       <a:close/>
    ///     </a:path>
    ///   </a:pathLst>
    /// </a:custGeom>
    /// ```
    /// 
    /// In the above example there is specified a four sided geometric shape that has all straight sides. While we only
    /// see three lines being drawn via the lnTo element there are actually four sides because the last point of
    /// (x=1562585, y=0) is connected to the first point in the creation path via a lnTo element
    /// 
    /// # Note
    /// 
    /// When the last point in the creation path does not meet with the first point in the creation path the
    /// generating application should connect the last point with the first via a straight line, thus creating a closed shape
    /// geometry.
    Close,
    /// This element specifies a set of new coordinates to move the shape cursor to. This element is only used for
    /// drawing a custom geometry. When this element is utilized the pt element is used to specify a new set of shape
    /// coordinates that the shape cursor should be moved to. This does not draw a line or curve to this new position
    /// from the old position but simply move the cursor to a new starting position. It is only when a path drawing
    /// element such as lnTo is used that a portion of the path is drawn.
    /// 
    /// # Xml example
    /// 
    /// Consider the case where a user wishes to begin drawing a custom geometry not at the default starting
    /// coordinates of x=0 , y=0 but at coordinates further inset into the shape coordinate space. The following
    /// DrawingML would specify such a case.
    /// 
    /// ```xml
    /// <a:custGeom>
    ///   <a:pathLst>
    ///     <a:path w="2824222" h="590309">
    ///       <a:moveTo>
    ///         <a:pt x="0" y="428263"/>
    ///       </a:moveTo>
    ///       <a:lnTo>
    ///         <a:pt x="1620455" y="590309"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="2824222" y="173620"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="1562582" y="0"/>
    ///       </a:lnTo>
    ///       <a:close/>
    ///     </a:path>
    ///   </a:pathLst>
    /// </a:custGeom>
    /// ```
    /// 
    /// Notice the moveTo element advances the y coordinates before any actual lines are drawn
    MoveTo(AdjPoint2D),
    /// This element specifies the drawing of a straight line from the current pen position to the new point specified.
    /// This line becomes part of the shape geometry, representing a side of the shape. The coordinate system used
    /// when specifying this line is the path coordinate system.
    LineTo(AdjPoint2D),
    /// This element specifies the existence of an arc within a shape path. It draws an arc with the specified parameters
    /// from the current pen position to the new point specified. An arc is a line that is bent based on the shape of a
    /// supposed circle. The length of this arc is determined by specifying both a start angle and an ending angle that
    /// act together to effectively specify an end point for the arc.
    ArcTo(Path2DArcTo),
    /// This element specifies to draw a quadratic bezier curve along the specified points. To specify a quadratic bezier
    /// curve there needs to be 2 points specified. The first is a control point used in the quadratic bezier calculation
    /// and the last is the ending point for the curve. The coordinate system used for this type of curve is the path
    /// coordinate system as this element is path specific.
    QuadBezierTo(AdjPoint2D, AdjPoint2D),
    /// This element specifies to draw a cubic bezier curve along the specified points. To specify a cubic bezier curve
    /// there needs to be 3 points specified. The first two are control points used in the cubic bezier calculation and the
    /// last is the ending point for the curve. The coordinate system used for this kind of curve is the path coordinate
    /// system as this element is path specific.
    CubicBezTo(AdjPoint2D, AdjPoint2D, AdjPoint2D),
}

impl Path2DCommand {
    pub fn is_choice_member<T>(name: T) -> bool
    where
        T: AsRef<str>,
    {
        match name.as_ref() {
            "close" | "moveTo" | "lnTo" | "arcTo" | "quadBezTo" | "cubicBezTo" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "close" => Ok(Path2DCommand::Close),
            "moveTo" => {
                let pt_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                Ok(Path2DCommand::MoveTo(AdjPoint2D::from_xml_element(pt_node)?))
            }
            "lnTo" => {
                let pt_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                Ok(Path2DCommand::LineTo(AdjPoint2D::from_xml_element(pt_node)?))
            }
            "arcTo" => Ok(Path2DCommand::ArcTo(Path2DArcTo::from_xml_element(xml_node)?)),
            "quadBezTo" => {
                let pt1_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                let pt2_node = xml_node
                    .child_nodes
                    .get(1)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                Ok(Path2DCommand::QuadBezierTo(
                    AdjPoint2D::from_xml_element(pt1_node)?,
                    AdjPoint2D::from_xml_element(pt2_node)?,
                ))
            }
            "cubicBezTo" => {
                let pt1_node = xml_node
                    .child_nodes
                    .get(0)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                let pt2_node = xml_node
                    .child_nodes
                    .get(1)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                let pt3_node = xml_node
                    .child_nodes
                    .get(2)
                    .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "pt"))?;
                Ok(Path2DCommand::CubicBezTo(
                    AdjPoint2D::from_xml_element(pt1_node)?,
                    AdjPoint2D::from_xml_element(pt2_node)?,
                    AdjPoint2D::from_xml_element(pt3_node)?,
                ))
            }
            _ => Err(Box::new(NotGroupMemberError::new(
                xml_node.name.clone(),
                "EG_Path2DCommand",
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path2DArcTo {
    /// This attribute specifies the width radius of the supposed circle being used to draw the
    /// arc. This gives the circle a total width of (2 * wR). This total width could also be called it's
    /// horizontal diameter as it is the diameter for the x axis only.
    pub width_radius: AdjCoordinate,
    /// This attribute specifies the height radius of the supposed circle being used to draw the
    /// arc. This gives the circle a total height of (2 * hR). This total height could also be called
    /// it's vertical diameter as it is the diameter for the y axis only.
    pub height_radius: AdjCoordinate,
    /// Specifies the start angle for an arc. This angle specifies what angle along the supposed
    /// circle path is used as the start position for drawing the arc. This start angle is locked to
    /// the last known pen position in the shape path. Thus guaranteeing a continuos shape
    /// path.
    pub start_angle: AdjAngle,
    /// Specifies the swing angle for an arc. This angle specifies how far angle-wise along the
    /// supposed cicle path the arc is extended. The extension from the start angle is always in
    /// the clockwise direction around the supposed circle.
    pub swing_angle: AdjAngle,
}

impl Path2DArcTo {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut width_radius = None;
        let mut height_radius = None;
        let mut start_angle = None;
        let mut swing_angle = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "wR" => width_radius = Some(value.parse()?),
                "hR" => height_radius = Some(value.parse()?),
                "stAng" => start_angle = Some(value.parse()?),
                "swAng" => swing_angle = Some(value.parse()?),
                _ => (),
            }
        }

        let width_radius = width_radius.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "wR"))?;
        let height_radius = height_radius.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "hR"))?;
        let start_angle = start_angle.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "stAng"))?;
        let swing_angle = swing_angle.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "swAng"))?;

        Ok(Self {
            width_radius,
            height_radius,
            start_angle,
            swing_angle,
        })
    }
}

/// This element specifies a creation path consisting of a series of moves, lines and curves that when combined
/// forms a geometric shape. This element is only utilized if a custom geometry is specified.
/// 
/// # Note
/// 
/// Since multiple paths are allowed the rules for drawing are that the path specified later in the pathLst is
/// drawn on top of all previous paths.
/// 
/// # Xml example
/// 
/// ```xml
/// <a:custGeom>
///   <a:pathLst>
///     <a:path w="2824222" h="590309">
///       <a:moveTo>
///         <a:pt x="0" y="428263"/>
///       </a:moveTo>
///       <a:lnTo>
///         <a:pt x="1620455" y="590309"/>
///       </a:lnTo>
///       <a:lnTo>
///         <a:pt x="2824222" y="173620"/>
///       </a:lnTo>
///       <a:lnTo>
///         <a:pt x="1562582" y="0"/>
///       </a:lnTo>
///       <a:close/>
///     </a:path>
///   </a:pathLst>
/// </a:custGeom>
/// ```
/// 
/// In the above example there is specified a four sided geometric shape that has all straight sides. While we only
/// see three lines being drawn via the lnTo element there are actually four sides because the last point of
/// (x=1562585, y=0) is connected to the first point in the creation path via a lnTo element
#[derive(Default, Debug, Clone)]
pub struct Path2D {
    /// Specifies the width, or maximum x coordinate that should be used for within the path
    /// coordinate system. This value determines the horizontal placement of all points within
    /// the corresponding path as they are all calculated using this width attribute as the max x
    /// coordinate.
    /// 
    /// Defaults to 0
    pub width: Option<PositiveCoordinate>,
    /// Specifies the height, or maximum y coordinate that should be used for within the path
    /// coordinate system. This value determines the vertical placement of all points within the
    /// corresponding path as they are all calculated using this height attribute as the max y
    /// coordinate.
    /// 
    /// Defaults to 0
    pub height: Option<PositiveCoordinate>,
    /// Specifies how the corresponding path should be filled. If this attribute is omitted, a value
    /// of "norm" is assumed.
    /// 
    /// Defaults to PathFillMode::Norm
    pub fill_mode: Option<PathFillMode>,
    /// Specifies if the corresponding path should have a path stroke shown. This is a boolean
    /// value that affect the outline of the path. If this attribute is omitted, a value of true is
    /// assumed.
    /// 
    /// Defaults to true
    pub stroke: Option<bool>,
    /// Specifies that the use of 3D extrusions are possible on this path. This allows the
    /// generating application to know whether 3D extrusion can be applied in any form. If this
    /// attribute is omitted then a value of 0, or false is assumed.
    /// 
    /// Defaults to true
    pub extrusion_ok: Option<bool>,
    pub commands: Vec<Path2DCommand>,
}

impl Path2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "w" => instance.width = Some(value.parse()?),
                "h" => instance.height = Some(value.parse()?),
                "fill" => instance.fill_mode = Some(value.parse()?),
                "stroke" => instance.stroke = Some(parse_xml_bool(value)?),
                "extrusionOk" => instance.extrusion_ok = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        for child_node in &xml_node.child_nodes {
            if Path2DCommand::is_choice_member(child_node.local_name()) {
                instance.commands.push(Path2DCommand::from_xml_element(child_node)?);
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct CustomGeometry2D {
    /// This element specifies the adjust values that are applied to the specified shape. An adjust value is simply a guide
    /// that has a value based formula specified. That is, no calculation takes place for an adjust value guide. Instead,
    /// this guide specifies a parameter value that is used for calculations within the shape guides.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:xfrm>
    ///   <a:off x="3200400" y="1600200"/>
    ///   <a:ext cx="1705233" cy="679622"/>
    /// </a:xfrm>
    /// <a:custGeom>
    ///   <a:avLst>
    ///     <a:gd name="myGuide" fmla="val 2"/>
    ///   </a:avLst>
    ///   <a:gdLst/>
    ///   <a:ahLst/>
    ///   <a:cxnLst/>
    ///   <a:rect l="0" t="0" r="0" b="0"/>
    ///   <a:pathLst>
    ///     <a:path w="2" h="2">
    ///       <a:moveTo>
    ///         <a:pt x="0" y="myGuide"/>
    ///       </a:moveTo>
    ///       <a:lnTo>
    ///         <a:pt x="2" y="myGuide"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="1" y="0"/>
    ///       </a:lnTo>
    ///       <a:close/>
    ///     </a:path>
    ///   </a:pathLst>
    /// </a:custGeom>
    /// ```
    pub adjust_value_list: Option<Vec<GeomGuide>>,
    /// This element specifies all the guides that are used for this shape. A guide is specified by the gd element and
    /// defines a calculated value that can be used for the construction of the corresponding shape.
    /// 
    /// # Note
    /// 
    /// Guides that have a literal value formula specified via fmla="val x" above should only be used within the
    /// adjust_value_list as an adjust value for the shape. This however is not strictly enforced.
    pub guide_list: Option<Vec<GeomGuide>>,
    /// This element specifies the adjust handles that are applied to a custom geometry. These adjust handles specify
    /// points within the geometric shape that can be used to perform certain transform operations on the shape.
    /// 
    /// # Example
    /// 
    /// Consider the scenario where a custom geometry, an arrow in this case, has been drawn and adjust
    /// handles have been placed at the top left corner of both the arrow head and arrow body. The user interface can
    /// then be made to transform only certain parts of the shape by using the corresponding adjust handle.
    /// 
    /// For instance if the user wished to change only the width of the arrow head then they would use the adjust
    /// handle located on the top left of the arrow head.
    pub adjust_handle_list: Option<Vec<AdjustHandle>>,
    /// This element specifies all the connection sites that are used for this shape. A connection site is specified by
    /// defining a point within the shape bounding box that can have a cxnSp element attached to it. These connection
    /// sites are specified using the shape coordinate system that is specified within the ext transform element.
    pub connection_site_list: Option<Vec<ConnectionSite>>,
    /// This element specifies the rectangular bounding box for text within a custGeom shape. The default for this
    /// rectangle is the bounding box for the shape. This can be modified using this elements four attributes to inset or
    /// extend the text bounding box.
    /// 
    /// # Note
    /// 
    /// Text specified to reside within this shape text rectangle can flow outside this bounding box. Depending on
    /// the autofit options within the txBody element the text might not entirely reside within this shape text rectangle.
    pub rect: Option<Box<GeomRect>>,
    /// This element specifies the entire path that is to make up a single geometric shape. The path_list can consist of
    /// many individual paths within it.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <a:custGeom>
    ///   <a:pathLst>
    ///     <a:path w="2824222" h="590309">
    ///       <a:moveTo>
    ///         <a:pt x="0" y="428263"/>
    ///       </a:moveTo>
    ///       <a:lnTo>
    ///         <a:pt x="1620455" y="590309"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="2824222" y="173620"/>
    ///       </a:lnTo>
    ///       <a:lnTo>
    ///         <a:pt x="1562582" y="0"/>
    ///       </a:lnTo>
    ///       <a:close/>
    ///     </a:path>
    ///   </a:pathLst>
    /// </a:custGeom>
    /// ```
    /// 
    /// In the above example there is specified a four sided geometric shape that has all straight sides. While we only
    /// see three lines being drawn via the lnTo element there are actually four sides because the last point of
    /// (x=1562585, y=0) is connected to the first point in the creation path via a lnTo element.
    /// 
    /// # Note
    /// 
    /// A geometry with multiple paths within it should be treated visually as if each path were a distinct shape.
    /// That is each creation path has its first point and last point joined to form a closed shape. However, the
    /// generating application should then connect the last point to the first point of the new shape. If a close element
    /// is encountered at the end of the previous creation path then this joining line should not be rendered by the
    /// generating application. The rendering should resume with the first line or curve on the new creation path.
    pub path_list: Vec<Box<Path2D>>,
}

impl CustomGeometry2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "avLst" => {
                    let mut vec = Vec::new();
                    for av_node in &child_node.child_nodes {
                        vec.push(GeomGuide::from_xml_element(av_node)?);
                    }
                    instance.adjust_value_list = Some(vec);
                }
                "gdLst" => {
                    let mut vec = Vec::new();
                    for gd_node in &child_node.child_nodes {
                        vec.push(GeomGuide::from_xml_element(gd_node)?);
                    }
                    instance.guide_list = Some(vec);
                }
                "ahLst" => {
                    let mut vec = Vec::new();
                    for ah_node in &child_node.child_nodes {
                        vec.push(AdjustHandle::from_xml_element(ah_node)?);
                    }
                    instance.adjust_handle_list = Some(vec);
                }
                "cxnLst" => {
                    let mut vec = Vec::new();
                    for cxn_node in &child_node.child_nodes {
                        vec.push(ConnectionSite::from_xml_element(cxn_node)?);
                    }
                    instance.connection_site_list = Some(vec);
                }
                "rect" => instance.rect = Some(Box::new(GeomRect::from_xml_element(child_node)?)),
                "pathLst" => {
                    for path_node in &child_node.child_nodes {
                        instance.path_list.push(Box::new(Path2D::from_xml_element(path_node)?));
                    }
                }
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct PresetGeometry2D {
    /// Specifies the preset geometry that is used for this shape. This preset can have any of the
    /// values in the enumerated list for ShapeType. This attribute is required in order for a
    /// preset geometry to be rendered.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <p:sp>
    ///   <p:nvSpPr>
    ///     <p:cNvPr id="4" name="Sun 3"/>
    ///     <p:cNvSpPr/>
    ///     <p:nvPr/>
    ///   </p:nvSpPr>
    ///   <p:spPr>
    ///     <a:xfrm>
    ///       <a:off x="1981200" y="533400"/>
    ///       <a:ext cx="1143000" cy="1066800"/>
    ///     </a:xfrm>
    ///     <a:prstGeom prst="sun">
    ///     </a:prstGeom>
    ///   </p:spPr>
    /// </p:sp>
    /// ```
    /// 
    /// In the above example a preset geometry has been used to define a shape. The shape
    /// utilized here is the sun shape.
    pub preset: ShapeType,
    pub adjust_value_list: Vec<GeomGuide>,
}

impl PresetGeometry2D {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let preset_attr = xml_node
            .attribute("prst")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "prst"))?;
        let preset = preset_attr.parse()?;
        let mut adjust_value_list = Vec::new();

        for child_node in &xml_node.child_nodes {
            if child_node.local_name() == "avLst" {
                for av_node in &child_node.child_nodes {
                    adjust_value_list.push(GeomGuide::from_xml_element(av_node)?);
                }
            }
        }

        Ok(Self {
            preset,
            adjust_value_list,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct ShapeProperties {
    /// Specifies that the picture should be rendered using only black and white coloring. That is
    /// the coloring information for the picture should be converted to either black or white
    /// when rendering the picture.
    /// 
    /// No gray is to be used in rendering this image, only stark black and stark white.
    /// 
    /// # Note
    /// 
    /// This does not mean that the picture itself that is stored within the file is
    /// necessarily a black and white picture. This attribute instead sets the rendering mode that
    /// the picture has applied to when rendering.
    pub black_and_white_mode: Option<BlackWhiteMode>,
    /// This element represents 2-D transforms for ordinary shapes.
    pub transform: Option<Box<Transform2D>>,
    pub geometry: Option<Geometry>,
    pub fill_properties: Option<FillProperties>,
    pub line_properties: Option<Box<LineProperties>>,
    pub effect_properties: Option<EffectProperties>,
    // TODO implement
    //pub scene_3d: Option<Scene3D>,
    //pub shape_3d: Option<Shape3D>,
}

impl ShapeProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        instance.black_and_white_mode = match xml_node.attribute("bwMode") {
            Some(value) => Some(value.parse()?),
            None => None,
        };

        for child_node in &xml_node.child_nodes {
            let child_local_name = child_node.local_name();
            if Geometry::is_choice_member(child_local_name) {
                instance.geometry = Some(Geometry::from_xml_element(child_node)?);
            } else if FillProperties::is_choice_member(child_local_name) {
                instance.fill_properties = Some(FillProperties::from_xml_element(child_node)?);
            } else if EffectProperties::is_choice_member(child_local_name) {
                instance.effect_properties = Some(EffectProperties::from_xml_element(child_node)?);
            } else {
                match child_local_name {
                    "xfrm" => instance.transform = Some(Box::new(Transform2D::from_xml_element(child_node)?)),
                    "ln" => instance.line_properties = Some(Box::new(LineProperties::from_xml_element(child_node)?)),
                    _ => (),
                }
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct ShapeStyle {
    pub line_reference: StyleMatrixReference,
    pub fill_reference: StyleMatrixReference,
    pub effect_reference: StyleMatrixReference,
    /// This element represents a reference to a themed font. When used it specifies which themed font to use along
    /// with a choice of color.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <fontRef idx="minor">
    ///   <schemeClr val="tx1"/>
    /// </fontRef>
    /// ```
    pub font_reference: FontReference,
}

impl ShapeStyle {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut line_reference = None;
        let mut fill_reference = None;
        let mut effect_reference = None;
        let mut font_reference = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "lnRef" => line_reference = Some(StyleMatrixReference::from_xml_element(child_node)?),
                "fillRef" => fill_reference = Some(StyleMatrixReference::from_xml_element(child_node)?),
                "effectRef" => effect_reference = Some(StyleMatrixReference::from_xml_element(child_node)?),
                "fontRef" => font_reference = Some(FontReference::from_xml_element(child_node)?),
                _ => (),
            }
        }

        let line_reference =
            line_reference.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "lnRef"))?;
        let fill_reference =
            fill_reference.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "fillRef"))?;
        let effect_reference =
            effect_reference.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "effectRef"))?;
        let font_reference =
            font_reference.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "fontRef"))?;

        Ok(Self {
            line_reference,
            fill_reference,
            effect_reference,
            font_reference,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FontReference {
    /// Specifies the identifier of the font to reference.
    pub index: FontCollectionIndex,
    pub color: Option<Color>,
}

impl FontReference {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let index_attr = xml_node
            .attribute("idx")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "idx"))?;
        let index = index_attr.parse()?;

        let color = match xml_node.child_nodes.get(0) {
            Some(clr_node) => Some(Color::from_xml_element(clr_node)?),
            None => None,
        };

        Ok(Self { index, color })
    }
}

#[derive(Debug, Clone)]
pub struct GraphicalObject {
    /// This element specifies the reference to a graphic object within the document. This graphic object is provided
    /// entirely by the document authors who choose to persist this data within the document.
    /// 
    /// # Note
    /// 
    /// Depending on the kind of graphical object used not every generating application that supports the
    /// OOXML framework has the ability to render the graphical object.
    pub graphic_data: GraphicalObjectData,
}

impl GraphicalObject {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let graphic_data_node = xml_node
            .child_nodes
            .get(0)
            .ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "graphicData"))?;
        let graphic_data = GraphicalObjectData::from_xml_element(graphic_data_node)?;

        Ok(Self { graphic_data })
    }
}

#[derive(Debug, Clone)]
pub struct GraphicalObjectData {
    // TODO implement
    //pub graphic_object: Vec<Any>,
    /// Specifies the URI, or uniform resource identifier that represents the data stored under
    /// this tag. The URI is used to identify the correct 'server' that can process the contents of
    /// this tag. 
    pub uri: String,
}

impl GraphicalObjectData {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let uri_attr = xml_node
            .attribute("uri")
            .ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "uri"))?;
        let uri = uri_attr.clone();

        Ok(Self { uri })
    }
}

#[derive(Debug, Clone)]
pub enum AnimationElementChoice {
    /// This element specifies a reference to a diagram that should be animated within a sequence of slide animations.
    /// In addition to simply acting as a reference to a diagram there is also animation build steps defined.
    Diagram(AnimationDgmElement),
    /// This element specifies a reference to a chart that should be animated within a sequence of slide animations. In
    /// addition to simply acting as a reference to a chart there is also animation build steps defined.
    Chart(AnimationChartElement),
}

impl AnimationElementChoice {
    pub fn is_choice_member<T>(name: T) -> bool
    where
        T: AsRef<str>,
    {
        match name.as_ref() {
            "dgm" | "chart" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "dgm" => Ok(AnimationElementChoice::Diagram(AnimationDgmElement::from_xml_element(
                xml_node,
            )?)),
            "chart" => Ok(AnimationElementChoice::Chart(AnimationChartElement::from_xml_element(
                xml_node,
            )?)),
            _ => Err(Box::new(NotGroupMemberError::new(
                xml_node.name.clone(),
                "CT_AnimationElementChoice",
            ))),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AnimationDgmElement {
    /// Specifies the GUID of the shape for this build step in the animation.
    /// 
    /// Defaults to {00000000-0000-0000-0000-000000000000}
    pub id: Option<Guid>,
    /// Specifies which step this part of the diagram should be built using. For instance the
    /// diagram can be built as one object meaning it is animated as a single graphic.
    /// Alternatively the diagram can be animated, or built as separate pieces.
    /// 
    /// Defaults to DgmBuildStep::Shape
    pub build_step: Option<DgmBuildStep>,
}

impl AnimationDgmElement {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "id" => instance.id = Some(value.clone()),
                "bldStep" => instance.build_step = Some(value.parse()?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct AnimationChartElement {
    /// Specifies the index of the series within the corresponding chart that should be animated.
    /// 
    /// Defaults to -1
    pub series_index: Option<i32>,
    /// Specifies the index of the category within the corresponding chart that should be
    /// animated.
    /// 
    /// Defaults to -1
    pub category_index: Option<i32>,
    /// Specifies which step this part of the chart should be built using. For instance the chart can
    /// be built as one object meaning it is animated as a single graphic. Alternatively the chart
    /// can be animated, or built as separate pieces.
    pub build_step: ChartBuildStep,
}

impl AnimationChartElement {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut series_index = None;
        let mut category_index = None;
        let mut build_step = None;

        for (attr, value) in &xml_node.attributes {
            match attr.as_ref() {
                "seriesIdx" => series_index = Some(value.parse()?),
                "categoryIdx" => category_index = Some(value.parse()?),
                "bldStep" => build_step = Some(value.parse()?),
                _ => (),
            }
        }

        let build_step = build_step.ok_or_else(|| MissingAttributeError::new(xml_node.name.clone(), "bldStep"))?;

        Ok(Self {
            series_index,
            category_index,
            build_step,
        })
    }
}

#[derive(Debug, Clone)]
pub enum AnimationGraphicalObjectBuildProperties {
    /// This element specifies how to build the animation for a diagram.
    /// 
    /// # Xml example
    /// 
    /// Consider having a diagram appear as on entity as opposed to by section. The bldDgm element should
    /// be used as follows:
    /// ```xml
    /// <p:bdldLst>
    ///   <p:bldGraphic spid="4" grpId="0">
    ///     <p:bldSub>
    ///       <a:bldDgm bld="one"/>
    ///     </p:bldSub>
    ///   </p:bldGraphic>
    /// </p:bldLst>
    /// ```
    BuildDiagram(AnimationDgmBuildProperties),
    /// This element specifies how to build the animation for a diagram.
    /// 
    /// # Xml example
    /// 
    /// Consider the following example where a chart is specified to be animated by category rather than as
    /// one entity. Thus, the bldChart element should be used as follows:
    /// ```xml
    /// <p:bdldLst>
    ///   <p:bldGraphic spid="4" grpId="0">
    ///     <p:bldSub>
    ///       <a:bldChart bld="category"/>
    ///     </p:bldSub>
    ///   </p:bldGraphic>
    /// </p:bldLst>
    /// ```
    BuildChart(AnimationChartBuildProperties),
}

impl AnimationGraphicalObjectBuildProperties {
    pub fn is_choice_member<T>(name: T) -> bool
    where
        T: AsRef<str>,
    {
        match name.as_ref() {
            "bldDgm" | "bldChart" => true,
            _ => false,
        }
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        match xml_node.local_name() {
            "bldDgm" => Ok(AnimationGraphicalObjectBuildProperties::BuildDiagram(
                AnimationDgmBuildProperties::from_xml_element(xml_node)?,
            )),
            "bldChart" => Ok(AnimationGraphicalObjectBuildProperties::BuildChart(
                AnimationChartBuildProperties::from_xml_element(xml_node)?,
            )),
            _ => Err(Box::new(NotGroupMemberError::new(
                xml_node.name.clone(),
                "CT_AnimationGraphicalObjectBuildProperties",
            ))),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AnimationDgmBuildProperties {
    /// Specifies how the chart is built. The animation animates the sub-elements in the
    /// container in the particular order defined by this attribute.
    /// 
    /// Defaults to AnimationDgmBuildType::AllAtOnce
    pub build_type: Option<AnimationDgmBuildType>,
    /// Specifies whether the animation of the objects in this diagram should be reversed or not.
    /// If this attribute is not specified, a value of false is assumed.
    /// 
    /// Defaults to false
    pub reverse: Option<bool>,
}

impl AnimationDgmBuildProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "bld" => instance.build_type = Some(value.parse()?),
                "rev" => instance.reverse = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Default, Debug, Clone)]
pub struct AnimationChartBuildProperties {
    /// Specifies how the chart is built. The animation animates the sub-elements in the
    /// container in the particular order defined by this attribute.
    /// 
    /// Defaults to AnimationChartBuildType::AllAtOnce
    pub build_type: Option<AnimationChartBuildType>,
    /// Specifies whether or not the chart background elements should be animated as well.
    /// 
    /// Defaults to true
    /// 
    /// # Note
    /// 
    /// An example of background elements are grid lines and the chart legend.
    pub animate_bg: Option<bool>,
}

impl AnimationChartBuildProperties {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for (attr, value) in &xml_node.attributes {
            match attr.as_str() {
                "bld" => instance.build_type = Some(value.parse()?),
                "animBg" => instance.animate_bg = Some(parse_xml_bool(value)?),
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct OfficeStyleSheet {
    pub name: Option<String>,
    /// This element defines the theme formatting options for the theme and is the workhorse of the theme. This is
    /// where the bulk of the shared theme information is contained and used by a document. This element contains
    /// the color scheme, font scheme, and format scheme elements which define the different formatting aspects of
    /// what a theme defines.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <themeElements>
    ///   <clrScheme name="sample">
    ///     ...
    ///   </clrScheme>
    ///   <fontScheme name="sample">
    ///     ...
    ///   </fontScheme>
    ///   <fmtScheme name="sample">
    ///     <fillStyleLst>
    ///       ...
    ///     </fillStyleLst>
    ///     <lnStyleLst>
    ///       ...
    ///     </lnStyleLst>
    ///     <effectStyleLst>
    ///       ...
    ///     </effectStyleLst>
    ///     <bgFillStyleLst>
    ///       ...
    ///     </bgFillStyleLst>
    ///   </fmtScheme>
    /// </themeElements>
    /// ```
    /// 
    /// In this example, we see the basic structure of how a theme elements is defined and have left out the true guts of
    /// each individual piece to save room. Each part (color scheme, font scheme, format scheme) is defined elsewhere
    /// within DrawingML.
    pub theme_elements: Box<BaseStyles>,
    /// This element allows for the definition of default shape, line, and textbox formatting properties. An application
    /// can use this information to format a shape (or text) initially on insertion into a document.
    pub object_defaults: Option<ObjectStyleDefaults>,
    /// This element is a container for the list of extra color schemes present in a document.
    /// 
    /// An ColorSchemeAndMapping element defines an auxiliary color scheme, which includes both a color scheme and
    /// color mapping. This is mainly used for backward compatibility concerns and roundtrips information required by
    /// earlier versions.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <extraClrScheme>
    ///   <clrScheme name="extraColorSchemeSample">
    ///     <dk1>
    ///       <sysClr val="windowText"/>
    ///     </dk1>
    ///     <lt1>
    ///       <sysClr val="window"/>
    ///     </lt1>
    ///     <dk2>
    ///       <srgbClr val="04617B"/>
    ///     </dk2>
    ///     <lt2>
    ///       <srgbClr val="DBF5F9"/>
    ///     </lt2>
    ///     <accent1>
    ///       <srgbClr val="0F6FC6"/>
    ///     </accent1>
    ///     <accent2>
    ///       <srgbClr val="009DD9"/>
    ///     </accent2>
    ///     <accent3>
    ///       <srgbClr val="0BD0D9"/>
    ///     </accent3>
    ///     <accent4>
    ///       <srgbClr val="10CF9B"/>
    ///     </accent4>
    ///     <accent5>
    ///       <srgbClr val="7CCA62"/>
    ///     </accent5>
    ///     <accent6>
    ///       <srgbClr val="A5C249"/>
    ///     </accent6>
    ///     <hlink>
    ///       <srgbClr val="FF9800"/>
    ///     </hlink>
    ///     <folHlink>
    ///       <srgbClr val="F45511"/>
    ///     </folHlink>
    ///   </clrScheme>
    ///   <clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1"
    ///     accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5"
    ///     accent6="accent6" hlink="hlink" folHlink="folHlink"/>
    /// </extraClrScheme>
    /// ```
    pub extra_color_scheme_list: Vec<ColorSchemeAndMapping>,
    /// This element allows for a custom color palette to be created and which shows up alongside other color schemes.
    /// This can be very useful, for example, when someone would like to maintain a corporate color palette.
    pub custom_color_list: Vec<CustomColor>,
}

impl OfficeStyleSheet {
    pub fn from_zip_file(zip_file: &mut ZipFile<'_>) -> Result<Self> {
        let mut xml_string = String::new();
        zip_file.read_to_string(&mut xml_string)?;
        let xml_node = XmlNode::from_str(xml_string.as_str())?;

        Self::from_xml_element(&xml_node)
    }

    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        trace!("parsing OfficeStyleSheet '{}'", xml_node.name);
        let name = xml_node.attribute("name").cloned();
        let mut theme_elements = None;
        let mut object_defaults = None;
        let mut extra_color_scheme_list = Vec::new();
        let mut custom_color_list = Vec::new();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "themeElements" => theme_elements = Some(Box::new(BaseStyles::from_xml_element(child_node)?)),
                "objectDefaults" => object_defaults = Some(ObjectStyleDefaults::from_xml_element(child_node)?),
                "extraClrSchemeLst" => {
                    for extra_color_scheme_node in &child_node.child_nodes {
                        extra_color_scheme_list.push(ColorSchemeAndMapping::from_xml_element(extra_color_scheme_node)?);
                    }
                }
                "custClrLst" => {
                    for cust_color_node in &child_node.child_nodes {
                        custom_color_list.push(CustomColor::from_xml_element(cust_color_node)?);
                    }
                }
                _ => (),
            }
        }

        let theme_elements =
            theme_elements.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "themeElements"))?;

        Ok(Self {
            name,
            theme_elements,
            object_defaults,
            extra_color_scheme_list,
            custom_color_list,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BaseStyles {
    pub color_scheme: Box<ColorScheme>,
    /// This element defines the font scheme within the theme. The font scheme consists of a pair of major and minor
    /// fonts for which to use in a document. The major font corresponds well with the heading areas of a document,
    /// and the minor font corresponds well with the normal text or paragraph areas.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <fontScheme name="sample">
    ///   <majorFont>
    ///   ...
    ///   </majorFont>
    ///   <minorFont>
    ///   ...
    ///   </minorFont>
    /// </fontScheme>
    /// ```
    pub font_scheme: FontScheme,
    /// This element contains the background fill styles, effect styles, fill styles, and line styles which define the style
    /// matrix for a theme. The style matrix consists of subtle, moderate, and intense fills, lines, and effects. The
    /// background fills are not generally thought of to directly be associated with the matrix, but do play a role in the
    /// style of the overall document. Usually, a given object chooses a single line style, a single fill style, and a single
    /// effect style in order to define the overall final look of the object.
    pub format_scheme: Box<StyleMatrix>,
}

impl BaseStyles {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        trace!("parsing BaseStyles '{}'", xml_node.name);
        let mut color_scheme = None;
        let mut font_scheme = None;
        let mut format_scheme = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "clrScheme" => color_scheme = Some(Box::new(ColorScheme::from_xml_element(child_node)?)),
                "fontScheme" => font_scheme = Some(FontScheme::from_xml_element(child_node)?),
                "fmtScheme" => format_scheme = Some(Box::new(StyleMatrix::from_xml_element(child_node)?)),
                _ => (),
            }
        }

        let color_scheme =
            color_scheme.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "clrScheme"))?;
        let font_scheme = font_scheme.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "fontScheme"))?;
        let format_scheme =
            format_scheme.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "fmtScheme"))?;

        Ok(Self {
            color_scheme,
            font_scheme,
            format_scheme,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StyleMatrix {
    /// Defines the name for the format scheme. The name is simply a human readable string
    /// which identifies the format scheme in the user interface.
    pub name: Option<String>,
    /// This element defines a set of three fill styles that are used within a theme. The three fill styles are arranged in
    /// order from subtle to moderate to intense.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <fillStyleLst>
    ///   <solidFill>
    ///   ...
    ///   </solidFill>
    ///   <gradFill rotWithShape="1">
    ///   ...
    ///   </gradFill>
    ///   <gradFill rotWithShape="1">
    ///   ...
    ///   </gradFill>
    /// </fillStyleLst>
    /// ```
    /// 
    /// In this example, we see three fill styles being defined within the fill style list. The first style is the subtle style and
    /// defines simply a solid fill. The second and third styles (moderate and intense fills respectively) define gradient
    /// fills.
    pub fill_style_list: Vec<FillProperties>,
    /// This element defines a list of three line styles for use within a theme. The three line styles are arranged in order
    /// from subtle to moderate to intense versions of lines. This list makes up part of the style matrix.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <lnStyleLst>
    ///   <ln w="9525" cap="flat" cmpd="sng" algn="ctr">
    ///     <solidFill>
    ///       <schemeClr val="phClr">
    ///         <shade val="50000"/>
    ///         <satMod val="103000"/>
    ///       </schemeClr>
    ///     </solidFill>
    ///     <prstDash val="solid"/>
    ///   </ln>
    ///   <ln w="25400" cap="flat" cmpd="sng" algn="ctr">
    ///     <solidFill>
    ///       <schemeClr val="phClr"/>
    ///     </solidFill>
    ///     <prstDash val="solid"/>
    ///   </ln>
    ///   <ln w="38100" cap="flat" cmpd="sng" algn="ctr">
    ///     <solidFill>
    ///       <schemeClr val="phClr"/>
    ///     </solidFill>
    ///     <prstDash val="solid"/>
    ///   </ln>
    /// </lnStyleLst>
    /// ```
    /// 
    /// In this example, we see three lines defined within a line style list. The first line corresponds to the subtle line,
    /// the second to the moderate, and the third corresponds to the intense line defined in the theme.
    pub line_style_list: Vec<Box<LineProperties>>,
    /// This element defines a set of three effect styles that create the effect style list for a theme. The effect styles are
    /// arranged in order of subtle to moderate to intense.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <effectStyleLst>
    ///   <effectStyle>
    ///     <effectLst>
    ///       <outerShdw blurRad="57150" dist="38100" dir="5400000"
    ///       algn="ctr" rotWithShape="0">
    ///       ...
    ///       </outerShdw>
    ///     </effectLst>
    ///   </effectStyle>
    ///   <effectStyle>
    ///     <effectLst>
    ///       <outerShdw blurRad="57150" dist="38100" dir="5400000"
    ///       algn="ctr" rotWithShape="0">
    ///       ...
    ///       </outerShdw>
    ///     </effectLst>
    ///   </effectStyle>
    ///   <effectStyle>
    ///     <effectLst>
    ///       <outerShdw blurRad="57150" dist="38100" dir="5400000"
    ///       algn="ctr" rotWithShape="0">
    ///       ...
    ///       </outerShdw>
    ///     </effectLst>
    ///     <scene3d>
    ///     ...
    ///     </scene3d>
    ///     <sp3d prstMaterial="powder">
    ///     ...
    ///     </sp3d>
    ///   </effectStyle>
    /// </effectStyleLst>
    /// ```
    /// 
    /// In this example, we see three effect styles defined. The first two (subtle and moderate) define an outer shadow
    /// as the effect, while the third effect style (intense) defines an outer shadow along with 3D properties which are
    /// to be applied to the object as well.
    pub effect_style_list: Vec<EffectStyleItem>,
    /// This element defines a list of background fills that are used within a theme. The background fills consist of three
    /// fills, arranged in order from subtle to moderate to intense.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <bgFillStyleLst>
    ///   <solidFill>
    ///   ...
    ///   </solidFill>
    ///   <gradFill rotWithShape="1">
    ///   ...
    ///   </gradFill>
    ///   <blipFill>
    ///   ...
    ///   </blipFill>
    /// </bgFillStyleLst>
    /// ```
    /// 
    /// In this example, we see that the list contains a solid fill for the subtle fill, a gradient fill for the moderate fill and
    /// an image fill for the intense background fill.
    pub bg_fill_style_list: Vec<FillProperties>,
}

impl StyleMatrix {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        trace!("parsing StyleMatrix '{}'", xml_node.name);
        let name = xml_node.attribute("name").cloned();
        let mut fill_style_list = Vec::new();
        let mut line_style_list = Vec::new();
        let mut effect_style_list = Vec::new();
        let mut bg_fill_style_list = Vec::new();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "fillStyleLst" => {
                    for fill_style_node in &child_node.child_nodes {
                        fill_style_list.push(FillProperties::from_xml_element(fill_style_node)?);
                    }
                }
                "lnStyleLst" => {
                    for line_style_node in &child_node.child_nodes {
                        line_style_list.push(Box::new(LineProperties::from_xml_element(line_style_node)?));
                    }
                }
                "effectStyleLst" => {
                    for effect_style_node in &child_node.child_nodes {
                        effect_style_list.push(EffectStyleItem::from_xml_element(effect_style_node)?);
                    }
                }
                "bgFillStyleLst" => {
                    for bg_fill_style_node in &child_node.child_nodes {
                        bg_fill_style_list.push(FillProperties::from_xml_element(bg_fill_style_node)?);
                    }
                }
                _ => (),
            }
        }

        if fill_style_list.len() < 3 {
            return Err(Box::new(LimitViolationError::new(
                xml_node.name.clone(),
                "fillStyleLst",
                Limit::Value(3),
                Limit::Unbounded,
                fill_style_list.len() as u32,
            )));
        }

        if line_style_list.len() < 3 {
            return Err(Box::new(LimitViolationError::new(
                xml_node.name.clone(),
                "lnStyleLst",
                Limit::Value(3),
                Limit::Unbounded,
                line_style_list.len() as u32,
            )));
        }

        if effect_style_list.len() < 3 {
            return Err(Box::new(LimitViolationError::new(
                xml_node.name.clone(),
                "effectStyleLst",
                Limit::Value(3),
                Limit::Unbounded,
                effect_style_list.len() as u32,
            )));
        }

        if bg_fill_style_list.len() < 3 {
            return Err(Box::new(LimitViolationError::new(
                xml_node.name.clone(),
                "bgFillStyleLst",
                Limit::Value(3),
                Limit::Unbounded,
                bg_fill_style_list.len() as u32,
            )));
        }

        Ok(Self {
            name,
            fill_style_list,
            line_style_list,
            effect_style_list,
            bg_fill_style_list,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct ObjectStyleDefaults {
    /// This element defines the formatting that is associated with the default shape. The default formatting can be
    /// applied to a shape when it is initially inserted into a document.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <spDef>
    ///   <spPr>
    ///     <solidFill>
    ///       <schemeClr val="accent2">
    ///         <shade val="75000"/>
    ///       </schemeClr>
    ///     </solidFill>
    ///   </spPr>
    ///   <bodyPr rtlCol="0" anchor="ctr"/>
    ///   <lstStyle>
    ///     <defPPr algn="ctr">
    ///       <defRPr/>
    ///     </defPPr>
    ///   </lstStyle>
    ///   <style>
    ///     <lnRef idx="1">
    ///       <schemeClr val="accent1"/>
    ///     </lnRef>
    ///     <fillRef idx="2">
    ///       <schemeClr val="accent1"/>
    ///     </fillRef>
    ///     <effectRef idx="1">
    ///       <schemeClr val="accent1"/>
    ///     </effectRef>
    ///     <fontRef idx="minor">
    ///       <schemeClr val="dk1"/>
    ///     </fontRef>
    ///   </style>
    /// </spDef>
    /// ```
    /// 
    /// In this example, we see a default shape which references a certain themed fill, line, effect, and font along with
    /// an override fill to these.
    pub shape_definition: Option<Box<DefaultShapeDefinition>>,
    /// This element defines a default line that is used within a document.
    /// 
    /// # Xml example
    /// 
    /// ```xml
    /// <lnDef>
    ///   <spPr/>
    ///   <bodyPr/>
    ///   <lstStyle/>
    ///   <style>
    ///     <lnRef idx="1">
    ///       <schemeClr val="accent2"/>
    ///     </lnRef>
    ///     <fillRef idx="0">
    ///       <schemeClr val="accent2"/>
    ///     </fillRef>
    ///     <effectRef idx="0">
    ///       <schemeClr val="accent2"/>
    ///     </effectRef>
    ///     <fontRef idx="minor">
    ///       <schemeClr val="tx1"/>
    ///     </fontRef>
    ///   </style>
    /// </lnDef>
    /// ```
    /// 
    /// In this example, we see that the default line for the document is being defined as a themed line which
    /// references the subtle line style with idx equal to 1.
    pub line_definition: Option<Box<DefaultShapeDefinition>>,
    /// This element defines the default formatting which is applied to text in a document by default. The default
    /// formatting can and should be applied to the shape when it is initially inserted into a document.
    /// 
    /// ```xml
    /// <txDef>
    ///   <spPr>
    ///     <solidFill>
    ///       <schemeClr val="accent2">
    ///         <shade val="75000"/>
    ///       </schemeClr>
    ///     </solidFill>
    ///   </spPr>
    ///   <bodyPr rtlCol="0" anchor="ctr"/>
    ///   <lstStyle>
    ///     <defPPr algn="ctr">
    ///       <defRPr/>
    ///     </defPPr>
    ///   </lstStyle>
    ///   <style>
    ///     <lnRef idx="1">
    ///       <schemeClr val="accent1"/>
    ///     </lnRef>
    ///     <fillRef idx="2">
    ///       <schemeClr val="accent1"/>
    ///     </fillRef>
    ///     <effectRef idx="1">
    ///       <schemeClr val="accent1"/>
    ///     </effectRef>
    ///     <fontRef idx="minor">
    ///       <schemeClr val="dk1"/>
    ///     </fontRef>
    ///   </style>
    /// </txDef>
    /// ```
    pub text_definition: Option<Box<DefaultShapeDefinition>>,
}

impl ObjectStyleDefaults {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut instance: Self = Default::default();

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "spDef" => {
                    instance.shape_definition = Some(Box::new(DefaultShapeDefinition::from_xml_element(child_node)?))
                }
                "lnDef" => {
                    instance.line_definition = Some(Box::new(DefaultShapeDefinition::from_xml_element(child_node)?))
                }
                "txDef" => {
                    instance.text_definition = Some(Box::new(DefaultShapeDefinition::from_xml_element(child_node)?))
                }
                _ => (),
            }
        }

        Ok(instance)
    }
}

#[derive(Debug, Clone)]
pub struct DefaultShapeDefinition {
    /// This element specifies the visual shape properties that can be applied to a shape.
    pub shape_properties: Box<ShapeProperties>,
    pub text_body_properties: Box<TextBodyProperties>,
    pub text_list_style: Box<TextListStyle>,
    /// This element specifies the style information for a shape.
    pub shape_style: Option<Box<ShapeStyle>>,
}

impl DefaultShapeDefinition {
    pub fn from_xml_element(xml_node: &XmlNode) -> Result<Self> {
        let mut shape_properties = None;
        let mut text_body_properties = None;
        let mut text_list_style = None;
        let mut shape_style = None;

        for child_node in &xml_node.child_nodes {
            match child_node.local_name() {
                "spPr" => shape_properties = Some(Box::new(ShapeProperties::from_xml_element(child_node)?)),
                "bodyPr" => text_body_properties = Some(Box::new(TextBodyProperties::from_xml_element(child_node)?)),
                "lstStyle" => text_list_style = Some(Box::new(TextListStyle::from_xml_element(child_node)?)),
                "style" => shape_style = Some(Box::new(ShapeStyle::from_xml_element(child_node)?)),
                _ => (),
            }
        }

        let shape_properties =
            shape_properties.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "spPr"))?;
        let text_body_properties =
            text_body_properties.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "bodyPr"))?;
        let text_list_style =
            text_list_style.ok_or_else(|| MissingChildNodeError::new(xml_node.name.clone(), "lstStyle"))?;

        Ok(Self {
            shape_properties,
            text_body_properties,
            text_list_style,
            shape_style,
        })
    }
}
