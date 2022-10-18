
#[derive(Clone)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

// ANSI codes generously provided from https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
// And https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
impl AnsiColor {
    pub const fn code(&self) -> &'static str {
        match self {
            AnsiColor::Black => "30",
            AnsiColor::Red => "31",
            AnsiColor::Green => "32",
            AnsiColor::Yellow => "33",
            AnsiColor::Blue => "34",
            AnsiColor::Magenta => "35",
            AnsiColor::Cyan => "36",
            AnsiColor::White => "37",
        }
    }
}
