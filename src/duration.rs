#[derive(Debug, Copy, Clone)]
pub struct WrittenDuration {
    log_len: i8,
    dots: u8,
}

impl WrittenDuration {
    pub const OCTUPLE_WHOLE: Self = Self::from_log_len(3);
    pub const QUADRUPLE_WHOLE: Self = Self::from_log_len(2);
    pub const DOUBLE_WHOLE: Self = Self::from_log_len(1);
    pub const WHOLE: Self = Self::from_log_len(0);
    pub const HALF: Self = Self::from_log_len(-1);
    pub const QUARTER: Self = Self::from_log_len(-2);
    pub const EIGHTH: Self = Self::from_log_len(-3);
    pub const SIXTEENTH: Self = Self::from_log_len(-4);
    pub const THIRTY_SECOND: Self = Self::from_log_len(-5);
    pub const SIXTY_FOURTH: Self = Self::from_log_len(-6);
    pub const HUNDRED_TWENTY_EIGHTH: Self = Self::from_log_len(-7);
    pub const TWO_HUNDRED_FIFTY_SIXTH: Self = Self::from_log_len(-8);
    
    pub const fn from_log_len(log_len: i8) -> Self {
        Self { log_len, dots: 0 }
    }
}