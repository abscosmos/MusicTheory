use std::{array, slice};
use std::cmp::Ordering;
use std::ops::Index;
use const_soft_float::soft_f32::SoftF32;
use serde::{Deserialize, Serialize};
use typed_floats::tf32::{self, StrictlyPositiveFinite};
use crate::note::Note;
use crate::pitch_class::PitchClass;
use crate::tuning::{self, Cents, DeviationBetweenError, Tuning, TwelveToneEqualTemperament};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RatioBasedTuning {
    pub reference: Note,
    pub freq_hz: StrictlyPositiveFinite,
    pub ratios: OctaveRatios,
    pub ratios_base: PitchClass,
}

impl RatioBasedTuning {
    pub const DEFAULT_JUST_INTONATION: Self = Self::a4_440hz(OctaveRatios::JUST_INTONATION_LIMIT_5, PitchClass::C);

    pub const fn new(reference: Note, freq_hz: f32, ratios: OctaveRatios, ratios_base: PitchClass) -> Option<Self> {
        match StrictlyPositiveFinite::new(freq_hz) {
            Ok(freq_hz) => Some(Self { reference, freq_hz, ratios, ratios_base }),
            Err(_) => None,
        }
    }

    pub const fn a4_440hz(ratios: OctaveRatios, ratios_base: PitchClass) -> Self {
        match Self::new(Note::A4, 440.0, ratios, ratios_base) {
            Some(tuning) => tuning,
            None => panic!("unreachable!: 440 in range (0, inf)"),
        }
    }

