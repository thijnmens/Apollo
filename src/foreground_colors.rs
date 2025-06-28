use std::fmt;
use std::ops::Add;
use crate::background_colors::BackgroundColors;
use crate::font_mode::FontMode;

pub enum ForegroundColors {
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

impl ForegroundColors {
    pub fn black() -> Self {
        ForegroundColors::Black("\x1B[30m")
    }

    pub fn red() -> Self {
        ForegroundColors::Red("\x1B[31m")
    }

    pub fn green() -> Self {
        ForegroundColors::Green("\x1B[32m")
    }

    pub fn yellow() -> Self {
        ForegroundColors::Yellow("\x1B[33m")
    }

    pub fn blue() -> Self {
        ForegroundColors::Blue("\x1B[34m")
    }

    pub fn magenta() -> Self {
        ForegroundColors::Magenta("\x1B[35m")
    }

    pub fn cyan() -> Self {
        ForegroundColors::Cyan("\x1B[36m")
    }

    pub fn white() -> Self {
        ForegroundColors::White("\x1B[37m")
    }

    pub fn default() -> Self {
        ForegroundColors::Default("\x1B[39m")
    }

    pub fn bright_black() -> Self {
        ForegroundColors::BrightBlack("\x1B[90m")
    }

    pub fn bright_red() -> Self {
        ForegroundColors::BrightRed("\x1B[91m")
    }

    pub fn bright_green() -> Self {
        ForegroundColors::BrightGreen("\x1B[92m")
    }

    pub fn bright_yellow() -> Self {
        ForegroundColors::BrightYellow("\x1B[93m")
    }

    pub fn bright_blue() -> Self {
        ForegroundColors::BrightBlue("\x1B[94m")
    }

    pub fn bright_magenta() -> Self {
        ForegroundColors::BrightMagenta("\x1B[95m")
    }

    pub fn bright_cyan() -> Self {
        ForegroundColors::BrightCyan("\x1B[96m")
    }

    pub fn bright_white() -> Self {
        ForegroundColors::BrightWhite("\x1B[97m")
    }
}

impl fmt::Display for ForegroundColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForegroundColors::Black(s) => write!(f, "{}", s),
            ForegroundColors::Red(s) => write!(f, "{}", s),
            ForegroundColors::Green(s) => write!(f, "{}", s),
            ForegroundColors::Yellow(s) => write!(f, "{}", s),
            ForegroundColors::Blue(s) => write!(f, "{}", s),
            ForegroundColors::Magenta(s) => write!(f, "{}", s),
            ForegroundColors::Cyan(s) => write!(f, "{}", s),
            ForegroundColors::White(s) => write!(f, "{}", s),
            ForegroundColors::Default(s) => write!(f, "{}", s),
            ForegroundColors::BrightBlack(s) => write!(f, "{}", s),
            ForegroundColors::BrightRed(s) => write!(f, "{}", s),
            ForegroundColors::BrightGreen(s) => write!(f, "{}", s),
            ForegroundColors::BrightYellow(s) => write!(f, "{}", s),
            ForegroundColors::BrightBlue(s) => write!(f, "{}", s),
            ForegroundColors::BrightMagenta(s) => write!(f, "{}", s),
            ForegroundColors::BrightCyan(s) => write!(f, "{}", s),
            ForegroundColors::BrightWhite(s) => write!(f, "{}", s)
        }
    }
}

impl Add<BackgroundColors> for ForegroundColors {
    type Output = String;

    fn add(self, rhs: BackgroundColors) -> Self::Output {
        let foreground_color = self.to_string().replace("\x1B[", "").replace("m", "");
        let background_color = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", foreground_color, background_color)
    }
}

impl Add<FontMode> for ForegroundColors {
    type Output = String;

    fn add(self, rhs: FontMode) -> Self::Output {
        let foreground_color = self.to_string().replace("\x1B[", "").replace("m", "");
        let font_mode = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", foreground_color, font_mode)
    }
}

impl Add<ForegroundColors> for ForegroundColors {
    type Output = String;

    fn add(self, rhs: ForegroundColors) -> Self::Output {
        let foreground_color_lhs = self.to_string().replace("\x1B[", "").replace("m", "");
        let foreground_color_rhs = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", foreground_color_lhs, foreground_color_rhs)
    }
}

