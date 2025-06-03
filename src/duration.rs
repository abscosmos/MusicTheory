use std::ops::Add;
use num_rational::Ratio;

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
    
    pub const fn with_dots(self, dots: u8) -> Self {
        Self { dots, ..self }
    }

    pub fn duration(self) -> Duration {
        let pow = self.log_len.unsigned_abs();
        
        assert!(
            pow < u32::BITS as _,
            "this written duration can't be represented by Duration's current implementation"
        );
        
        let mut base = Ratio::from_integer(1 << pow);
        
        if self.log_len.is_negative() {
            base = base.recip();
        }
        
        if self.dots > 0 {
            assert!(
                self.dots < u32::BITS as _,
                "this dot number can't be represented by Duration's current implementation"
            );
            
            base *= Ratio::from_integer(2) - Ratio::new_raw(1, 1 << self.dots);
        }
        
        Duration(base)
    }
}

impl From<WrittenDuration> for Duration {
    fn from(written_dur: WrittenDuration) -> Self {
        written_dur.duration()
    }
}

// TODO: change name to avoid collision with std::time::Duration?
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Duration(Ratio<u32>);

impl Duration {
    // TODO: properly define this method
    pub const ZERO: Self = Self(Ratio::new_raw(0, 1));
    
    pub fn new(dur: Ratio<u32>) -> Self {
        Self(dur.reduced())
    }
    
    pub fn ratio(self) -> Ratio<u32> {
        self.0
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}