    pub const fn from_twelve_tet(twelve_tet: TwelveToneEqualTemperament) -> Self {
        Self {
            reference: twelve_tet.reference,
            freq_hz: twelve_tet.freq_hz,
            ratios: OctaveRatios::TWELVE_TET,
            ratios_base: PitchClass::C, // doesn't matter, theoretically
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct OctaveRatios([StrictlyPositiveFinite; 12]);

// TODO: handle different ratios between A4 & d5
#[expect(clippy::excessive_precision, reason = "ensures values are as accurate as possible")]
impl OctaveRatios {
    // TODO: constants are defined like this since neither Result::ok nor Result::expect are const yet

    // this interprets an interval of 6 semitones (tritone) as a d5
    pub const JUST_INTONATION_LIMIT_5: Self = {
        let Ok(ratios) = Self::with_ratios(
            16.0/15.0,
            9.0/8.0,
            6.0/5.0,
            5.0/4.0,
            4.0/3.0,
            64.0/45.0,
            3.0/2.0,
            8.0/5.0,
            5.0/3.0,
            9.0/5.0,
            15.0/8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // this interprets an interval of 6 semitones (tritone) as a d5
    pub const JUST_INTONATION_LIMIT_7: Self = {
        let Ok(ratios) = Self::with_ratios(
            15.0 / 14.0,
            8.0 / 7.0,
            6.0 / 5.0,
            5.0 / 4.0,
            4.0 / 3.0,
            10.0 / 7.0,
            3.0 / 2.0,
            8.0 / 5.0,
            5.0 / 3.0,
            7.0 / 4.0,
            15.0 / 8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // this interprets an interval of 6 semitones (tritone) as a d5
    pub const PYTHAGOREAN: Self = {
        let Ok(ratios) = Self::with_ratios(
            256.0/243.0,
            9.0/8.0,
            32.0/27.0,
            81.0/64.0,
            4.0/3.0,
            1024.0/729.0,
            3.0/2.0,
            128.0/81.0,
            27.0/16.0,
            16.0/9.0,
            243.0/128.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // this interprets an interval of 6 semitones (tritone) as a d5
    // TODO: this can be calculate with a formula: https://en.wikipedia.org/wiki/Quarter-comma_meantone#12-tone_scale
    pub const QUARTER_COMMA_MEANTONE: Self = {
        let Ok(ratios) = Self::with_ratios(
            1.06998448796,
            1.11803398875,
            1.19627902498,
            1.25,
            1.33748060995,
            1.4310835056,
            1.49534878122,
            1.6,
            1.67185076244,
            1.788854382,
            1.86918597653,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    pub const WERCKMEISTER_I: Self = {
        const SQRT_2: f32 = std::f32::consts::SQRT_2;
        const TWO_4TH_ROOT: f32 = 1.18920711500272106671750;

        let Ok(ratios) = Self::with_ratios(
            256.0 / 243.0,
            64.0 / 81.0 * SQRT_2,
            32.0 / 27.0,
            256.0 / 243.0 * TWO_4TH_ROOT,
            4.0 / 3.0,
            1024.0 / 729.0,
            8.0 / 9.0 * TWO_4TH_ROOT * TWO_4TH_ROOT * TWO_4TH_ROOT,
            128.0 / 81.0,
            1024.0 / 729.0 * TWO_4TH_ROOT,
            16.0 / 9.0,
            128.0 / 81.0 * TWO_4TH_ROOT,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    pub const WERCKMEISTER_II: Self = {
        const CUBE_ROOT_2: f32 = 1.25992104989487316476721;
        const CUBE_ROOT_4: f32 = 1.58740105196819947475171;

        let Ok(ratios) = Self::with_ratios(
            16384.0 / 19683.0 * CUBE_ROOT_2,
            8.0 / 9.0 * CUBE_ROOT_2,
            32.0 / 27.0,
            64.0 / 81.0 * CUBE_ROOT_4,
            4.0 / 3.0,
            1024.0 / 729.0,
            32.0 / 27.0 * CUBE_ROOT_2,
            8192.0 / 6561.0 * CUBE_ROOT_2,
            256.0 / 243.0 * CUBE_ROOT_4,
            9.0 / (4.0 * CUBE_ROOT_2),
            4096.0 / 2187.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    pub const WERCKMEISTER_III: Self = {
        const TWO_4TH_ROOT: f32 = 1.18920711500272106671750;
        const SQRT_2: f32 = std::f32::consts::SQRT_2;
        const EIGHT_4TH_ROOT: f32 = 1.68179283050742908606225;

        let Ok(ratios) = Self::with_ratios(
            8.0 / 9.0 * TWO_4TH_ROOT,
            9.0 / 8.0,
            1.0 * TWO_4TH_ROOT,
            8.0 / 9.0 * SQRT_2,
            9.0 / 8.0 * TWO_4TH_ROOT,
            1.0 * SQRT_2,
            3.0 / 2.0,
            128.0 / 81.0,
            1.0 * EIGHT_4TH_ROOT,
            3.0 / EIGHT_4TH_ROOT,
            4.0 / 3.0 * SQRT_2,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    /// This has the 2 semitone interval at ratio of 28/25, not 49/44 as originally written,
    /// as the value written by Werckmeister [might be incorrect][wikipedia].
    ///
    /// [wikipedia]: https://en.wikipedia.org/wiki/Werckmeister_temperament#Werckmeister_IV_(VI):_the_Septenarius_tunings
    pub const WERCKMEISTER_IV: Self = {
        let Ok(ratios) = Self::with_ratios(
            98.0 / 93.0,
            28.0 / 25.0,
            196.0 / 165.0,
            49.0 / 39.0,
            4.0 / 3.0,
            196.0 / 139.0,
            196.0 / 131.0,
            49.0 / 31.0,
            196.0 / 117.0,
            98.0 / 55.0,
            49.0 / 26.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    pub const KIRNBERGER_I: Self = {
        let Ok(ratios) = Self::with_ratios(
            256.0 / 243.0,
            9.0 / 8.0,
            32.0 / 27.0,
            5.0 / 4.0,
            4.0 / 3.0,
            45.0 / 32.0,
            3.0 / 2.0,
            128.0 / 81.0,
            5.0 / 3.0,
            16.0 / 9.0,
            15.0 / 8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // fun fact: the wikipedia entry for this has a typo for the exact frequency ratio for A!
    // It says 3 * sqrt(5) / 2 instead of 3 * sqrt(5) / 4
    //
    // TODO: suggest an edit to wikipedia?
    // https://en.wikipedia.org/wiki/Kirnberger_temperament#Practical_temperaments:_Kirnberger_II
    pub const KIRNBERGER_II: Self = {
        const SQRT_5: f32 = 2.23606797749978969640917;

        let Ok(ratios) = Self::with_ratios(
            256.0 / 243.0,
            9.0 / 8.0,
            32.0 / 27.0,
            5.0 / 4.0,
            4.0 / 3.0,
            45.0 / 32.0,
            3.0 / 2.0,
            128.0 / 81.0,
            3.0 / 4.0 * SQRT_5,
            16.0 / 9.0,
            15.0 / 8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    pub const KIRNBERGER_III: Self = {
        const FIVE_4TH_ROOT: f32 = 1.49534878122122054191190;

        let Ok(ratios) = Self::with_ratios(
            256.0 / 243.0,
            FIVE_4TH_ROOT * FIVE_4TH_ROOT / 2.0,
            32.0 / 27.0,
            5.0 / 4.0,
            4.0 / 3.0,
            45.0 / 32.0,
            1.0 * FIVE_4TH_ROOT,
            128.0 / 81.0,
            FIVE_4TH_ROOT * FIVE_4TH_ROOT * FIVE_4TH_ROOT / 2.0,
            16.0 / 9.0,
            15.0 / 8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // TODO: implement Vallotti & Young temperaments

    pub const TWELVE_TET: Self = {
        // 2^(1/12), can't calculate const so hardcoded
        const SPACING: SoftF32 = SoftF32(1.0594630943);

        let Ok(one) = StrictlyPositiveFinite::new(1.0) else {
            panic!("unreachable!: 1.0 in (0, inf)");
        };

        let mut ratios = [one; 12];

        let mut i = 1;

        while i < ratios.len() {
            let soft_next = SoftF32(ratios[i - 1].get()).mul(SPACING);

            ratios[i] = match StrictlyPositiveFinite::new(soft_next.0) {
                Ok(next) => next,
                _ => panic!("unreachable!: must be within (0, inf)")
            };

            i += 1;
        }

        let octave = SoftF32(ratios[11].get()).mul(SPACING);

        assert!(
            matches!(octave.sub(SoftF32(2.0)).cmp(SoftF32(1e-5)), Some(Ordering::Less)),
            "should equal two at the end"
        );

        let Ok(ratios) = Self::new(ratios) else {
            panic!("ratios should be monotonically increasing, and in [1, 2)");
        };

        ratios
    };

    // this calls Self::with_ratios internally for a single source of truth
    pub const fn new(ratios: [StrictlyPositiveFinite; 12]) -> Result<Self, JustIntonationRatiosError> {
        let [
            unison,
            minor_second,
            major_second,
            minor_third,
            major_third,
            perfect_fourth,
            tritone,
            perfect_fifth,
            minor_sixth,
            major_sixth,
            minor_seventh,
            major_seventh,
        ] = ratios;

        // if unison != 1.0 (complicated because of const)
        if !matches!(SoftF32(unison.get()).cmp(SoftF32(1.0)), Some(Ordering::Equal)) {
            return Err(JustIntonationRatiosError::UnisonNotIdentity);
        }

        Self::with_ratios(
            minor_second.get(),
            major_second.get(),
            minor_third.get(),
            major_third.get(),
            perfect_fourth.get(),
            tritone.get(),
            perfect_fifth.get(),
            minor_sixth.get(),
            major_sixth.get(),
            minor_seventh.get(),
            major_seventh.get(),
        )
    }

    #[expect(clippy::too_many_arguments, reason = "one argument for each semitone ivl in octave")]
    pub const fn with_ratios(
        minor_second: f32,
        major_second: f32,
        minor_third: f32,
        major_third: f32,
        perfect_fourth: f32,
        tritone: f32,
        perfect_fifth: f32,
        minor_sixth: f32,
        major_sixth: f32,
        minor_seventh: f32,
        major_seventh: f32,
    ) -> Result<Self, JustIntonationRatiosError> {
        let ratios = [
            1.0,
            minor_second,
            major_second,
            minor_third,
            major_third,
            perfect_fourth,
            tritone,
            perfect_fifth,
            minor_sixth,
            major_sixth,
            minor_seventh,
            major_seventh,
        ];

        let mut res = [tf32::MAX; 12];

        // check strictly ascending
        let mut i = 0;
        while i < ratios.len() {
            if i + 1 != ratios.len() &&
                matches!(
                    SoftF32(ratios[i]).cmp(SoftF32(ratios[i + 1])),
                    Some(Ordering::Greater | Ordering::Equal),
                )
            {
                return Err(JustIntonationRatiosError::NotStrictlyIncreasing);
            }

            res[i] = match StrictlyPositiveFinite::new(ratios[i]) {
                Ok(checked) => checked,
                _ => return Err(JustIntonationRatiosError::InvalidRatio),
            };

            i += 1;
        }

        // instead of checking if all are in [1.0, 2.0), since already checked strictly increasing
        // and first is 1.0, only need to check last!
        match SoftF32(*ratios.last().expect("should have 12 elems")).cmp(SoftF32(2.0)) {
            Some(Ordering::Less | Ordering::Equal) => Ok(Self(res)),
            Some(Ordering::Greater) => Err(JustIntonationRatiosError::InvalidRatio),
            None => panic!("unreachable!: uncomparable values already handled"),
        }
    }
    
    pub fn as_array(self) -> [StrictlyPositiveFinite; 12] {
        self.0
    }

    pub fn deviation_from(self, other: Self, reference: Note, ref_freq_hz: StrictlyPositiveFinite) -> Result<[Cents; 12], DeviationBetweenError> {
        // TODO: is there a way to calculate this without having to actually compute frequencies? can it be computed just from ratios?

        tuning::deviation_between(
            &RatioBasedTuning {
                reference,
                freq_hz: ref_freq_hz,
                ratios: other,
                ratios_base: reference.as_pitch_class(),
            },
            &RatioBasedTuning {
                reference,
                freq_hz: ref_freq_hz,
                ratios: self,
                ratios_base: reference.as_pitch_class(),
            },
            reference.as_pitch_class(),
        )
    }

    pub fn deviation_from_twelve_tet(self) -> [Cents; 12] {
        let reference = Note::MIDDLE_C;

        let freq_hz = reference.as_frequency_hz()
            .expect("middle c should be in range for hz conversion");


        let res = tuning::deviation_between(
            &TwelveToneEqualTemperament {
                reference,
                freq_hz
            },
            &RatioBasedTuning {
                reference,
                freq_hz,
                ratios: self,
                ratios_base: reference.as_pitch_class(),
            },
            reference.as_pitch_class(),
        );

        res.expect("since ratios are bound (1, 2), and since using 12TET tuning of C4, should always be valid")
    }

    pub fn iter(&self) -> slice::Iter<'_, StrictlyPositiveFinite> {
        self.0.iter()
    }
}

impl Tuning for RatioBasedTuning {
    fn freq_to_note(&self, hz: StrictlyPositiveFinite) -> Option<(Note, Cents)> {
        let ref_offset = self.ratios_base.semitones_to(self.reference.as_pitch_class()).0 as usize;
        let ref_to_base = self.ratios[ref_offset];

        // if the reference pitch class is before the pitch class of the base
        // add one, since the "ratio octave" is different
        // (see comment in note_to_freq_hz)
        let ref_adj = (self.reference.as_pitch_class() < self.ratios_base) as i16;
        let ref_ratio_octave = self.reference.octave - ref_adj;

        let base0_freq = self.freq_hz.get() * ref_to_base.recip().get() * 2.0_f32.powf(-ref_ratio_octave as _);

        let ratio_from_base0 = hz.get() / base0_freq;
        let ratio_octave = ratio_from_base0.log2().floor() as i16;
        let ratio_within_octave = StrictlyPositiveFinite::new(ratio_from_base0 / 2.0_f32.powf(ratio_octave as _))
            .expect("ratio shouldn't be negative, nan, or infinity (unless octave is very very large)");

        let best_pc = (0..12)
            .map(|c| PitchClass::from_repr(c).expect("in range"))
            .min_by_key(|&pc| {
                let offset = self.ratios_base.semitones_to(pc).0 as usize;
                (self.ratios[offset] - ratio_within_octave).abs()
            })?;

        // +1 if the note's pitch class is less than the base,
        // as it would be in a different "ratio octave" then
        let note_adj = (best_pc < self.ratios_base) as i16;

        let best_note = Note {
            pitch: best_pc.into(),
            octave: ratio_octave + note_adj,
        };

        let best_offset = self.ratios_base.semitones_to(best_pc).0 as usize;
        let cents = Cents::between_frequencies(self.ratios[best_offset], ratio_within_octave)?;

        debug_assert!(
            (cents.get() - Cents::from_note(best_note, hz, self).expect("should be in range").get()).abs() < 1e-3,
            "using difference within an octave should be valid",
        );

        Some((best_note, cents))
    }

    fn note_to_freq_hz(&self, note: Note) -> Option<StrictlyPositiveFinite> {
        let pitch_offset = self.ratios_base.semitones_to(note.pitch.as_pitch_class()).0 as usize;
        let pitch_ratio = self.ratios[pitch_offset];

        let ref_offset = self.ratios_base.semitones_to(self.reference.pitch.as_pitch_class()).0 as usize;
        let ref_to_base = self.ratios[ref_offset];

        // octaves are C->B, but "ratio octaves" are base->(base-1).
        // for example, if base=B:  A4 and B4 are in different ratio octaves
        // adjust by -1 for notes whose pitch class is "before" the base.
        let ref_ratio_adj = (self.reference.pitch.as_pitch_class() < self.ratios_base) as i16;
        let note_ratio_adj = (note.pitch.as_pitch_class() < self.ratios_base) as i16;

        let octave_diff = (note.octave - self.reference.octave) + (ref_ratio_adj - note_ratio_adj);

        // reference_freq * (base / reference_pitch) * 2^(octave_diff) * pitch_ratio
        let hz = self.freq_hz.get()
            * ref_to_base.recip().get()
            * 2.0_f32.powf(octave_diff as _)
            * pitch_ratio.get();

        StrictlyPositiveFinite::new(hz).ok()
    }
}

impl Default for RatioBasedTuning {
    fn default() -> Self {
        Self::DEFAULT_JUST_INTONATION
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum JustIntonationRatiosError {
    #[error("Ratio between unisons must be 1/1")]
    UnisonNotIdentity,
    #[error("Ratios must be strictly increasing order")]
    NotStrictlyIncreasing,
    #[error("The ratios were not in range [1.0, 2.0)")]
    InvalidRatio,
}

impl Index<usize> for OctaveRatios {
    type Output = StrictlyPositiveFinite;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IntoIterator for OctaveRatios {
    type Item = StrictlyPositiveFinite;
    type IntoIter = array::IntoIter<StrictlyPositiveFinite, 12>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pitch::Pitch;

    /// This test does not check that the returned frequencies are "correct",
    /// as I cannot find a reliable source of note to freq conversions for
    /// any sort of just intonation tuning.
    ///
    /// This simply tests that, between changes in implementation, the results are still the same.
    #[test]
    fn consistent_freq_to_hz() {
        let cases = [
            (Note::new(Pitch::A_SHARP, 1), 59.399998),
            (Note::new(Pitch::B, 2), 123.75),
            (Note::new(Pitch::C, 4), 264.0),
            (Note::new(Pitch::F_SHARP, 5), 750.93335),
            (Note::new(Pitch::E, 12), 84480.0),
            (Note::new(Pitch::C_SHARP, 18), 4613734.5),
        ];

        let tuning = RatioBasedTuning::DEFAULT_JUST_INTONATION;

        for (note, hz) in cases {
            let hz = StrictlyPositiveFinite::new(hz).expect("hz in range");

            assert_eq!(tuning.note_to_freq_hz(note), Some(hz));
        }
    }

    #[test]
    fn twelve_tet_ratios() {
        let tuning_eq_temp = TwelveToneEqualTemperament::A4_440;

        let tuning_ratios = {
            let mut ratio_based = RatioBasedTuning::from_twelve_tet(tuning_eq_temp);
            ratio_based.ratios_base = PitchClass::B;

            ratio_based
        };

        for note in (u8::MIN..=u8::MAX).map(Note::from_midi) {
            let hz_eq_temp = tuning_eq_temp.note_to_freq_hz(note).expect("should return some for all MIDI notes");
            let hz_ratios = tuning_ratios.note_to_freq_hz(note).expect("should return some for all MIDI notes");

            let abs_diff = (hz_eq_temp - hz_ratios).abs();

            assert!(
                (abs_diff / hz_eq_temp).get() < 1e-6,
                "calculating freq using precomputed ratios should give same answer; failed: {note}, eq_temp: {hz_eq_temp}, ratios: {hz_ratios}, diff: {abs_diff}",
            );
        }
    }
}