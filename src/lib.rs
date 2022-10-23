pub mod color;
pub mod container;
pub mod string;
pub mod style;
pub mod windows;

#[cfg(test)]
mod tests {
    use crate::{color::AnsiColor, style::AnsiStyle, string::AnsiString, container::AnsiStyleContainer};

    #[cfg(windows)]
    use crate::{windows::enable_ansi_support};

    #[test]
    fn test_bold() {
        // Create style.
        let style = vec![AnsiStyle::Bold];
        // Create text with style.
        let text = AnsiString::with_styles_vec("world!", style);
        // Print text.
        println!("Hello, {}", text);
    }

    #[test]
    fn test_colors() {
        // Create style.
        let style = vec![AnsiStyle::ForegroundColor(AnsiColor::Green)];
        // Create text with style.
        let text = AnsiString::with_styles_vec("world!", style);
        // Print text.
        println!("Hello, {}", text);
    }

    #[test]
    fn test_containers() {
        // Create styles.
        let style = vec![AnsiStyle::ForegroundColor(AnsiColor::Red)];
        // Create container.
        let container = AnsiStyleContainer::from_vec(style);
        // Apply container's compiled style string to text.
        let text = container.apply("world!");
        // Print text.
        println!("Hello, {}", text);
    }

    #[test]
    #[cfg(windows)]
    fn test_windows_enable_ansi_support() {
        let result = enable_ansi_support();
        assert_eq!(result, Ok(()));
    }
}