impl Add<String> for ForegroundColors {
    type Output = String;

    fn add(self, rhs: String) -> Self::Output {
        let foreground_color = self.to_string().replace("\x1B[", "").replace("m", "");
        let string = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", foreground_color, string)
    }
}

impl Add<ForegroundColors> for String {
    type Output = String;

    fn add(self, rhs: ForegroundColors) -> Self::Output {
        let string = self.to_string().replace("\x1B[", "").replace("m", "");
        let foreground_color = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", string, foreground_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black() {
        assert_eq!("\x1B[30m", ForegroundColors::black().to_string());
    }
    
    #[test]
    fn test_red() {
        assert_eq!("\x1B[31m", ForegroundColors::red().to_string());
    }
    
    #[test]
    fn test_green() {
        assert_eq!("\x1B[32m", ForegroundColors::green().to_string());
    }
    
    #[test]
    fn test_yellow() {
        assert_eq!("\x1B[33m", ForegroundColors::yellow().to_string());
    }
    
    #[test]
    fn test_blue() {
        assert_eq!("\x1B[34m", ForegroundColors::blue().to_string());
    }
    
    #[test]
    fn test_magenta() {
        assert_eq!("\x1B[35m", ForegroundColors::magenta().to_string());
    }
    
    #[test]
    fn test_cyan() {
        assert_eq!("\x1B[36m", ForegroundColors::cyan().to_string());
    }
    
    #[test]
    fn test_white() {
        assert_eq!("\x1B[37m", ForegroundColors::white().to_string());
    }
    
    #[test]
    fn test_default() {
        assert_eq!("\x1B[39m", ForegroundColors::default().to_string());
    }
    
    #[test]
    fn test_bright_black() {
        assert_eq!("\x1B[90m", ForegroundColors::bright_black().to_string());
    }
    
    #[test]
    fn test_bright_red() {
        assert_eq!("\x1B[91m", ForegroundColors::bright_red().to_string());
    }
    
    #[test]
    fn test_bright_green() {
        assert_eq!("\x1B[92m", ForegroundColors::bright_green().to_string());
    }
    
    #[test]
    fn test_bright_yellow() {
        assert_eq!("\x1B[93m", ForegroundColors::bright_yellow().to_string());
    }
    
    #[test]
    fn test_bright_blue() {
        assert_eq!("\x1B[94m", ForegroundColors::bright_blue().to_string());
    }
    
    #[test]
    fn test_bright_magenta() {
        assert_eq!("\x1B[95m", ForegroundColors::bright_magenta().to_string());
    }
    
    #[test]
    fn test_bright_cyan() {
        assert_eq!("\x1B[96m", ForegroundColors::bright_cyan().to_string());
    }
    
    #[test]
    fn test_bright_white() {
        assert_eq!("\x1B[97m", ForegroundColors::bright_white().to_string());
    }

    #[test]
    fn test_foreground_color_add_foreground_color() {
        let foreground_color_one = ForegroundColors::cyan();
        let foreground_color_two = ForegroundColors::magenta();

        assert_eq!("\x1B[36;35m", foreground_color_one + foreground_color_two);
    }

    #[test]
    fn test_foreground_color_add_background_color() {
        let foreground_color = ForegroundColors::cyan();
        let background_color = BackgroundColors::magenta();

        assert_eq!("\x1B[36;45m", foreground_color + background_color);
    }

    #[test]
    fn test_foreground_color_add_font_mode() {
        let foreground_color = ForegroundColors::cyan();
        let font_mode = FontMode::underline();

        assert_eq!("\x1B[36;4m", foreground_color + font_mode);
    }

    #[test]
    fn test_foreground_color_add_string() {
        let foreground_color = ForegroundColors::cyan();
        let string = BackgroundColors::black() + FontMode::strikethrough();

        assert_eq!("\x1B[36;40;9m", foreground_color + string);
    }

    #[test]
    fn test_string_add_foreground_color() {
        let string = BackgroundColors::black() + FontMode::strikethrough();
        let foreground_color = ForegroundColors::cyan();

        assert_eq!("\x1B[40;9;36m", string + foreground_color);
    }
}