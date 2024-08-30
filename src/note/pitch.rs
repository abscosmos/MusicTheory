use std::fmt;
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::note::accidental::AccidentalSign;
use crate::note::letter::Letter;
use crate::note::pitch_class::PitchClass;
use crate::semitone::Semitone;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pitch(pub(super) i16);

impl Pitch {
    pub fn from_letter_and_accidental(letter: Letter, accidental_sign: AccidentalSign) -> Self {
        let col_offset = accidental_sign.as_offset().0;

        let row_offset = match letter {
            Letter::C => 0,
            Letter::D => 2,
            Letter::E => 4,
            Letter::F => -1,
            Letter::G => 1,
            Letter::A => 3,
            Letter::B => 5,
        };

        let pitch = row_offset + 7 * col_offset;

        Self::from_fifths_from_c(pitch)
    }

    pub fn as_fifths_from_c(&self) -> i16 {
        self.0
    }

    pub fn from_fifths_from_c(fifths: i16) -> Self {
        Self(fifths)
    }

    pub fn as_pitch_class(&self) -> PitchClass {
        let fifths_plus_one = self.as_fifths_from_c() + 1;

        let semitone_offset = fifths_plus_one.div_euclid(7);

        let semitones_from_c = match fifths_plus_one.rem_euclid(7) {
            n if n % 2 == 0 => n + 5,
            n => n - 1,
        } + semitone_offset;

        let semitones_from_c: u8 = semitones_from_c
            .rem_euclid(12)
            .try_into()
            .expect("i8::rem_euclid(12) must be [0, 12)");

        PitchClass::try_from(semitones_from_c)
            .expect("i8::rem_euclid(12) must be [0, 12)")
    }

    // TODO: is this right?
    pub fn semitones_between(&self, rhs: Self) -> Semitone {
        let lhs = self.as_pitch_class() as u8 as i8;
        let rhs = rhs.as_pitch_class() as u8 as i8;

        Semitone((rhs - lhs) as _)
    }

    // pub fn accidental_sign(&self) -> AccidentalSign {
    //     match (self.as_fifths_from_c() + 1).div_euclid(7) {
    //         -2 => AccidentalSign::DoubleFlat,
    //         -1 => AccidentalSign::Flat,
    //         0 => AccidentalSign::Natural,
    //         1 => AccidentalSign::Sharp,
    //         2 => AccidentalSign::DoubleSharp,
    //         _ => unreachable!("Pitch doesn't support more than 2 flats or sharps")
    //     }
    // }

    pub fn semitones_offset_from_c(&self) -> Semitone {
        let fifths_plus_one = self.as_fifths_from_c() + 1;

        let n = match fifths_plus_one.rem_euclid(7) {
            0 => 5, // F
            1 => 0, // C
            2 => 7, // G
            3 => 2, // D
            4 => 9, // A
            5 => 4, // E
            6 => 11, // B
            _ => unreachable!("i8::rem_euclid(7) must be [0, 7)")
        } + fifths_plus_one.div_euclid(7);

        Semitone(n as _)
    }

    // TODO: inverse of this method
    pub fn apply_interval_ascending(&self, interval: &Interval) -> Self {
        self.apply_interval(interval, true)
    }

    pub fn apply_interval_descending(&self, interval: &Interval) -> Self {
        self.apply_interval(interval, false)
    }

    pub fn apply_interval(&self, interval: &Interval, ascending: bool) -> Self {
        use IntervalSize as S;
        use IntervalQuality as Q;

        let start = match interval.size().as_simple() {
            S::Unison | S::Octave => 7,
            S::Second => 9,
            S::Third => 11,
            S::Fourth => 6,
            S::Fifth => 8,
            S::Sixth => 10,
            S::Seventh => 12,
            _ => unreachable!("a simple interval can't be bigger than a octave")
        };

        let quality_offset = match interval.quality() {
            Q::DoublyAugmented => -1,
            Q::Augmented => 0,
            Q::Perfect | Q::Major => 1,
            Q::Minor => 2,
            Q::Diminished => match interval.size().is_perfect() {
                true => 2,
                false => 3,
            }
            Q::DoublyDiminished => match interval.size().is_perfect() {
                true => 3,
                false => 4,
            }
        };

        let offset = start - 7 * quality_offset;

        let dir_offset = if ascending { offset } else { -offset };

        Self::from_fifths_from_c(self.as_fifths_from_c() + dir_offset)
    }

    pub fn set_natural(&self) -> Self {
        let fifths = (self.as_fifths_from_c() + 1).rem_euclid(7) - 1;

        Self::from_fifths_from_c(fifths)
    }

    pub fn to_note_in_key(&self, key: ()) {
        todo!()
    }
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: fix debug
        write!(f, "{}", self.temp_debug())
    }
}

impl EnharmonicEq for Pitch {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.as_pitch_class() == rhs.as_pitch_class()
    }
}

impl From<PitchClass> for Pitch {
    fn from(value: PitchClass) -> Self {
        match value {
            PitchClass::C => Pitch::C,
            PitchClass::Cs => Pitch::C_SHARP,
            PitchClass::D => Pitch::D,
            PitchClass::Ds => Pitch::D_SHARP,
            PitchClass::E => Pitch::E,
            PitchClass::F => Pitch::F,
            PitchClass::Fs => Pitch::F_SHARP,
            PitchClass::G => Pitch::G,
            PitchClass::Gs => Pitch::G_SHARP,
            PitchClass::A => Pitch::A,
            PitchClass::As => Pitch::A_SHARP,
            PitchClass::B => Pitch::B,
        }
    }
}
