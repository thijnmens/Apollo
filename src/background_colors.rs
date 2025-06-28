use std::fmt;
use std::ops::Add;
use crate::font_mode::FontMode;
use crate::foreground_colors::ForegroundColors;

pub enum BackgroundColors {
    Black(&'static str),
    Red(&'static str),
    Green(&'static str),
    Yellow(&'static str),
    Blue(&'static str),
    Magenta(&'static str),
    Cyan(&'static str),
    White(&'static str),
    Default(&'static str),
    BrightBlack(&'static str),
    BrightRed(&'static str),
    BrightGreen(&'static str),
    BrightYellow(&'static str),
    BrightBlue(&'static str),
    BrightMagenta(&'static str),
    BrightCyan(&'static str),
    BrightWhite(&'static str),
}

impl BackgroundColors {
    pub fn black() -> Self {
        BackgroundColors::Black("\x1B[40m")
    }

    pub fn red() -> Self {
        BackgroundColors::Red("\x1B[41m")
    }

    pub fn green() -> Self {
        BackgroundColors::Green("\x1B[42m")
    }

    pub fn yellow() -> Self {
        BackgroundColors::Yellow("\x1B[43m")
    }

    pub fn blue() -> Self {
        BackgroundColors::Blue("\x1B[44m")
    }

    pub fn magenta() -> Self {
        BackgroundColors::Magenta("\x1B[45m")
    }

    pub fn cyan() -> Self {
        BackgroundColors::Cyan("\x1B[46m")
    }

    pub fn white() -> Self {
        BackgroundColors::White("\x1B[47m")
    }

    pub fn default() -> Self {
        BackgroundColors::Default("\x1B[49m")
    }

    pub fn bright_black() -> Self {
        BackgroundColors::BrightBlack("\x1B[100m")
    }

    pub fn bright_red() -> Self {
        BackgroundColors::BrightRed("\x1B[101m")
    }

    pub fn bright_green() -> Self {
        BackgroundColors::BrightGreen("\x1B[102m")
    }

    pub fn bright_yellow() -> Self {
        BackgroundColors::BrightYellow("\x1B[103m")
    }

    pub fn bright_blue() -> Self {
        BackgroundColors::BrightBlue("\x1B[104m")
    }

    pub fn bright_magenta() -> Self {
        BackgroundColors::BrightMagenta("\x1B[105m")
    }

    pub fn bright_cyan() -> Self {
        BackgroundColors::BrightCyan("\x1B[106m")
    }

    pub fn bright_white() -> Self {
        BackgroundColors::BrightWhite("\x1B[107m")
    }
}

impl fmt::Display for BackgroundColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackgroundColors::Black(s) => write!(f, "{}", s),
            BackgroundColors::Red(s) => write!(f, "{}", s),
            BackgroundColors::Green(s) => write!(f, "{}", s),
            BackgroundColors::Yellow(s) => write!(f, "{}", s),
            BackgroundColors::Blue(s) => write!(f, "{}", s),
            BackgroundColors::Magenta(s) => write!(f, "{}", s),
            BackgroundColors::Cyan(s) => write!(f, "{}", s),
            BackgroundColors::White(s) => write!(f, "{}", s),
            BackgroundColors::Default(s) => write!(f, "{}", s),
            BackgroundColors::BrightBlack(s) => write!(f, "{}", s),
            BackgroundColors::BrightRed(s) => write!(f, "{}", s),
            BackgroundColors::BrightGreen(s) => write!(f, "{}", s),
            BackgroundColors::BrightYellow(s) => write!(f, "{}", s),
            BackgroundColors::BrightBlue(s) => write!(f, "{}", s),
            BackgroundColors::BrightMagenta(s) => write!(f, "{}", s),
            BackgroundColors::BrightCyan(s) => write!(f, "{}", s),
            BackgroundColors::BrightWhite(s) => write!(f, "{}", s)
        }
    }
}

impl Add<ForegroundColors> for BackgroundColors {
    type Output = String;

    fn add(self, rhs: ForegroundColors) -> Self::Output {
        let background_color = self.to_string().replace("\x1B[", "").replace("m", "");
        let foreground_color = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", background_color, foreground_color)
    }
}

impl Add<FontMode> for BackgroundColors {
    type Output = String;

    fn add(self, rhs: FontMode) -> Self::Output {
        let background_color = self.to_string().replace("\x1B[", "").replace("m", "");
        let font_mode = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", background_color, font_mode)
    }
}

impl Add<BackgroundColors> for BackgroundColors {
    type Output = String;

    fn add(self, rhs: BackgroundColors) -> Self::Output {
        let background_color_lhs = self.to_string().replace("\x1B[", "").replace("m", "");
        let background_color_rhs = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", background_color_lhs, background_color_rhs)
    }
}

impl Add<String> for BackgroundColors {
    type Output = String;

    fn add(self, rhs: String) -> Self::Output {
        let background_color = self.to_string().replace("\x1B[", "").replace("m", "");
        let string = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", background_color, string)
    }
}

impl Add<BackgroundColors> for String {
    type Output = String;

    fn add(self, rhs: BackgroundColors) -> Self::Output {
        let string = self.to_string().replace("\x1B[", "").replace("m", "");
        let background_color = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", string, background_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black() {
        assert_eq!("\x1B[40m", BackgroundColors::black().to_string());
    }

    #[test]
    fn test_red() {
        assert_eq!("\x1B[41m", BackgroundColors::red().to_string());
    }

    #[test]
    fn test_green() {
        assert_eq!("\x1B[42m", BackgroundColors::green().to_string());
    }

    #[test]
    fn test_yellow() {
        assert_eq!("\x1B[43m", BackgroundColors::yellow().to_string());
    }

    #[test]
    fn test_blue() {
        assert_eq!("\x1B[44m", BackgroundColors::blue().to_string());
    }

    #[test]
    fn test_magenta() {
        assert_eq!("\x1B[45m", BackgroundColors::magenta().to_string());
    }

    #[test]
    fn test_cyan() {
        assert_eq!("\x1B[46m", BackgroundColors::cyan().to_string());
    }

    #[test]
    fn test_white() {
        assert_eq!("\x1B[47m", BackgroundColors::white().to_string());
    }

    #[test]
    fn test_default() {
        assert_eq!("\x1B[49m", BackgroundColors::default().to_string());
    }

    #[test]
    fn test_bright_black() {
        assert_eq!("\x1B[100m", BackgroundColors::bright_black().to_string());
    }

    #[test]
    fn test_bright_red() {
        assert_eq!("\x1B[101m", BackgroundColors::bright_red().to_string());
    }

    #[test]
    fn test_bright_green() {
        assert_eq!("\x1B[102m", BackgroundColors::bright_green().to_string());
    }

    #[test]
    fn test_bright_yellow() {
        assert_eq!("\x1B[103m", BackgroundColors::bright_yellow().to_string());
    }

    #[test]
    fn test_bright_blue() {
        assert_eq!("\x1B[104m", BackgroundColors::bright_blue().to_string());
    }

    #[test]
    fn test_bright_magenta() {
        assert_eq!("\x1B[105m", BackgroundColors::bright_magenta().to_string());
    }

    #[test]
    fn test_bright_cyan() {
        assert_eq!("\x1B[106m", BackgroundColors::bright_cyan().to_string());
    }

    #[test]
    fn test_bright_white() {
        assert_eq!("\x1B[107m", BackgroundColors::bright_white().to_string());
    }

    #[test]
    fn test_background_color_add_foreground_color() {
        let background_color = BackgroundColors::red();
        let foreground_color = ForegroundColors::bright_yellow();

        assert_eq!("\x1B[41;93m", background_color + foreground_color);
    }

    #[test]
    fn test_background_color_add_background_color() {
        let background_color_one = BackgroundColors::red();
        let background_color_two = BackgroundColors::bright_yellow();

        assert_eq!("\x1B[41;103m", background_color_one + background_color_two);
    }

    #[test]
    fn test_background_color_add_font_mode() {
        let background_color = BackgroundColors::red();
        let font_mode = FontMode::italic();

        assert_eq!("\x1B[41;3m", background_color + font_mode);
    }

    #[test]
    fn test_background_color_add_string() {
        let background_color = BackgroundColors::red();
        let string = FontMode::italic() + ForegroundColors::bright_blue();

        assert_eq!("\x1B[41;3;94m", background_color + string);
    }

    #[test]
    fn test_string_add_background_color() {
        let string = FontMode::italic() + ForegroundColors::bright_blue();
        let background_color = BackgroundColors::red();

        assert_eq!("\x1B[3;94;41m", string + background_color);
    }
}