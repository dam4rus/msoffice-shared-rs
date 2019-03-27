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
pub type Percentage = f32;
pub type PositivePercentage = f32; // TODO: 0 <= n < inf
pub type PositiveFixedPercentage = f32; // TODO: 0 <= n <= 100000
pub type FixedPercentage = f32; // TODO: -100000 <= n <= 100000
pub type HexColorRGB = String;
pub type Coordinate = i64;
pub type PositiveCoordinate = u64;
pub type Coordinate32 = i32;
pub type PositiveCoordinate32 = u32;
pub type LineWidth = Coordinate32;
pub type DrawingElementId = u32;
pub type Angle = i32;
pub type FixedAngle = Angle; // TODO: -5400000 <= n <= 5400000
pub type PositiveFixedAngle = Angle; // TODO: 0 <= n <= 21600000
pub type GeomGuideName = String;
pub type GeomGuideFormula = String;
pub type StyleMatrixColumnIndex = u32;
pub type TextColumnCount = i32; // TODO: 1 <= n <= 16
pub type TextFontScalePercent = Percentage; // TODO: 1000 <= n <= 100000
pub type TextSpacingPercent = Percentage; // TODO: 0 <= n <= 13200000
pub type TextSpacingPoint = i32; // TODO: 0 <= n <= 158400
pub type TextMargin = Coordinate32; // TODO: 0 <= n <= 51206400
pub type TextIndent = Coordinate32; // TODO: -51206400 <= n <= 51206400
pub type TextIndentLevelType = i32; // TODO; 0 <= n <= 8
pub type TextBulletSizePercent = Percentage; // TODO: 0.25 <= n <= 4.0
pub type TextFontSize = i32; // TODO: 100 <= n <= 400000
pub type TextTypeFace = String;
pub type TextLanguageID = String;
pub type Panose = String; // TODO: hex, length=10
pub type TextBulletStartAtNum = i32; // TODO: 1 <= n <= 32767
pub type Lang = String;
pub type TextNonNegativePoint = i32; // TODO: 0 <= n <= 400000
pub type TextPoint = i32; // TODO: -400000 <= n <= 400000
pub type ShapeId = String;

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum PathFillMode {
    #[from_str = "none"]
    None,
    #[from_str = "norm"]
    Norm,
    #[from_str = "lighten"]
    Lighten,
    #[from_str = "lightenLess"]
    LightenLess,
    #[from_str = "darken"]
    Darken,
    #[from_str = "darkenLess"]
    DarkenLess,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineCap {
    #[from_str = "rnd"]
    Round,
    #[from_str = "sq"]
    Square,
    #[from_str = "flat"]
    Flat,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum CompoundLine {
    #[from_str = "sng"]
    Single,
    #[from_str = "dbl"]
    Double,
    #[from_str = "thickThin"]
    ThickThin,
    #[from_str = "thinThick"]
    ThinThick,
    #[from_str = "tri"]
    Triple,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum PenAlignment {
    #[from_str = "ctr"]
    Center,
    #[from_str = "in"]
    Inset,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetLineDashVal {
    #[from_str = "solid"]
    Solid,
    #[from_str = "dot"]
    Dot,
    #[from_str = "dash"]
    Dash,
    #[from_str = "lgDash"]
    LgDash,
    #[from_str = "dashDot"]
    DashDot,
    #[from_str = "lgDashDot"]
    LgDashDot,
    #[from_str = "ldDashDotDot"]
    LgDashDotDot,
    #[from_str = "sysDash"]
    SysDash,
    #[from_str = "sysDot"]
    SysDot,
    #[from_str = "sysDashDot"]
    SysDashDot,
    #[from_str = "sysDashDotDot"]
    SysDashDotDot,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineEndWidth {
    #[from_str = "sm"]
    Small,
    #[from_str = "med"]
    Medium,
    #[from_str = "lg"]
    Large,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum LineEndLength {
    #[from_str = "sm"]
    Small,
    #[from_str = "med"]
    Medium,
    #[from_str = "lg"]
    Large,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetShadowVal {
    #[from_str = "shdw1"]
    Shdw1,
    #[from_str = "shdw2"]
    Shdw2,
    #[from_str = "shdw3"]
    Shdw3,
    #[from_str = "shdw4"]
    Shdw4,
    #[from_str = "shdw5"]
    Shdw5,
    #[from_str = "shdw6"]
    Shdw6,
    #[from_str = "shdw7"]
    Shdw7,
    #[from_str = "shdw8"]
    Shdw8,
    #[from_str = "shdw9"]
    Shdw9,
    #[from_str = "shdw10"]
    Shdw10,
    #[from_str = "shdw11"]
    Shdw11,
    #[from_str = "shdw12"]
    Shdw12,
    #[from_str = "shdw13"]
    Shdw13,
    #[from_str = "shdw14"]
    Shdw14,
    #[from_str = "shdw15"]
    Shdw15,
    #[from_str = "shdw16"]
    Shdw16,
    #[from_str = "shdw17"]
    Shdw17,
    #[from_str = "shdw18"]
    Shdw18,
    #[from_str = "shdw19"]
    Shdw19,
    #[from_str = "shdw20"]
    Shdw20,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum EffectContainerType {
    #[from_str = "sib"]
    Sib,
    #[from_str = "tree"]
    Tree,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum FontCollectionIndex {
    #[from_str = "major"]
    Major,
    #[from_str = "minor"]
    Minor,
    #[from_str = "none"]
    None,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum DgmBuildStep {
    #[from_str = "sp"]
    Shape,
    #[from_str = "bg"]
    Background,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum ChartBuildStep {
    #[from_str = "category"]
    Category,
    #[from_str = "ptInCategory"]
    PtInCategory,
    #[from_str = "series"]
    Series,
    #[from_str = "ptInSeries"]
    PtInSeries,
    #[from_str = "allPts"]
    AllPts,
    #[from_str = "gridLegend"]
    GridLegend,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum OnOffStyleType {
    #[from_str = "on"]
    On,
    #[from_str = "off"]
    Off,
    #[from_str = "def"]
    Def,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum SystemColorVal {
    #[from_str = "scrollBar"]
    ScrollBar,
    #[from_str = "background"]
    Background,
    #[from_str = "activeCaption"]
    ActiveCaption,
    #[from_str = "inactiveCaption"]
    InactiveCaption,
    #[from_str = "menu"]
    Menu,
    #[from_str = "window"]
    Window,
    #[from_str = "windowFrame"]
    WindowFrame,
    #[from_str = "menuText"]
    MenuText,
    #[from_str = "windowText"]
    WindowText,
    #[from_str = "captionText"]
    CaptionText,
    #[from_str = "activeBorder"]
    ActiveBorder,
    #[from_str = "inactiveBorder"]
    InactiveBorder,
    #[from_str = "appWorkspace"]
    AppWorkspace,
    #[from_str = "highlight"]
    Highlight,
    #[from_str = "highlightText"]
    HighlightText,
    #[from_str = "btnFace"]
    BtnFace,
    #[from_str = "btnShadow"]
    BtnShadow,
    #[from_str = "grayText"]
    GrayText,
    #[from_str = "btnText"]
    BtnText,
    #[from_str = "inactiveCaptionText"]
    InactiveCaptionText,
    #[from_str = "btnHighlight"]
    BtnHighlight,
    #[from_str = "3dDkShadow"]
    DkShadow3d,
    #[from_str = "3dLight"]
    Light3d,
    #[from_str = "infoText"]
    InfoText,
    #[from_str = "infoBk"]
    InfoBk,
    #[from_str = "hotLight"]
    HotLight,
    #[from_str = "gradientActiveCaption"]
    GradientActiveCaption,
    #[from_str = "gradientInactiveCaption"]
    GradientInactiveCaption,
    #[from_str = "menuHighlight"]
    MenuHighlight,
    #[from_str = "menubar"]
    MenuBar,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum PresetColorVal {
    #[from_str = "aliceBlue"]
    AliceBlue,
    #[from_str = "antiqueWhite"]
    AntiqueWhite,
    #[from_str = "aqua"]
    Aqua,
    #[from_str = "aquamarine"]
    Aquamarine,
    #[from_str = "azure"]
    Azure,
    #[from_str = "beige"]
    Beige,
    #[from_str = "bisque"]
    Bisque,
    #[from_str = "black"]
    Black,
    #[from_str = "blanchedAlmond"]
    BlanchedAlmond,
    #[from_str = "blue"]
    Blue,
    #[from_str = "blueViolet"]
    BlueViolet,
    #[from_str = "brown"]
    Brown,
    #[from_str = "burlyWood"]
    BurlyWood,
    #[from_str = "cadetBlue"]
    CadetBlue,
    #[from_str = "chartreuse"]
    Chartreuse,
    #[from_str = "chocolate"]
    Chocolate,
    #[from_str = "coral"]
    Coral,
    #[from_str = "cornflowerBlue"]
    CornflowerBlue,
    #[from_str = "cornsilk"]
    Cornsilk,
    #[from_str = "crimson"]
    Crimson,
    #[from_str = "cyan"]
    Cyan,
    #[from_str = "darkBlue"]
    DarkBlue,
    #[from_str = "darkCyan"]
    DarkCyan,
    #[from_str = "darkGoldenrod"]
    DarkGoldenrod,
    #[from_str = "darkGray"]
    DarkGray,
    #[from_str = "darkGrey"]
    DarkGrey,
    #[from_str = "darkGreen"]
    DarkGreen,
    #[from_str = "darkKhaki"]
    DarkKhaki,
    #[from_str = "darkMagenta"]
    DarkMagenta,
    #[from_str = "darkOliveGreen"]
    DarkOliveGreen,
    #[from_str = "darkOrange"]
    DarkOrange,
    #[from_str = "darkOrchid"]
    DarkOrchid,
    #[from_str = "darkRed"]
    DarkRed,
    #[from_str = "darkSalmon"]
    DarkSalmon,
    #[from_str = "darkSeaGreen"]
    DarkSeaGreen,
    #[from_str = "darkSlateBlue"]
    DarkSlateBlue,
    #[from_str = "darkSlateGray"]
    DarkSlateGray,
    #[from_str = "darkSlateGrey"]
    DarkSlateGrey,
    #[from_str = "darkTurquoise"]
    DarkTurqoise,
    #[from_str = "darkViolet"]
    DarkViolet,
    #[from_str = "dkBlue"]
    DkBlue,
    #[from_str = "dkCyan"]
    DkCyan,
    #[from_str = "dkGoldenrod"]
    DkGoldenrod,
    #[from_str = "dkGray"]
    DkGray,
    #[from_str = "dkGrey"]
    DkGrey,
    #[from_str = "dkGreen"]
    DkGreen,
    #[from_str = "dkKhaki"]
    DkKhaki,
    #[from_str = "dkMagenta"]
    DkMagenta,
    #[from_str = "dkOliveGreen"]
    DkOliveGreen,
    #[from_str = "dkOrange"]
    DkOrange,
    #[from_str = "dkOrchid"]
    DkOrchid,
    #[from_str = "dkRed"]
    DkRed,
    #[from_str = "dkSalmon"]
    DkSalmon,
    #[from_str = "dkSeaGreen"]
    DkSeaGreen,
    #[from_str = "dkSlateBlue"]
    DkSlateBlue,
    #[from_str = "dkSlateGray"]
    DkSlateGray,
    #[from_str = "dkSlateGrey"]
    DkSlateGrey,
    #[from_str = "dkTurquoise"]
    DkTurquoise,
    #[from_str = "dkViolet"]
    DkViolet,
    #[from_str = "deepPink"]
    DeepPink,
    #[from_str = "deepSkyBlue"]
    DeepSkyBlue,
    #[from_str = "dimGray"]
    DimGray,
    #[from_str = "dimGrey"]
    DimGrey,
    #[from_str = "dodgerBlue"]
    DodgerBluet,
    #[from_str = "firebrick"]
    Firebrick,
    #[from_str = "floralWhite"]
    FloralWhite,
    #[from_str = "forestGreen"]
    ForestGreen,
    #[from_str = "fuchsia"]
    Fuchsia,
    #[from_str = "gainsboro"]
    Gainsboro,
    #[from_str = "ghostWhite"]
    GhostWhite,
    #[from_str = "gold"]
    Gold,
    #[from_str = "goldenrod"]
    Goldenrod,
    #[from_str = "gray"]
    Gray,
    #[from_str = "grey"]
    Grey,
    #[from_str = "green"]
    Green,
    #[from_str = "greenYellow"]
    GreenYellow,
    #[from_str = "honeydew"]
    Honeydew,
    #[from_str = "hotPink"]
    HotPink,
    #[from_str = "indianRed"]
    IndianRed,
    #[from_str = "indigo"]
    Indigo,
    #[from_str = "ivory"]
    Ivory,
    #[from_str = "khaki"]
    Khaki,
    #[from_str = "lavender"]
    Lavender,
    #[from_str = "lavenderBlush"]
    LavenderBlush,
    #[from_str = "lawnGreen"]
    LawnGreen,
    #[from_str = "lemonChiffon"]
    LemonChiffon,
    #[from_str = "lightBlue"]
    LightBlue,
    #[from_str = "lightCoral"]
    LightCoral,
    #[from_str = "lightCyan"]
    LightCyan,
    #[from_str = "lightGoldenrodYellow"]
    LightGoldenrodYellow,
    #[from_str = "lightGray"]
    LightGray,
    #[from_str = "lightGrey"]
    LightGrey,
    #[from_str = "lightGreen"]
    LightGreen,
    #[from_str = "lightPink"]
    LightPink,
    #[from_str = "lightSalmon"]
    LightSalmon,
    #[from_str = "lightSeaGreen"]
    LightSeaGreen,
    #[from_str = "lightSkyBlue"]
    LightSkyBlue,
    #[from_str = "lightSlateGray"]
    LightSlateGray,
    #[from_str = "lightSlateGrey"]
    LightSlateGrey,
    #[from_str = "lightSteelBlue"]
    LightSteelBlue,
    #[from_str = "lightYellow"]
    LightYellow,
    #[from_str = "ltBlue"]
    LtBlue,
    #[from_str = "ltCoral"]
    LtCoral,
    #[from_str = "ltCyan"]
    LtCyan,
    #[from_str = "ltGoldenrodYellow"]
    LtGoldenrodYellow,
    #[from_str = "ltGray"]
    LtGray,
    #[from_str = "ltGrey"]
    LtGrey,
    #[from_str = "ltGreen"]
    LtGreen,
    #[from_str = "ltPink"]
    LtPink,
    #[from_str = "ltSalmon"]
    LtSalmon,
    #[from_str = "ltSeaGreen"]
    LtSeaGreen,
    #[from_str = "ltSkyBlue"]
    LtSkyBlue,
    #[from_str = "ltSlateGray"]
    LtSlateGray,
    #[from_str = "ltSlateGrey"]
    LtSlateGrey,
    #[from_str = "ltSteelBlue"]
    LtSteelBlue,
    #[from_str = "ltYellow"]
    LtYellow,
    #[from_str = "lime"]
    Lime,
    #[from_str = "limeGreen"]
    LimeGreen,
    #[from_str = "linen"]
    Linen,
    #[from_str = "magenta"]
    Magenta,
    #[from_str = "maroon"]
    Maroon,
    #[from_str = "medAquamarine"]
    MedAquamarine,
    #[from_str = "medBlue"]
    MedBlue,
    #[from_str = "medOrchid"]
    MedOrchid,
    #[from_str = "medPurple"]
    MedPurple,
    #[from_str = "medSeaGreen"]
    MedSeaGreen,
    #[from_str = "medSlateBlue"]
    MedSlateBlue,
    #[from_str = "medSpringGreen"]
    MedSpringGreen,
    #[from_str = "medTurquoise"]
    MedTurquoise,
    #[from_str = "medVioletRed"]
    MedVioletRed,
    #[from_str = "mediumAquamarine"]
    MediumAquamarine,
    #[from_str = "mediumBlue"]
    MediumBlue,
    #[from_str = "mediumOrchid"]
    MediumOrchid,
    #[from_str = "mediumPurple"]
    MediumPurple,
    #[from_str = "mediumSeaGreen"]
    MediumSeaGreen,
    #[from_str = "mediumSlateBlue"]
    MediumSlateBlue,
    #[from_str = "mediumSpringGreen"]
    MediumSpringGreen,
    #[from_str = "mediumTurquoise"]
    MediumTurquoise,
    #[from_str = "mediumVioletRed"]
    MediumVioletRed,
    #[from_str = "midnightBlue"]
    MidnightBlue,
    #[from_str = "mintCream"]
    MintCream,
    #[from_str = "mistyRose"]
    MistyRose,
    #[from_str = "moccasin"]
    Moccasin,
    #[from_str = "navajoWhite"]
    NavajoWhite,
    #[from_str = "navy"]
    Navy,
    #[from_str = "oldLace"]
    OldLace,
    #[from_str = "olive"]
    Olive,
    #[from_str = "oliveDrab"]
    OliveDrab,
    #[from_str = "orange"]
    Orange,
    #[from_str = "orangeRed"]
    OrangeRed,
    #[from_str = "orchid"]
    Orchid,
    #[from_str = "paleGoldenrod"]
    PaleGoldenrod,
    #[from_str = "paleGreen"]
    PaleGreen,
    #[from_str = "paleTurquoise"]
    PaleTurquoise,
    #[from_str = "paleVioletRed"]
    PaleVioletRed,
    #[from_str = "papayaWhip"]
    PapayaWhip,
    #[from_str = "peachPuff"]
    PeachPuff,
    #[from_str = "peru"]
    Peru,
    #[from_str = "pink"]
    Pink,
    #[from_str = "plum"]
    Plum,
    #[from_str = "powderBlue"]
    PowderBlue,
    #[from_str = "purple"]
    Purple,
    #[from_str = "red"]
    Red,
    #[from_str = "rosyBrown"]
    RosyBrown,
    #[from_str = "royalBlue"]
    RoyalBlue,
    #[from_str = "saddleBrown"]
    SaddleBrown,
    #[from_str = "salmon"]
    Salmon,
    #[from_str = "sandyBrown"]
    SandyBrown,
    #[from_str = "seaGreen"]
    SeaGreen,
    #[from_str = "seaShell"]
    SeaShell,
    #[from_str = "sienna"]
    Sienna,
    #[from_str = "silver"]
    Silver,
    #[from_str = "skyBlue"]
    SkyBlue,
    #[from_str = "slateBlue"]
    SlateBlue,
    #[from_str = "slateGray"]
    SlateGray,
    #[from_str = "slateGrey"]
    SlateGrey,
    #[from_str = "snow"]
    Snow,
    #[from_str = "springGreen"]
    SpringGreen,
    #[from_str = "steelBlue"]
    SteelBlue,
    #[from_str = "tan"]
    Tan,
    #[from_str = "teal"]
    Teal,
    #[from_str = "thistle"]
    Thistle,
    #[from_str = "tomato"]
    Tomato,
    #[from_str = "turquoise"]
    Turquoise,
    #[from_str = "violet"]
    Violet,
    #[from_str = "wheat"]
    Wheat,
    #[from_str = "white"]
    White,
    #[from_str = "whiteSmoke"]
    WhiteSmoke,
    #[from_str = "yellow"]
    Yellow,
    #[from_str = "yellowGreen"]
    YellowGreen,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextAlignType {
    #[from_str = "l"]
    Left,
    #[from_str = "ctr"]
    Center,
    #[from_str = "r"]
    Right,
    #[from_str = "just"]
    Justified,
    #[from_str = "justLow"]
    JustifiedLow,
    #[from_str = "dist"]
    Distritbuted,
    #[from_str = "thaiDist"]
    ThaiDistributed,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextFontAlignType {
    #[from_str = "auto"]
    Auto,
    #[from_str = "t"]
    Top,
    #[from_str = "ctr"]
    Center,
    #[from_str = "base"]
    Baseline,
    #[from_str = "b"]
    Bottom,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextAutonumberScheme {
    #[from_str = "alphaLcParenBoth"]
    AlphaLcParenBoth,
    #[from_str = "alphaUcParenBoth"]
    AlphaUcParenBoth,
    #[from_str = "alphaLcParenR"]
    AlphaLcParenR,
    #[from_str = "alphaUcParenR"]
    AlphaUcParenR,
    #[from_str = "alphaLcPeriod"]
    AlphaLcPeriod,
    #[from_str = "alphaUcPeriod"]
    AlphaUcPeriod,
    #[from_str = "arabicParenBoth"]
    ArabicParenBoth,
    #[from_str = "arabicParenR"]
    ArabicParenR,
    #[from_str = "arabicPeriod"]
    ArabicPeriod,
    #[from_str = "arabicPlain"]
    ArabicPlain,
    #[from_str = "romanLcParenBoth"]
    RomanLcParenBoth,
    #[from_str = "romanUcParenBoth"]
    RomanUcParenBoth,
    #[from_str = "romanLcParenR"]
    RomanLcParenR,
    #[from_str = "romanUcParenR"]
    RomanUcParenR,
    #[from_str = "romanLcPeriod"]
    RomanLcPeriod,
    #[from_str = "romanUcPeriod"]
    RomanUcPeriod,
    #[from_str = "circleNumDbPlain"]
    CircleNumDbPlain,
    #[from_str = "circleNumWdBlackPlain"]
    CircleNumWdBlackPlain,
    #[from_str = "circleNumWdWhitePlain"]
    CircleNumWdWhitePlain,
    #[from_str = "arabicDbPeriod"]
    ArabicDbPeriod,
    #[from_str = "arabicDbPlain"]
    ArabicDbPlain,
    #[from_str = "ea1ChsPeriod"]
    Ea1ChsPeriod,
    #[from_str = "ea1ChsPlain"]
    Ea1ChsPlain,
    #[from_str = "ea1ChtPeriod"]
    Ea1ChtPeriod,
    #[from_str = "ea1ChtPlain"]
    Ea1ChtPlain,
    #[from_str = "ea1JpnChsDbPeriod"]
    Ea1JpnChsDbPeriod,
    #[from_str = "ea1JpnKorPlain"]
    Ea1JpnKorPlain,
    #[from_str = "ea1JpnKorPeriod"]
    Ea1JpnKorPeriod,
    #[from_str = "arabic1Minus"]
    Arabic1Minus,
    #[from_str = "arabic2Minus"]
    Arabic2Minus,
    #[from_str = "hebrew2Minus"]
    Hebrew2Minus,
    #[from_str = "thaiAlphaPeriod"]
    ThaiAlphaPeriod,
    #[from_str = "thaiAlphaParenR"]
    ThaiAlphaParenR,
    #[from_str = "thaiAlphaParenBoth"]
    ThaiAlphaParenBoth,
    #[from_str = "thaiNumPeriod"]
    ThaiNumPeriod,
    #[from_str = "thaiNumParenR"]
    ThaiNumParenR,
    #[from_str = "thaiNumParenBoth"]
    ThaiNumParenBoth,
    #[from_str = "hindiAlphaPeriod"]
    HindiAlphaPeriod,
    #[from_str = "hindiNumPeriod"]
    HindiNumPeriod,
    #[from_str = "hindiNumParenR"]
    HindiNumParenR,
    #[from_str = "hindiAlpha1Period"]
    HindiAlpha1Period,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum PathShadeType {
    #[from_str = "shape"]
    Shape,
    #[from_str = "circle"]
    Circle,
    #[from_str = "rect"]
    Rect,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextTabAlignType {
    #[from_str = "l"]
    Left,
    #[from_str = "ctr"]
    Center,
    #[from_str = "r"]
    Right,
    #[from_str = "dec"]
    Decimal,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextUnderlineType {
    #[from_str = "none"]
    None,
    #[from_str = "words"]
    Words,
    #[from_str = "sng"]
    Single,
    #[from_str = "dbl"]
    Double,
    #[from_str = "heavy"]
    Heavy,
    #[from_str = "dotted"]
    Dotted,
    #[from_str = "dottedHeavy"]
    DottedHeavy,
    #[from_str = "dash"]
    Dash,
    #[from_str = "dashHeavy"]
    DashHeavy,
    #[from_str = "dashLong"]
    DashLong,
    #[from_str = "dashLongHeavy"]
    DashLongHeavy,
    #[from_str = "dotDash"]
    DotDash,
    #[from_str = "dotDashHeavy"]
    DotDashHeavy,
    #[from_str = "dotDotDash"]
    DotDotDash,
    #[from_str = "dotDotDashHeavy"]
    DotDotDashHeavy,
    #[from_str = "wavy"]
    Wavy,
    #[from_str = "wavyHeavy"]
    WavyHeavy,
    #[from_str = "wavyDbl"]
    WavyDouble,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextStrikeType {
    #[from_str = "noStrike"]
    NoStrike,
    #[from_str = "sngStrike"]
    SingleStrike,
    #[from_str = "dblStrike"]
    DoubleStrike,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextCapsType {
    #[from_str = "none"]
    None,
    #[from_str = "small"]
    Small,
    #[from_str = "all"]
    All,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextVertOverflowType {
    #[from_str = "overflow"]
    Overflow,
    #[from_str = "ellipsis"]
    Ellipsis,
    #[from_str = "clip"]
    Clip,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextHorzOverflowType {
    #[from_str = "overflow"]
    Overflow,
    #[from_str = "clip"]
    Clip,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextVerticalType {
    #[from_str = "horz"]
    Horizontal,
    #[from_str = "vert"]
    Vertical,
    #[from_str = "vert270"]
    Vertical270,
    #[from_str = "wordArtVert"]
    WordArtVertical,
    #[from_str = "eaVert"]
    EastAsianVertical,
    #[from_str = "mongolianVert"]
    MongolianVertical,
    #[from_str = "wordArtVertRtl"]
    WordArtVerticalRtl,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextWrappingType {
    #[from_str = "none"]
    None,
    #[from_str = "square"]
    Square,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum TextAnchoringType {
    #[from_str = "t"]
    Top,
    #[from_str = "ctr"]
    Center,
    #[from_str = "b"]
    Bottom,
    #[from_str = "just"]
    Justified,
    #[from_str = "dist"]
    Distributed,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum BlackWhiteMode {
    #[from_str = "clr"]
    Color,
    #[from_str = "auto"]
    Auto,
    #[from_str = "gray"]
    Gray,
    #[from_str = "ltGray"]
    LightGray,
    #[from_str = "invGray"]
    InverseGray,
    #[from_str = "grayWhite"]
    GrayWhite,
    #[from_str = "blackGray"]
    BlackGray,
    #[from_str = "blackWhite"]
    BlackWhite,
    #[from_str = "black"]
    Black,
    #[from_str = "white"]
    White,
    #[from_str = "hidden"]
    Hidden,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationBuildType {
    #[from_str = "allAtOnce"]
    AllAtOnce,
}

#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationDgmOnlyBuildType {
    #[from_str = "one"]
    One,
    #[from_str = "lvlOne"]
    LvlOne,
    #[from_str = "lvlAtOnce"]
    LvlAtOnce,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum AnimationChartOnlyBuildType {
    #[from_str = "series"]
    Series,
    #[from_str = "category"]
    Category,
    #[from_str = "seriesElement"]
    SeriesElement,
    #[from_str = "categoryElement"]
    CategoryElement,
}

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

#[derive(Debug, Clone, Copy, FromStr)]
pub enum BlipCompression {
    #[from_str = "email"]
    Email,
    #[from_str = "screen"]
    Screen,
    #[from_str = "print"]
    Print,
    #[from_str = "hqprint"]
    HqPrint,
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
    /// |:Sequence Index       :|:Element (Color) Name             :|
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
    pub kumimoji: Option<bool>,
    pub language: Option<TextLanguageID>,
    pub alternative_language: Option<TextLanguageID>,
    pub font_size: Option<TextFontSize>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<TextUnderlineType>,
    pub strikethrough: Option<TextStrikeType>,
    pub kerning: Option<TextNonNegativePoint>,
    pub caps_type: Option<TextCapsType>,
    pub spacing: Option<TextPoint>,
    pub normalize_heights: Option<bool>,
    pub baseline: Option<Percentage>,
    pub no_proofing: Option<bool>,
    pub dirty: Option<bool>,          // true
    pub spelling_error: Option<bool>, // false
    pub smarttag_clean: Option<bool>, // true
    pub smarttag_id: Option<u32>,     // 0
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
                "cap" => instance.caps_type = Some(value.parse()?),
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
    pub margin_left: Option<TextMargin>,
    pub margin_right: Option<TextMargin>,
    pub level: Option<TextIndentLevelType>,
    pub indent: Option<TextIndent>,
    pub align: Option<TextAlignType>,
    pub default_tab_size: Option<Coordinate32>,
    pub rtl: Option<bool>,
    pub east_asian_line_break: Option<bool>,
    pub font_align: Option<TextFontAlignType>,
    pub latin_line_break: Option<bool>,
    pub hanging_punctuations: Option<bool>,
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
    pub properties: Option<Box<TextParagraphProperties>>,
    pub text_run_list: Vec<TextRun>,
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
    LineBreak(Box<TextLineBreak>),
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
    pub id: Guid,
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
    pub rotate_angle: Option<Angle>,
    pub paragraph_spacing: Option<bool>,
    pub vertical_overflow: Option<TextVertOverflowType>,
    pub horizontal_overflow: Option<TextHorzOverflowType>,
    pub vertical_type: Option<TextVerticalType>,
    pub wrap_type: Option<TextWrappingType>,
    pub left_inset: Option<Coordinate32>,
    pub top_inset: Option<Coordinate32>,
    pub right_inset: Option<Coordinate32>,
    pub bottom_inset: Option<Coordinate32>,
    pub column_count: Option<TextColumnCount>,
    pub space_between_columns: Option<PositiveCoordinate32>,
    pub rtl_columns: Option<bool>,
    pub is_from_word_art: Option<bool>,
    pub anchor: Option<TextAnchoringType>,
    pub anchor_center: Option<bool>,
    pub force_antialias: Option<bool>,
    pub upright: Option<bool>,
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
                "rot" => instance.rotate_angle = Some(value.parse::<Angle>()?),
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
    NoAutoFit,
    NormalAutoFit(TextNormalAutoFit),
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
    pub font_scale: Option<TextFontScalePercent>,           // 100000
    pub line_spacing_reduction: Option<TextSpacingPercent>, // 0
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
    pub id: DrawingElementId,
    pub name: String,
    pub description: Option<String>,
    pub hidden: Option<bool>, // false
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
    pub prefer_relative_resize: Option<bool>, // true
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
