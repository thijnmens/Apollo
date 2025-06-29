pub enum Levels {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
    NONE,
}

impl Levels {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::DEBUG => 0,
            Self::INFO => 1,
            Self::WARN => 2,
            Self::ERROR => 3,
            Self::CRITICAL => 4,
            Self::NONE => u8::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_correct_order() {
        assert!(Levels::NONE.as_u8() > Levels::CRITICAL.as_u8());
        assert!(Levels::CRITICAL.as_u8() > Levels::ERROR.as_u8());
        assert!(Levels::ERROR.as_u8() > Levels::WARN.as_u8());
        assert!(Levels::WARN.as_u8() > Levels::INFO.as_u8());
        assert!(Levels::INFO.as_u8() > Levels::DEBUG.as_u8());
    }
}
