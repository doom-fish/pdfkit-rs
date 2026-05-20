use crate::types::{PdfLineStyle, PdfTextAnnotationIconType};

macro_rules! pdf_string_enum {
    ($(#[$meta:meta])* pub enum $name:ident { $($variant:ident => $raw:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[doc = concat!("Wraps `", stringify!($name), "` values.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = concat!("Wraps the corresponding `", stringify!($name), "` value.")]
                $variant
            ),+
        }

        impl $name {
            #[doc = "Returns the corresponding PDFKit string constant."]
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $raw),+
                }
            }

            #[doc = "Parses the corresponding PDFKit string constant."]
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
    /// Converts a `PDFLineStyle`-backed value into `PDFAnnotationLineEndingStyle`.
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

    /// Converts this value back to the corresponding `PDFLineStyle` case.
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
    /// Converts a `PDFTextAnnotationIconType`-backed value into `PDFAnnotationTextIconName`.
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

    /// Converts this value back to the corresponding `PDFTextAnnotationIconType` case.
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

#[cfg(test)]
mod tests {
    use super::{
        PdfAnnotationKey, PdfAnnotationLineEndingStyle, PdfAnnotationTextIconName,
        PdfAnnotationWidgetSubtype,
    };
    use crate::types::{PdfLineStyle, PdfTextAnnotationIconType};

    #[test]
    fn annotation_keys_round_trip_through_names() {
        assert_eq!(PdfAnnotationKey::WidgetValue.name(), "/V");
        assert_eq!(PdfAnnotationKey::from_name("/AA"), Some(PdfAnnotationKey::AdditionalActions));
        assert_eq!(PdfAnnotationKey::from_name("/missing"), None);
    }

    #[test]
    fn line_ending_styles_round_trip_with_pdf_line_styles() {
        let cases = [
            (PdfLineStyle::None, PdfAnnotationLineEndingStyle::None),
            (PdfLineStyle::Square, PdfAnnotationLineEndingStyle::Square),
            (PdfLineStyle::Circle, PdfAnnotationLineEndingStyle::Circle),
            (PdfLineStyle::Diamond, PdfAnnotationLineEndingStyle::Diamond),
            (PdfLineStyle::OpenArrow, PdfAnnotationLineEndingStyle::OpenArrow),
            (PdfLineStyle::ClosedArrow, PdfAnnotationLineEndingStyle::ClosedArrow),
        ];

        for (line_style, annotation_style) in cases {
            assert_eq!(PdfAnnotationLineEndingStyle::from_line_style(line_style), annotation_style);
            assert_eq!(annotation_style.line_style(), line_style);
            assert_eq!(PdfAnnotationLineEndingStyle::from_name(annotation_style.name()), Some(annotation_style));
        }
    }

    #[test]
    fn text_icon_names_round_trip_with_icon_types() {
        let cases = [
            (PdfTextAnnotationIconType::Comment, PdfAnnotationTextIconName::Comment),
            (PdfTextAnnotationIconType::Key, PdfAnnotationTextIconName::Key),
            (PdfTextAnnotationIconType::Note, PdfAnnotationTextIconName::Note),
            (PdfTextAnnotationIconType::Help, PdfAnnotationTextIconName::Help),
            (PdfTextAnnotationIconType::NewParagraph, PdfAnnotationTextIconName::NewParagraph),
            (PdfTextAnnotationIconType::Paragraph, PdfAnnotationTextIconName::Paragraph),
            (PdfTextAnnotationIconType::Insert, PdfAnnotationTextIconName::Insert),
        ];

        for (icon_type, icon_name) in cases {
            assert_eq!(PdfAnnotationTextIconName::from_icon_type(icon_type), icon_name);
            assert_eq!(icon_name.icon_type(), icon_type);
            assert_eq!(PdfAnnotationTextIconName::from_name(icon_name.name()), Some(icon_name));
        }
    }

    #[test]
    fn widget_subtypes_round_trip_through_names() {
        assert_eq!(PdfAnnotationWidgetSubtype::Button.name(), "/Btn");
        assert_eq!(PdfAnnotationWidgetSubtype::from_name("/Tx"), Some(PdfAnnotationWidgetSubtype::Text));
        assert_eq!(PdfAnnotationWidgetSubtype::from_name("/Other"), None);
    }
}
