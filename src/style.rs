use crate::color::AnsiColor;

pub enum AnsiStyle {
    ForegroundColor(AnsiColor),
    Bold,
    Italics,
    Underline,
}

impl AnsiStyle {
    pub const fn code(&self) -> &str {
        match self {
            Self::ForegroundColor(color) => color.code(),
            Self::Bold => "1",
            Self::Italics => "3",
            Self::Underline => "4",
        }
    }
}
