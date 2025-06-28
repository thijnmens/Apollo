use std::fmt;
use std::ops::Add;
use crate::background_colors::BackgroundColors;
use crate::foreground_colors::ForegroundColors;

pub enum FontMode {
    Bold(&'static str),
    Dim(&'static str),
    Italic(&'static str),
    Underline(&'static str),
    Blinking(&'static str),
    Reverse(&'static str),
    Invisible(&'static str),
    Strikethrough(&'static str),
    ResetAll(&'static str),
    ResetBold(&'static str),
    ResetDim(&'static str),
    ResetItalic(&'static str),
    ResetUnderline(&'static str),
    ResetBlinking(&'static str),
    ResetReverse(&'static str),
    ResetInvisible(&'static str),
    ResetStrikethrough(&'static str),
}

impl FontMode {
    pub fn bold() -> Self {
        FontMode::Bold("\x1B[1m")
    }

    pub fn dim() -> Self {
        FontMode::Dim("\x1B[2m")
    }

    pub fn italic() -> Self {
        FontMode::Italic("\x1B[3m")
    }

    pub fn underline() -> Self {
        FontMode::Underline("\x1B[4m")
    }

    pub fn blinking() -> Self {
        FontMode::Blinking("\x1B[5m")
    }

    pub fn reverse() -> Self {
        FontMode::Reverse("\x1B[7m")
    }

    pub fn invisible() -> Self {
        FontMode::Invisible("\x1B[8m")
    }

    pub fn strikethrough() -> Self {
        FontMode::Strikethrough("\x1B[9m")
    }

    pub fn reset_all() -> Self {
        FontMode::ResetAll("\x1B[0m")
    }

    pub fn reset_bold() -> Self {
        FontMode::ResetBold("\x1B[22m")
    }

    pub fn reset_dim() -> Self {
        FontMode::ResetDim("\x1B[22m")
    }

    pub fn reset_italic() -> Self {
        FontMode::ResetItalic("\x1B[23m")
    }

    pub fn reset_underline() -> Self {
        FontMode::ResetUnderline("\x1B[24m")
    }

    pub fn reset_blinking() -> Self {
        FontMode::ResetBlinking("\x1B[25m")
    }

    pub fn reset_reverse() -> Self {
        FontMode::ResetReverse("\x1B[27m")
    }

    pub fn reset_invisible() -> Self {
        FontMode::ResetInvisible("\x1B[28m")
    }

    pub fn reset_strikethrough() -> Self {
        FontMode::ResetStrikethrough("\x1B[29m")
    }
}

impl fmt::Display for FontMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontMode::Bold(s) => write!(f, "{}", s),
            FontMode::Dim(s) => write!(f, "{}", s),
            FontMode::Italic(s) => write!(f, "{}", s),
            FontMode::Underline(s) => write!(f, "{}", s),
            FontMode::Blinking(s) => write!(f, "{}", s),
            FontMode::Reverse(s) => write!(f, "{}", s),
            FontMode::Invisible(s) => write!(f, "{}", s),
            FontMode::Strikethrough(s) => write!(f, "{}", s),
            FontMode::ResetAll(s) => write!(f, "{}", s),
            FontMode::ResetBold(s) => write!(f, "{}", s),
            FontMode::ResetDim(s) => write!(f, "{}", s),
            FontMode::ResetItalic(s) => write!(f, "{}", s),
            FontMode::ResetUnderline(s) => write!(f, "{}", s),
            FontMode::ResetBlinking(s) => write!(f, "{}", s),
            FontMode::ResetReverse(s) => write!(f, "{}", s),
            FontMode::ResetInvisible(s) => write!(f, "{}", s),
            FontMode::ResetStrikethrough(s) => write!(f, "{}", s)
        }
    }
}

impl Add<BackgroundColors> for FontMode {
    type Output = String;

    fn add(self, rhs: BackgroundColors) -> Self::Output {
        let font_mode = self.to_string().replace("\x1B[", "").replace("m", "");
        let background_color = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", font_mode, background_color)
    }
}

impl Add<ForegroundColors> for FontMode {
    type Output = String;

    fn add(self, rhs: ForegroundColors) -> Self::Output {
        let font_mode = self.to_string().replace("\x1B[", "").replace("m", "");
        let foreground_color = rhs.to_string().replace("\x1B[", "").replace("m", "");

        format!("\x1B[{};{}m", font_mode, foreground_color)
    }
}

impl Add<FontMode> for FontMode {
    type Output = String;

    fn add(self, rhs: FontMode) -> Self::Output {
        let font_mode_lhs = self.to_string().replace("\x1B[", "").replace("m", "");
        let font_mode_rhs = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", font_mode_lhs, font_mode_rhs)
    }
}

impl Add<String> for FontMode {
    type Output = String;

    fn add(self, rhs: String) -> Self::Output {
        let font_mode = self.to_string().replace("\x1B[", "").replace("m", "");
        let string = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", font_mode, string)
    }
}

impl Add<FontMode> for String {
    type Output = String;

    fn add(self, rhs: FontMode) -> Self::Output {
        let string = self.to_string().replace("\x1B[", "").replace("m", "");
        let font_mode = rhs.to_string().replace("\x1B[", "").replace("m", "");


        format!("\x1B[{};{}m", string, font_mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!("\x1B[1m", FontMode::bold().to_string());
    }
    
    #[test]
    fn test_dim() {
        assert_eq!("\x1B[2m", FontMode::dim().to_string());
    }
    
    #[test]
    fn test_italic() {
        assert_eq!("\x1B[3m", FontMode::italic().to_string());
    }
    
    #[test]
    fn test_underline() {
        assert_eq!("\x1B[4m", FontMode::underline().to_string());
    }
    
    #[test]
    fn test_blinking() {
        assert_eq!("\x1B[5m", FontMode::blinking().to_string());
    }
    
    #[test]
    fn test_reverse() {
        assert_eq!("\x1B[7m", FontMode::reverse().to_string());
    }
    
    #[test]
    fn test_invisible() {
        assert_eq!("\x1B[8m", FontMode::invisible().to_string());
    }
    
    #[test]
    fn test_strikethrough() {
        assert_eq!("\x1B[9m", FontMode::strikethrough().to_string());
    }
    
    #[test]
    fn test_reset_all() {
        assert_eq!("\x1B[0m", FontMode::reset_all().to_string());
    }
    
    #[test]
    fn test_reset_bold() {
        assert_eq!("\x1B[22m", FontMode::reset_bold().to_string());
    }
    
    #[test]
    fn test_reset_dim() {
        assert_eq!("\x1B[22m", FontMode::reset_dim().to_string());
    }
    
    #[test]
    fn test_reset_italic() {
        assert_eq!("\x1B[23m", FontMode::reset_italic().to_string());
    }
    
    #[test]
    fn test_reset_underline() {
        assert_eq!("\x1B[24m", FontMode::reset_underline().to_string());
    }
    
    #[test]
    fn test_reset_blinking() {
        assert_eq!("\x1B[25m", FontMode::reset_blinking().to_string());
    }
    
    #[test]
    fn test_reset_reverse() {
        assert_eq!("\x1B[27m", FontMode::reset_reverse().to_string());
    }
    
    #[test]
    fn test_reset_invisible() {
        assert_eq!("\x1B[28m", FontMode::reset_invisible().to_string());
    }
    
    #[test]
    fn test_reset_strikethrough() {
        assert_eq!("\x1B[29m", FontMode::reset_strikethrough().to_string());
    }
    
    #[test]
    fn test_font_mode_add_foreground_color() {
        let font_mode = FontMode::blinking();
        let foreground_color = ForegroundColors::blue();
        
        assert_eq!("\x1B[5;34m", font_mode + foreground_color);
    }

    #[test]
    fn test_font_mode_add_background_color() {
        let font_mode = FontMode::blinking();
        let background_color = BackgroundColors::blue();

        assert_eq!("\x1B[5;44m", font_mode + background_color);
    }
    
    #[test]
    fn test_font_mode_add_font_mode() {
        let font_mode_one = FontMode::blinking();
        let font_mode_two = FontMode::bold();
        
        assert_eq!("\x1B[5;1m", font_mode_one + font_mode_two);
    }

    #[test]
    fn test_font_mode_add_string() {
        let font_mode = FontMode::underline();
        let string = BackgroundColors::yellow() + ForegroundColors::bright_magenta();

        assert_eq!("\x1B[4;43;95m", font_mode + string);
    }

    #[test]
    fn test_string_add_font_mode() {
        let string = BackgroundColors::yellow() + ForegroundColors::bright_magenta();
        let font_mode = FontMode::underline();

        assert_eq!("\x1B[43;95;4m", string + font_mode);
    }
}