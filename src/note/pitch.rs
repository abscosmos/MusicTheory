use strum_macros::EnumIter;
use crate::enharmonic::EnharmonicEq;
use crate::note::accidental::AccidentalSign;
use crate::note::letter::Letter;
use crate::note::pitch_class::PitchClass;

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
}

impl EnharmonicEq for Pitch {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.as_pitch_class() == rhs.as_pitch_class()
    }
}