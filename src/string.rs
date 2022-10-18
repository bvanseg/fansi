use std::fmt;

use crate::style::AnsiStyle;

pub struct AnsiString {
    pub text: String,
    pub styles: Vec<String>,
}

impl AnsiString {
    pub fn with_styles_arr(text: &str, styles: &[AnsiStyle]) -> Self {
        return AnsiString::with_styles_vec(text, styles.to_vec());
    }

    pub fn with_styles_vec(text: &str, styles: Vec<AnsiStyle>) -> Self {
        AnsiString {
            text: text.to_string(),
            styles: styles.iter().map(|f| f.code().to_string()).collect(),
        }
    }

    pub fn as_string(&self) -> String {
        format!("\x1b[{}m{}\x1b[{}", self.styles.join(";"), self.text, "0m")
    }
}

impl fmt::Display for AnsiString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_string())
    }
}
