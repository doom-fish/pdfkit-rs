use crate::types::{PdfLineStyle, PdfTextAnnotationIconType};

macro_rules! pdf_string_enum {
    ($(#[$meta:meta])* pub enum $name:ident { $($variant:ident => $raw:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $raw),+
                }
            }

            #[must_use]
            pub fn from_name(raw: &str) -> Option<Self> {
                match raw {
                    $($raw => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

pdf_string_enum! {
    pub enum PdfAnnotationKey {
        AdditionalActions => "/AA",
        AppearanceDictionary => "/AP",
        AppearanceState => "/AS",
        DefaultAppearance => "/DA",
        Destination => "/Dest",
        Flags => "/F",
        HighlightingMode => "/H",
        IconName => "/Name",
        InkList => "/InkList",
        InteriorColor => "/IC",
        LineEndingStyles => "/LE",
        LinePoints => "/L",
        Name => "/NM",
        Open => "/Open",
        Page => "/P",
        Parent => "/Parent",
        Popup => "/Popup",
        QuadPoints => "/QuadPoints",
        Quadding => "/Q",
        TextLabel => "/T",
        WidgetAppearanceDictionary => "/MK",
        WidgetBackgroundColor => "/BG",
        WidgetBorderColor => "/BC",
        WidgetCaption => "/CA",
        WidgetDefaultValue => "/DV",
        WidgetDownCaption => "/AC",
        WidgetFieldFlags => "/Ff",
        WidgetFieldType => "/FT",
        WidgetMaxLen => "/MaxLen",
        WidgetOptions => "/Opt",
        WidgetRolloverCaption => "/RC",
        WidgetRotation => "/R",
        WidgetTextLabelUi => "/TU",
        WidgetValue => "/V"
    }
}

pdf_string_enum! {
    pub enum PdfAnnotationHighlightingMode {
        None => "/N",
        Invert => "/I",
        Outline => "/O",
        Push => "/P"
    }
}

pdf_string_enum! {
    pub enum PdfAnnotationLineEndingStyle {
        None => "/None",
        Square => "/Square",
        Circle => "/Circle",
        Diamond => "/Diamond",
        OpenArrow => "/OpenArrow",
        ClosedArrow => "/ClosedArrow"
    }
}

impl PdfAnnotationLineEndingStyle {
    #[must_use]
    pub const fn from_line_style(style: PdfLineStyle) -> Self {
        match style {
            PdfLineStyle::None => Self::None,
            PdfLineStyle::Square => Self::Square,
            PdfLineStyle::Circle => Self::Circle,
            PdfLineStyle::Diamond => Self::Diamond,
            PdfLineStyle::OpenArrow => Self::OpenArrow,
            PdfLineStyle::ClosedArrow => Self::ClosedArrow,
        }
    }

    #[must_use]
    pub const fn line_style(self) -> PdfLineStyle {
        match self {
            Self::None => PdfLineStyle::None,
            Self::Square => PdfLineStyle::Square,
            Self::Circle => PdfLineStyle::Circle,
            Self::Diamond => PdfLineStyle::Diamond,
            Self::OpenArrow => PdfLineStyle::OpenArrow,
            Self::ClosedArrow => PdfLineStyle::ClosedArrow,
        }
    }
}

pdf_string_enum! {
    pub enum PdfAnnotationSubtype {
        Text => "/Text",
        Link => "/Link",
        FreeText => "/FreeText",
        Line => "/Line",
        Square => "/Square",
        Circle => "/Circle",
        Highlight => "/Highlight",
        Underline => "/Underline",
        StrikeOut => "/StrikeOut",
        Ink => "/Ink",
        Stamp => "/Stamp",
        Popup => "/Popup",
        Widget => "/Widget"
    }
}

pdf_string_enum! {
    pub enum PdfAnnotationTextIconName {
        Comment => "/Comment",
        Key => "/Key",
        Note => "/Note",
        Help => "/Help",
        NewParagraph => "/NewParagraph",
        Paragraph => "/Paragraph",
        Insert => "/Insert"
    }
}

impl PdfAnnotationTextIconName {
    #[must_use]
    pub const fn from_icon_type(icon_type: PdfTextAnnotationIconType) -> Self {
        match icon_type {
            PdfTextAnnotationIconType::Comment => Self::Comment,
            PdfTextAnnotationIconType::Key => Self::Key,
            PdfTextAnnotationIconType::Note => Self::Note,
            PdfTextAnnotationIconType::Help => Self::Help,
            PdfTextAnnotationIconType::NewParagraph => Self::NewParagraph,
            PdfTextAnnotationIconType::Paragraph => Self::Paragraph,
            PdfTextAnnotationIconType::Insert => Self::Insert,
        }
    }

    #[must_use]
    pub const fn icon_type(self) -> PdfTextAnnotationIconType {
        match self {
            Self::Comment => PdfTextAnnotationIconType::Comment,
            Self::Key => PdfTextAnnotationIconType::Key,
            Self::Note => PdfTextAnnotationIconType::Note,
            Self::Help => PdfTextAnnotationIconType::Help,
            Self::NewParagraph => PdfTextAnnotationIconType::NewParagraph,
            Self::Paragraph => PdfTextAnnotationIconType::Paragraph,
            Self::Insert => PdfTextAnnotationIconType::Insert,
        }
    }
}

pdf_string_enum! {
    pub enum PdfAnnotationWidgetSubtype {
        Button => "/Btn",
        Choice => "/Ch",
        Signature => "/Sig",
        Text => "/Tx"
    }
}
