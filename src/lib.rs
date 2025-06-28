pub mod foreground_colors;
pub mod background_colors;
pub mod font_mode;
pub mod levels;

use chrono::Utc;
use crate::foreground_colors::ForegroundColors;
use crate::background_colors::BackgroundColors;
use crate::font_mode::FontMode;
use crate::levels::Levels;

pub struct Apollo {
    pub logging_level: Levels
}

impl Default for Apollo {
    fn default() -> Self {
        Self::new()
    }
}

impl Apollo {

    /// Creates a new Apollo instance
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apollo_logger::Apollo;
    ///
    /// let l = Apollo::new(); // The default logging level is Debug
    ///
    /// l.debug("This message will be printed");
    /// l.warn("This message will also be printed");
    /// ```
    /// If you require a different logging level, please use the following code instead
    /// ```
    /// use crate::apollo_logger::Apollo;
    /// use crate::apollo_logger::levels::Levels;
    ///
    /// let l = Apollo { logging_level: Levels::INFO };
    ///
    /// l.debug("This message will NOT printed");
    /// l.warn("This message will be printed");
    /// ```
    pub fn new() -> Apollo {
        Apollo { logging_level: Levels::DEBUG }
    }
    
    
    /// Gets the current time in Day/Months/Year Hour:Minute:Second.Millisecond format
    fn get_time_as_string(&self) -> String {
        Utc::now().format("%D %H:%M:%S%.3f").to_string()
    }

    /// Prints a message to the console with the DEBUG label
    ///
    /// # Arguments
    ///
    /// * `s`: String to print to the console
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apollo_logger::Apollo;
    ///
    /// let l = Apollo::new();
    ///
    /// l.warn("This is an debug message");
    /// ```
    pub fn debug(&self, s: &str) -> Option<String> {

        // Check if the logging level is high enough
        if self.logging_level.as_u8() > Levels::DEBUG.as_u8() {
            return None;
        }

        // Get current time
        let current_time: String = self.get_time_as_string();

        // Get colors to print
        let date: ForegroundColors = ForegroundColors::bright_green();
        let label: ForegroundColors = ForegroundColors::cyan();
        let text: String = ForegroundColors::bright_white() + FontMode::bold();

        // Print to console
        let message = format!("{date}[{current_time}]\x1B[0m {label}[ DEBUG ]\x1B[0m | {text}{s}\x1B[0m");
        println!("{message}");

        Some(message)
    }

    /// Prints a message to the console with the INFO label
    ///
    /// # Arguments
    ///
    /// * `s`: String to print to the console
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apollo_logger::Apollo;
    ///
    /// let l = Apollo::new();
    ///
    /// l.info("This is an info message");
    /// ```
    pub fn info(&self, s: &str) -> Option<String> {

        // Check if the logging level is high enough
        if self.logging_level.as_u8() > Levels::INFO.as_u8() {
            return None;
        }

        // Get current time is [day/month/year hour:minute:second] format
        let current_time: String = self.get_time_as_string();

        // Get colors to print
        let date: ForegroundColors = ForegroundColors::bright_green();
        let label: ForegroundColors = ForegroundColors::blue();
        let text: ForegroundColors = ForegroundColors::bright_white();

        // Print to console
        let message = format!("{date}[{current_time}]\x1B[0m {label}[ INFO  ]\x1B[0m | {text}{s}\x1B[0m");
        println!("{message}");

        Some(message)
    }

    /// Prints a message to the console with the WARN label
    ///
    /// # Arguments
    ///
    /// * `s`: String to print to the console
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apollo_logger::Apollo;
    ///
    /// let l = Apollo::new();
    ///
    /// l.warn("This is an warning message");
    /// ```
    pub fn warn(&self, s: &str) -> Option<String> {

        // Check if the logging level is high enough
        if self.logging_level.as_u8() > Levels::WARN.as_u8() {
            return None;
        }

        // Get current time is [day/month/year hour:minute:second] format
        let current_time: String = self.get_time_as_string();

        // Get colors to print
        let date: ForegroundColors = ForegroundColors::bright_green();
        let label: ForegroundColors = ForegroundColors::yellow();
        let text: ForegroundColors = ForegroundColors::yellow();

        // Print to console
        let message: String = format!("{date}[{current_time}]\x1B[0m {label}[ WARN  ]\x1B[0m | {text}{s}\x1B[0m");
        println!("{message}");

        Some(message)
    }

