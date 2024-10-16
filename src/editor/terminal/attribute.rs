use crossterm::style::Color;

use super::super::AnnotationType;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 211,
                    g: 211,
                    b: 211,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 153,
                }),
            },
            AnnotationType::Number => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 99,
                    b: 71,
                }),
                background: None,
            },
            AnnotationType::KeyWord => Self {
                foreground: Some(Color::Rgb {
                    r: 100,
                    g: 149,
                    b: 237,
                }),
                background: None,
            },
            AnnotationType::Type => Self {
                foreground: Some(Color::Rgb {
                    r: 175,
                    g: 225,
                    b: 175,
                }),
                background: None,
            },
            AnnotationType::KnownValue => Self {
                foreground: Some(Color::Rgb {
                    r: 195,
                    g: 177,
                    b: 225,
                }),
                background: None,
            },
        }
    }
}
