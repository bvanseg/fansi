use crate::{style::AnsiStyle};

pub struct AnsiStyleContainer {
    pub styles: Vec<AnsiStyle>,
    compiled_styles: String
}

impl AnsiStyleContainer {
    pub fn from_vec(styles: Vec<AnsiStyle>) -> Self {
        let compiled_styles = styles.iter().map(AnsiStyle::code).collect();
        
        return AnsiStyleContainer {
            styles: styles,
            compiled_styles: compiled_styles
        }
    }

    pub fn from_arr(styles: &[AnsiStyle]) -> Self {
        return AnsiStyleContainer::from_vec(styles.to_vec());
    }

    pub fn apply(&self, text: &str) -> String {
        format!("\x1b[{}m{}\x1b[{}", self.compiled_styles, text, "0m")
    }
}