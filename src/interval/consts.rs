use std::num::NonZeroU16;
use crate::interval::{Interval, IntervalNumber, IntervalQuality};

use IntervalNumber as IN;
use IntervalQuality as IQ;

macro_rules! define_consts {
    ($($quality:ident $num:ident),* $(,)?) => {
        $(
            // TODO: std::concat_idents is nightly only
            paste::paste! { pub const [<$quality _ $num>]: Self = unsafe { Self::new_unchecked(IQ::$quality, IN::$num) }; }
        )*
        
        pub const ALL_CONSTS: &[Self] = &[ // TODO: use count metavar when stabilized
            $(
                paste::paste! { Self::[<$quality _ $num>] }
            ),*
        ];
    };
}

impl Interval {
    // this function is only to be used for defining constants
    const unsafe fn new_unchecked(quality: IntervalQuality, number: IntervalNumber) -> Self {
        Self { quality, number }
    }
    
    define_consts! {
        PERFECT UNISON,
        DIMINISHED SECOND,
        MINOR SECOND,
        AUGMENTED UNISON,
        MAJOR SECOND,
        DIMINISHED THIRD,
        MINOR THIRD,
        AUGMENTED SECOND,
        MAJOR THIRD,
        DIMINISHED FOURTH,
        PERFECT FOURTH,
        AUGMENTED THIRD,
        DIMINISHED FIFTH,
        AUGMENTED FOURTH,
        PERFECT FIFTH,
        DIMINISHED SIXTH,
        MINOR SIXTH,
        AUGMENTED FIFTH,
        MAJOR SIXTH,
        DIMINISHED SEVENTH,
        MINOR SEVENTH,
        AUGMENTED SIXTH,
        MAJOR SEVENTH,
        DIMINISHED OCTAVE,
        PERFECT OCTAVE,
        AUGMENTED SEVENTH,
        
        DIMINISHED NINTH,
        MINOR NINTH,
        AUGMENTED OCTAVE,
        MAJOR NINTH,
        DIMINISHED TENTH,
        MINOR TENTH,
        AUGMENTED NINTH,
        MAJOR TENTH,
        DIMINISHED ELEVENTH,
        PERFECT ELEVENTH,
        AUGMENTED TENTH,
        DIMINISHED TWELFTH,
        AUGMENTED ELEVENTH,
        PERFECT TWELFTH,
        DIMINISHED THIRTEENTH,
        MINOR THIRTEENTH,
        AUGMENTED TWELFTH,
        MAJOR THIRTEENTH,
        DIMINISHED FOURTEENTH,
        MINOR FOURTEENTH,
        AUGMENTED THIRTEENTH,
        MAJOR FOURTEENTH,
        DIMINISHED FIFTEENTH,
        PERFECT FIFTEENTH,
        AUGMENTED FOURTEENTH,
    }
}

impl IntervalNumber {
    pub const UNISON: Self = Self::new(1).expect("nonzero");
    pub const SECOND: Self =  Self::new(2).expect("nonzero");
    pub const THIRD: Self =  Self::new(3).expect("nonzero");
    pub const FOURTH: Self =  Self::new(4).expect("nonzero");
    pub const FIFTH: Self =  Self::new(5).expect("nonzero");
    pub const SIXTH: Self =  Self::new(6).expect("nonzero");
    pub const SEVENTH: Self =  Self::new(7).expect("nonzero");
    pub const OCTAVE: Self =  Self::new(8).expect("nonzero");

    pub const NINTH: Self = Self::new(9).expect("nonzero");
    pub const TENTH: Self =  Self::new(10).expect("nonzero");
    pub const ELEVENTH: Self =  Self::new(11).expect("nonzero");
    pub const TWELFTH: Self =  Self::new(12).expect("nonzero");
    pub const THIRTEENTH: Self =  Self::new(13).expect("nonzero");
    pub const FOURTEENTH: Self =  Self::new(14).expect("nonzero");
    pub const FIFTEENTH: Self =  Self::new(15).expect("nonzero");
}

impl IntervalQuality {
    // these consts are only for use in macro
    const PERFECT: Self = Self::Perfect;
    const MAJOR: Self = Self::Major;
    const MINOR: Self = Self::Minor;
    
    pub const DIMINISHED: Self = Self::Diminished(NonZeroU16::new(1).expect("nonzero"));
    pub const AUGMENTED: Self = Self::Augmented(NonZeroU16::new(1).expect("nonzero"));
}