    /// Prints a message to the console with the ERROR label
    ///
    /// # Arguments
    ///
    /// * `s`: String to print to the console
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apollo_logger::Apollo;
    ///
    /// let l = Apollo::new();
    ///
    /// l.error("This is an error message");
    /// ```
    pub fn error(&self, s: &str) -> Option<String> {

        // Check if the logging level is high enough
        if self.logging_level.as_u8() > Levels::ERROR.as_u8() {
            return None;
        }

        // Get current time is [day/month/year hour:minute:second] format
        let current_time: String = self.get_time_as_string();

        // Get colors to print
        let date: ForegroundColors = ForegroundColors::bright_green();
        let label: ForegroundColors = ForegroundColors::red();
        let text: ForegroundColors = ForegroundColors::red();

        // Print to console
        let message: String = format!("{date}[{current_time}]\x1B[0m {label}[ ERROR ]\x1B[0m | {text}{s}\x1B[0m");
        eprintln!("{message}");

        Some(message)
    }

    /// Prints a message to the console with the CRITICAL label
    ///
    /// # Arguments
    ///
    /// * `s`: String to print to the console
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apollo_logger::Apollo;
    ///
    /// let l = Apollo::new();
    ///
    /// l.critical("This is an critical message");
    /// ```
    pub fn critical(&self, s: &str) -> Option<String> {

        // Check if the logging level is high enough
        if self.logging_level.as_u8() > Levels::CRITICAL.as_u8() {
            return None;
        }

        // Get current time is [day/month/year hour:minute:second] format
        let current_time: String = self.get_time_as_string();

        // Get colors to print
        let date: ForegroundColors = ForegroundColors::bright_green();
        let label: ForegroundColors = ForegroundColors::bright_red();
        let text: String = ForegroundColors::bright_white() + BackgroundColors::bright_red() + FontMode::bold();

        // Print to console
        let message: String = format!("{date}[{current_time}]\x1B[0m {label}[ CRIT  ]\x1B[0m | {text}{s}\x1B[0m");
        eprintln!("{message}");

        Some(message)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// Test if debug will log to console with default logger level
    #[test]
    fn test_debug() {
        let logger = Apollo::new();
        assert!(logger.debug("This is a test debug message").is_some());
    }

    /// Test if debug will return None when logging level is too high
    #[test]
    fn test_debug_under_level() {
        let logger = Apollo { logging_level: Levels::INFO };
        assert!(logger.debug("This is a test debug message").is_none());
    }

    /// Test if info will log to console with default logger level
    #[test]
    fn test_info() {
        let logger = Apollo::new();
        assert!(logger.info("This is a test info message").is_some());
    }

    /// Test if info will return None when logging level is too high
    #[test]
    fn test_info_under_level() {
        let logger = Apollo { logging_level: Levels::WARN };
        assert!(logger.info("This is a test info message").is_none());
    }

    /// Test if warn will log to console with default logger level
    #[test]
    fn test_warn() {
        let logger = Apollo::new();
        assert!(logger.warn("This is a test warning message").is_some());
    }

    /// Test if warn will return None when logging level is too high
    #[test]
    fn test_warn_under_level() {
        let logger = Apollo { logging_level: Levels::ERROR };
        assert!(logger.warn("This is a test warning message").is_none());
    }

    /// Test if error will log to console with default logger level
    #[test]
    fn test_error() {
        let logger = Apollo::new();
        assert!(logger.error("This is a test error message").is_some());
    }

    /// Test if error will return None when logging level is too high
    #[test]
    fn test_error_under_level() {
        let logger = Apollo { logging_level: Levels::CRITICAL };
        assert!(logger.error("This is a test error message").is_none());
    }

    /// Test if critical will log to console with default logger level
    #[test]
    fn test_critical() {
        let logger = Apollo::new();
        assert!(logger.critical("This is a test critical message").is_some());
    }

    /// Test if critical will return None when logging level is too high
    #[test]
    fn test_critical_under_level() {
        let logger = Apollo { logging_level: Levels::NONE };
        assert!(logger.critical("This is a test critical message").is_none());
    }

    /// Test if nothing gets logged when logging level is None
    #[test]
    fn test_logging_level_none() {
        let logger = Apollo { logging_level: Levels::NONE };
        assert!(logger.debug("This is a test debug message").is_none());
        assert!(logger.info("This is a test info message").is_none());
        assert!(logger.warn("This is a test warning message").is_none());
        assert!(logger.error("This is a test error message").is_none());
        assert!(logger.critical("This is a test critical message").is_none());
    }
    
    #[test]
    fn test_default_creates_new_instance() {
        let logger = Apollo::default();
        
        assert!(logger.debug("This is a test debug message").is_some());
    }
}