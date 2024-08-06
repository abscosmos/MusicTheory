use strum_macros::EnumIter;
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::note::accidental::AccidentalSign;
use crate::note::letter::Letter;
use crate::note::pitch_class::PitchClass;
use crate::semitone::Semitone;

#[repr(i8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum Pitch {
    FDoubleFlat = -15,
    CDoubleFlat = -14,
    GDoubleFlat = -13,
    DDoubleFlat = -12,
    ADoubleFlat = -11,
    EDoubleFlat = -10,
    BDoubleFlat = -9,

    FFlat = -8,
    CFlat = -7,
    GFlat = -6,
    DFlat = -5,
    AFlat = -4,
    EFlat = -3,
    BFlat = -2,

    F = -1,
    C = 0,
    G = 1,
    D = 2,
    A = 3,
    E = 4,
    B = 5,

    FSharp = 6,
    CSharp = 7,
    GSharp = 8,
    DSharp = 9,
    ASharp = 10,
    ESharp = 11,
    BSharp = 12,

    FDoubleSharp = 13,
    CDoubleSharp = 14,
    GDoubleSharp = 15,
    DDoubleSharp = 16,
    ADoubleSharp = 17,
    EDoubleSharp = 18,
    BDoubleSharp = 19,
}

impl Pitch {
    pub fn from_letter_and_accidental(letter: Letter, accidental_sign: AccidentalSign) -> Self {
        let col_offset = accidental_sign.as_offset().0 as i8;

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
            .expect("should be within [-15,19]")
    }

    pub fn as_fifths_from_c(&self) -> i8 {
        *self as _
    }

    // TODO: how does wrapping work?
    pub fn from_fifths_from_c(fifths: i8) -> Option<Self> {
        match fifths {
            -15 => Some(Pitch::FDoubleFlat),
            -14 => Some(Pitch::CDoubleFlat),
            -13 => Some(Pitch::GDoubleFlat),
            -12 => Some(Pitch::DDoubleFlat),
            -11 => Some(Pitch::ADoubleFlat),
            -10 => Some(Pitch::EDoubleFlat),
            -9 => Some(Pitch::BDoubleFlat),
            -8 => Some(Pitch::FFlat),
            -7 => Some(Pitch::CFlat),
            -6 => Some(Pitch::GFlat),
            -5 => Some(Pitch::DFlat),
            -4 => Some(Pitch::AFlat),
            -3 => Some(Pitch::EFlat),
            -2 => Some(Pitch::BFlat),
            -1 => Some(Pitch::F),
            0 => Some(Pitch::C),
            1 => Some(Pitch::G),
            2 => Some(Pitch::D),
            3 => Some(Pitch::A),
            4 => Some(Pitch::E),
            5 => Some(Pitch::B),
            6 => Some(Pitch::FSharp),
            7 => Some(Pitch::CSharp),
            8 => Some(Pitch::GSharp),
            9 => Some(Pitch::DSharp),
            10 => Some(Pitch::ASharp),
            11 => Some(Pitch::ESharp),
            12 => Some(Pitch::BSharp),
            13 => Some(Pitch::FDoubleSharp),
            14 => Some(Pitch::CDoubleSharp),
            15 => Some(Pitch::GDoubleSharp),
            16 => Some(Pitch::DDoubleSharp),
            17 => Some(Pitch::ADoubleSharp),
            18 => Some(Pitch::EDoubleSharp),
            19 => Some(Pitch::BDoubleSharp),
            _ => None,
        }
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

    pub fn semitones_between(&self, rhs: Self) -> Semitone {
        let lhs = self.as_pitch_class() as u8 as i8;
        let rhs = rhs.as_pitch_class() as u8 as i8;

        Semitone((rhs - lhs) as _)
    }

    pub fn apply_interval(&self, interval: &Interval) -> Option<Self> {
        use IntervalSize as S;
        use IntervalQuality as Q;

        let offset = match (interval.quality(), interval.size().as_simple()) {
            // 0 semitones
            (Q::Perfect, S::Unison) => 0,
            (Q::Diminished, S::Second) => -12,

            // 1 semitone
            (Q::Minor, S::Second) => -5,
            (Q::Augmented, S::Unison) => 7,

            // 2 semitones
            (Q::Major, S::Second) => 2,
            (Q::Diminished, S::Third) => -10,

            // 3 semitones
            (Q::Minor, S::Third) => -3,
            (Q::Augmented, S::Second) => 9,

            // 4 semitones
            (Q::Major, S::Third) => 4,
            (Q::Diminished, S::Fourth) => -8,

            // 5 semitones
            (Q::Perfect, S::Fourth) => -1,
            (Q::Augmented, S::Third) => 11,

            // 6 semitones
            (Q::Augmented, S::Fourth) => 6,
            (Q::Diminished, S::Fifth) => -6,

            // 7 semitones
            (Q::Perfect, S::Fifth) => 1,
            (Q::Diminished, S::Sixth) => -11,

            // 8 semitones
            (Q::Minor, S::Sixth) => -4,
            (Q::Augmented, S::Fifth) => 8,

            // 9 semitones
            (Q::Major, S::Sixth) => 3,
            (Q::Diminished, S::Seventh) => -9,

            // 10 semitones
            (Q::Minor, S::Seventh) => -2,
            (Q::Augmented, S::Sixth) => 10,

            // 11 semitones
            (Q::Major, S::Seventh) => 5,
            (Q::Diminished, S::Octave) => -7,

            // 12 semitones
            (Q::Perfect, S::Octave) => 0,
            (Q::Augmented, S::Seventh) => 12,

            // 13 semitones
            (Q::Augmented, S::Octave) => 7,

            _ => return None,
        };

        let res = Self::from_fifths_from_c(self.as_fifths_from_c() + offset);

        if let Some(res) = &res {
            assert_eq!(
                self.semitones_between(*res).0.rem_euclid(12), interval.semitones().0 % 12,
                "pitch: {:?}, + interval: {} = res: {:?} \nRes semitone diff: {}, expected: {}",
                self, interval.shorthand(), res, self.semitones_between(*res).0, interval.semitones().0 % 12
            );
        }

        res
    }
}

impl EnharmonicEq for Pitch {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.as_pitch_class() == rhs.as_pitch_class()
    }
}