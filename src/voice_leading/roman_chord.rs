use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::interval::{Interval, IntervalQuality};
use crate::key::Key;
use crate::pcset::PitchClassSet;
use crate::pitch::Pitch;
use crate::scales::heptatonic::DiatonicMode;
use strum_macros::{EnumIter, EnumString, FromRepr};

// not typed at all!
pub mod inversions {
    pub const INV_ROOT: u8 = 0;
    pub const INV_6: u8 = 1;
    pub const INV_64: u8 = 2;
    pub const INV_65: u8 = 1;
    pub const INV_43: u8 = 2;
    pub const INV_42: u8 = 3;
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromRepr, EnumString, EnumIter, Serialize, Deserialize)]
pub enum ScaleDegree {
    I = 1,
    II = 2,
    III = 3,
    IV = 4,
    V = 5,
    VI = 6,
    VII = 7,
}

impl ScaleDegree {
    pub fn as_idx(self) -> u8 {
        (self as u8) - 1
    }

    pub fn from_idx(idx: u8) -> Option<Self> {
        Self::from_repr(idx + 1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
}


#[derive(Debug, thiserror::Error, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[error("Invalid inversion for chord type")]
pub struct InvalidInversionError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RomanChord {
    pub degree: ScaleDegree,
    pub triad_quality: Quality,
    seventh_quality: Option<Quality>,
    inversion: u8,
}

impl RomanChord {
    pub fn new(
        degree: ScaleDegree,
        triad_quality: Quality,
        seventh_quality: Option<Quality>,
        inversion: u8,
    ) -> Result<Self, InvalidInversionError> {
        let max_inversion = if seventh_quality.is_some() { 3 } else { 2 };

        if inversion > max_inversion {
            return Err(InvalidInversionError);
        }

        Ok(Self {
            degree,
            triad_quality,
            seventh_quality,
            inversion,
        })
    }

    pub fn triad(degree: ScaleDegree, triad_quality: Quality) -> Self {
        Self::new(degree, triad_quality, None, inversions::INV_ROOT)
            .expect("root position inversion always valid")
    }

    pub fn seventh(degree: ScaleDegree, triad_quality: Quality, seventh_quality: Quality) -> Self {
        Self::new(degree, triad_quality, Some(seventh_quality), inversions::INV_ROOT)
            .expect("root position inversion always valid")
    }

    pub fn seventh_quality(&self) -> Option<Quality> {
        self.seventh_quality
    }

    pub fn inversion(&self) -> u8 {
        self.inversion
    }

    pub fn with_inversion(self, inversion: u8) -> Result<Self, InvalidInversionError> {
        Self::new(
            self.degree,
            self.triad_quality,
            self.seventh_quality,
            inversion,
        )
    }

    pub fn has_seventh(&self) -> bool {
        self.seventh_quality.is_some()
    }

    pub fn len(&self) -> u8 {
        if self.has_seventh() {
            4
        } else {
            3
        }
    }

    // TODO: should this factor in inversions?
    pub fn intervals(&self) -> Vec<Interval> {
        use Interval as I;
        use Quality as Q;

        let mut intervals = vec![I::PERFECT_UNISON];

        let triad = match self.triad_quality {
            Q::Major => [I::MAJOR_THIRD, I::PERFECT_FIFTH],
            Q::Minor => [I::MINOR_THIRD, I::PERFECT_FIFTH],
            Q::Diminished => [I::MINOR_THIRD, I::DIMINISHED_FIFTH],
            Q::Augmented => [I::MAJOR_THIRD, I::AUGMENTED_FIFTH],
        };

        let seventh = self.seventh_quality.map(|q| match q {
            Quality::Major => I::MAJOR_SEVENTH,
            Quality::Minor => I::MINOR_SEVENTH,
            Quality::Diminished => I::DIMINISHED_SEVENTH,
            Quality::Augmented => I::AUGMENTED_SEVENTH,
        });

        intervals.extend(triad);
        intervals.extend(seventh);

        intervals
    }

    // source of truth for alterations
    // TODO: move this somewhere else?
    pub(crate) fn mode_has_raised_leading_tone(mode: DiatonicMode) -> bool {
        matches!(mode, DiatonicMode::Aeolian | DiatonicMode::Dorian)
    }

    pub fn root_in_key(&self, key: Key) -> Pitch {
        // TODO: this scale function is experimental
        let scale = key.scale().build_default();
        let mut root = scale[self.degree.as_idx() as usize];

        // TODO: maybe this function is overkill?
        if self.should_raise_leading_tone(key).unwrap_or(false) {
            root = root.transpose(Interval::AUGMENTED_UNISON);
        }

        root
    }

    fn should_raise_leading_tone(&self, key: Key) -> Option<bool> {
        if !matches!(self.degree, ScaleDegree::V | ScaleDegree::VII) || !Self::mode_has_raised_leading_tone(key.mode) {
            return Some(false);
        }

        fn intervals_match(mut scale: [Pitch; 7], intervals: &[Interval], degree: ScaleDegree) -> bool {
            assert!(
                (3..=4).contains(&intervals.len()),
                "triad or seventh must have either 3 or four intervals"
            );

            let degree = degree.as_idx() as usize;

            scale.rotate_left(degree);

            let bass = scale[0];
            let third = scale[2];
            let fifth = scale[4];
            let seventh = scale[6];

            let exp_third = bass.distance_to(third).as_simple();
            let exp_fifth = bass.distance_to(fifth).as_simple();
            let exp_seventh = bass.distance_to(seventh).as_simple();

            let [_, third, fifth, ..] = *intervals else {
                unreachable!("triad or seventh must have at least 3 intervals");
            };

            let seventh = intervals.get(3).copied();

            exp_third == third && exp_fifth == fifth && seventh.is_none_or(|s| s == exp_seventh)
        }

        let scale = key.scale().build_default();

        let scale_raised = {
            let mut s = scale;
            s[6] = s[6].transpose(Interval::AUGMENTED_UNISON);
            s
        };

        let intervals = self.intervals();

        if intervals_match(scale_raised, &intervals, self.degree) {
            Some(true)
        } else if intervals_match(scale, &intervals, self.degree) {
            Some(false)
        } else {
            None
        }
    }

    pub fn pitches(&self, key: Key) -> Vec<Pitch> {
        let root = self.root_in_key(key);

        let mut pitches = self.intervals().into_iter()
            .map(|i| root.transpose(i))
            .collect::<Vec<_>>();

        pitches.rotate_left(self.inversion as _);

        pitches
    }

    pub fn bass(&self, key: Key) -> Pitch {
        let root = self.root_in_key(key);

        let intervals = self.intervals();

        root.transpose(
            *intervals.get(self.inversion as usize).expect("should be valid inversion")
        )
    }

    pub fn pitch_class_set(&self, key: Key) -> PitchClassSet {
        self.pitches(key)
            .into_iter()
            .map(Pitch::as_pitch_class)
            .collect()
    }

    // TODO: should this be in mode instead of in key, since the pitch is irrelevant?
    pub fn diatonic_in_key(
        degree: ScaleDegree,
        key: Key,
        with_seventh: bool,
    ) -> Self {
        use Quality as Q;
        use Interval as I;
        use IntervalQuality as IQ;

        let mut scale = {
            let mut scale = key.scale().build_default();

            if matches!(degree, ScaleDegree::V | ScaleDegree::VII) && Self::mode_has_raised_leading_tone(key.mode) {
                scale[6] = scale[6].transpose(Interval::AUGMENTED_UNISON);
            }

            scale
        };

        let degree_idx = degree.as_idx() as usize;

        scale.rotate_left(degree_idx);

        let root = scale[0];
        let third = scale[2];
        let fifth = scale[4];

        let third_interval = root.distance_to(third).as_simple();
        let fifth_interval = root.distance_to(fifth).as_simple();

        let triad_quality = match (third_interval, fifth_interval) {
            (I::MAJOR_THIRD, I::PERFECT_FIFTH) => Q::Major,
            (I::MINOR_THIRD, I::PERFECT_FIFTH) => Q::Minor,
            (I::MINOR_THIRD, I::DIMINISHED_FIFTH) => Q::Diminished,
            (I::MAJOR_THIRD, I::AUGMENTED_FIFTH) => Q::Augmented,
            _ => unreachable!("diatonic triad intervals must be either maj, min, dim, or aug"),
        };

        let seventh_quality = with_seventh.then(|| {
            let seventh = scale[6];

            match root.distance_to(seventh).as_simple().quality() {
                IQ::Major => Q::Major,
                IQ::Minor => Q::Minor,
                IQ::DIMINISHED => Q::Diminished,
                IQ::AUGMENTED => Q::Augmented,
                IQ::Perfect => unreachable!("sevenths cannot be perfect"),
                IQ::Diminished(_) | IQ::Augmented(_) => unreachable!("sevenths cannot be multiply diminished or augmented")
            }
        });

        Self {
            degree,
            triad_quality,
            seventh_quality,
            inversion: 0,
        }
    }
}

impl fmt::Display for RomanChord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Quality as Q;

        let mut s = format!("{:?}", self.degree);

        if matches!(self.triad_quality, Q::Minor | Q::Diminished) {
            s = s.to_ascii_lowercase();
        }

        fn push_irregular_quality(s: &mut String, triad: Quality, seventh: Quality) {
            fn quality_char(q: Quality) -> char {
                match q {
                    Q::Major => 'M',
                    Q::Minor => 'm',
                    Q::Diminished => 'd',
                    Q::Augmented => 'A',
                }
            }

            s.push('(');
            s.push(quality_char(triad));
            s.push(quality_char(seventh));
            s.push(')');
        }

        match (self.triad_quality, self.seventh_quality()) {
            (Q::Major, None) => { /* none */ },
            (Q::Major, Some(Q::Major)) => { /* none */ },
            (Q::Major, Some(Q::Minor)) if self.degree == ScaleDegree::V => { /* none */ }, // dominant
            (Q::Major, Some(Q::Minor)) => push_irregular_quality(&mut s, Q::Major, Q::Minor),
            (Q::Major, Some(seventh)) => push_irregular_quality(&mut s, Q::Major, seventh),
            (Q::Minor, None) => { /* none */ },
            (Q::Minor, Some(Q::Minor)) => { /* none */ },
            (Q::Minor, Some(seventh)) => push_irregular_quality(&mut s, Q::Minor, seventh),
            (Q::Augmented, None) => s.push('+'),
            (Q::Augmented, Some(seventh)) => push_irregular_quality(&mut s, Q::Augmented, seventh),
            (Q::Diminished, None) => s.push('o'),
            (Q::Diminished, Some(Q::Diminished)) => s.push('o'),
            (Q::Diminished, Some(Q::Minor)) => s.push('ø'),
            (Q::Diminished, Some(seventh)) => push_irregular_quality(&mut s, Q::Diminished, seventh),
        }

        match (self.has_seventh(), self.inversion) {
            // root
            (false, 0) => { /* none */ },
            (true, 0) => s.push('7'),
            // first
            (false, 1) => s.push('6'),
            (true, 1) => s.push_str("6/5"),
            // second
            (false, 2) => s.push_str("6/4"),
            (true, 2) => s.push_str("4/3"),
            // third
            (true, 3) => s.push_str("4/2"),
            _ => unreachable!("invalid inversion for chord type"),
        }

        f.write_str(&s)
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum RomanChordFromStrError {
    #[error("Invalid numeral")]
    InvalidNumeral,
    #[error("Couldn't parse a valid quality")]
    InvalidQuality,
    #[error("Invalid inversion")]
    InvalidInversion,
}

impl FromStr for RomanChord {
    type Err = RomanChordFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Quality as Q;

        let non_numeral = s.char_indices()
            .find(|(_, c)| !matches!(c.to_ascii_uppercase(), 'I' | 'V'))
            .map(|(i, _)| i)
            .unwrap_or(s.len());

        let numeral = s[..non_numeral]
            .to_ascii_uppercase()
            .parse::<ScaleDegree>()
            .map_err(|_| RomanChordFromStrError::InvalidNumeral)?;

        let is_upper = {
            let is_upper = s[..non_numeral].chars().all(|c| c.is_ascii_uppercase());
            let is_lower = s[..non_numeral].chars().all(|c| c.is_ascii_lowercase());

            if !is_upper && !is_lower {
                return Err(RomanChordFromStrError::InvalidQuality);
            }

            is_upper
        };

        let rest = &s[non_numeral..];
        let mut rest_chars = rest.chars().peekable();

        let (triad_quality, seventh_quality, explicit) = match rest_chars.peek() {
            Some('+') if is_upper => {
                rest_chars.next();

                (Q::Augmented, None, false)
            },
            Some('+') if !is_upper => return Err(RomanChordFromStrError::InvalidQuality),
            Some('o') if !is_upper => {
                rest_chars.next();

                (Q::Diminished, Some(Q::Diminished), false)
            },
            Some('ø') if !is_upper => {
                rest_chars.next();

                (Q::Diminished, Some(Q::Minor), true)
            },
            Some('o') | Some('ø') if is_upper => return Err(RomanChordFromStrError::InvalidQuality),
            Some('(') => {
                rest_chars.next();

                fn from_quality_char(c: char) -> Option<Quality> {
                    match c {
                        'M' => Some(Q::Major),
                        'm' => Some(Q::Minor),
                        'd' => Some(Q::Diminished),
                        'A' => Some(Q::Augmented),
                        _ => None,
                    }
                }

                let triad_quality = from_quality_char(
                    rest_chars.next().ok_or(RomanChordFromStrError::InvalidQuality)?
                )
                    .ok_or(RomanChordFromStrError::InvalidQuality)?;

                let seventh_quality = from_quality_char(
                    rest_chars.next().ok_or(RomanChordFromStrError::InvalidQuality)?
                )
                    .ok_or(RomanChordFromStrError::InvalidQuality)?;

                if rest_chars.next() != Some(')') {
                    return Err(RomanChordFromStrError::InvalidQuality);
                }

                if matches!(
                    (triad_quality, is_upper),
                    (Q::Major | Q::Augmented, false) | (Q::Minor | Q::Diminished, true)
                ) {
                    return Err(RomanChordFromStrError::InvalidQuality);
                }

                (triad_quality, Some(seventh_quality), true)
            },
            _ if is_upper => (Q::Major, Some(Q::Major), false),
            _ if !is_upper => (Q::Minor, Some(Q::Minor), false),
            _ => unreachable!("all cases covered"),
        };

        use inversions::*;

        let inversion = rest_chars.collect::<String>();

        // this parses V7 as dominant seventh
        // if !explicit && matches!((numeral, triad_quality, seventh_quality), (ScaleDegree::V, Quality::Major, Some(Quality::Major))) {
        //     seventh_quality = Some(Quality::Minor)
        // }

        // TODO: this collect isn't ideal :(
        match inversion.as_str() {
            "" if !explicit => Ok(Self::triad(numeral, triad_quality)),
            "6" if !explicit => Ok(Self::triad(numeral, triad_quality).with_inversion(INV_6).expect("valid")),
            "6/4" if !explicit => Ok(Self::triad(numeral, triad_quality).with_inversion(INV_64).expect("valid")),

            "6" | "6/4" if explicit => Err(RomanChordFromStrError::InvalidInversion),

            "" if explicit => Ok(Self::seventh(numeral, triad_quality, seventh_quality.expect("should be some"))),
            "7" if seventh_quality.is_some() => Ok(
                Self::seventh(numeral, triad_quality, seventh_quality.expect("should be some"))
            ),
            "6/5" if seventh_quality.is_some() => Ok(
                Self::seventh(numeral, triad_quality, seventh_quality.expect("should be some"))
                    .with_inversion(INV_65).expect("valid")
            ),
            "4/3" if seventh_quality.is_some() => Ok(
                Self::seventh(numeral, triad_quality, seventh_quality.expect("should be some"))
                    .with_inversion(INV_43).expect("valid")
            ),
            "4/2" if seventh_quality.is_some() => Ok(
                Self::seventh(numeral, triad_quality, seventh_quality.expect("should be some"))
                    .with_inversion(INV_42).expect("valid")
            ),

            _ => Err(RomanChordFromStrError::InvalidInversion),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use strum::IntoEnumIterator;
    use crate::key::Key;
    use crate::pitch::Pitch;
    use super::{RomanChord, ScaleDegree, Quality};

    fn assert_diatonic_chord(key: Key, degree: ScaleDegree, triad: Quality, seventh: Quality) {
        let expected = RomanChord::seventh(degree, triad, seventh);
        let got = RomanChord::diatonic_in_key(degree, key, true);

        assert_eq!(expected, got, "expected: {expected}, got {got}");
    }

    #[test]
    fn major_diatonic() {
        use ScaleDegree as D;
        use Quality as Q;

        let maj = Key::major(Pitch::C);

        assert_diatonic_chord(maj, D::I, Q::Major, Q::Major);
        assert_diatonic_chord(maj, D::II, Q::Minor, Q::Minor);
        assert_diatonic_chord(maj, D::III, Q::Minor, Q::Minor);
        assert_diatonic_chord(maj, D::IV, Q::Major, Q::Major);
        assert_diatonic_chord(maj, D::V, Q::Major, Q::Minor);
        assert_diatonic_chord(maj, D::VI, Q::Minor, Q::Minor);
        assert_diatonic_chord(maj, D::VII, Q::Diminished, Q::Minor);
    }

    #[test]
    fn minor_diatonic() {
        use ScaleDegree as D;
        use Quality as Q;

        let min = Key::minor(Pitch::C);

        assert_diatonic_chord(min, D::I, Q::Minor, Q::Minor);
        assert_diatonic_chord(min, D::II, Q::Diminished, Q::Minor);
        assert_diatonic_chord(min, D::III, Q::Major, Q::Major);
        assert_diatonic_chord(min, D::IV, Q::Minor, Q::Minor);
        assert_diatonic_chord(min, D::V, Q::Major, Q::Minor);
        assert_diatonic_chord(min, D::VI, Q::Major, Q::Major);
        assert_diatonic_chord(min, D::VII, Q::Diminished, Q::Diminished);
    }

    #[test]
    fn from_str_correct() {
        let qualities = [Quality::Major, Quality::Minor, Quality::Diminished, Quality::Augmented];
        let some_qualities = qualities.map(Some);

        for deg in ScaleDegree::iter() {
            for triad in qualities {
                for seventh_qualities in some_qualities.iter()
                    .copied()
                    .chain(std::iter::once(None))
                {
                    for inv in 0..=3 {
                        if let Ok(chord) = RomanChord::new(deg, triad, seventh_qualities, inv)
                            && !matches!((deg, triad, seventh_qualities), (ScaleDegree::V, Quality::Major, Some(Quality::Minor)))
                        {
                            assert_eq!(
                                Ok(chord), chord.to_string().parse(),
                                "Failed to parse: {chord}",
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
        fn from_str_invalid_inputs() {
        use super::RomanChordFromStrError as Error;

        assert_eq!(
            RomanChord::from_str(""), Err(Error::InvalidNumeral),
            "empty string should fail"
        );

        assert_eq!(
            RomanChord::from_str("VIII"), Err(Error::InvalidNumeral),
            "invalid numeral should fail"
        );

        assert_eq!(
            RomanChord::from_str("X"), Err(Error::InvalidNumeral),
            "invalid characters in numeral position should fail"
        );

        assert_eq!(
            RomanChord::from_str("Iv"), Err(Error::InvalidQuality),
            "mixed case in numeral (inconsistent case) should fail"
        );

        assert_eq!(
            RomanChord::from_str("iv+"), Err(Error::InvalidQuality),
            "lowercase augmented marker should fail"
        );

        assert_eq!(
            RomanChord::from_str("IVo"), Err(Error::InvalidQuality),
            "uppercase diminished marker should fail"
        );

        assert_eq!(
            RomanChord::from_str("IIø"), Err(Error::InvalidQuality),
            "half-diminished on uppercase should fail"
        );

        assert_eq!(
            RomanChord::from_str("I(Mm"), Err(Error::InvalidQuality),
            "missing closing paren should fail"
        );

        assert_eq!(
            RomanChord::from_str("I(MX)"), Err(Error::InvalidQuality),
            "invalid quality parenthetical should fail"
        );

        assert_eq!(
            RomanChord::from_str("i(Mm)"), Err(Error::InvalidQuality),
            "invalid wrong case should fail"
        );

        assert_eq!(
            RomanChord::from_str("I8"), Err(Error::InvalidInversion),
            "invalid inversion marker should fail"
        );

        assert_eq!(
            RomanChord::from_str("I+7"), Err(Error::InvalidInversion),
            "augmented triad cannot be a seventh, since seventh's quality is ambiguous"
        );

        assert_eq!(
            RomanChord::from_str("III+4/3"), Err(Error::InvalidInversion),
            "augmented triad cannot use seventh inversion"
        );

        assert_eq!(
            RomanChord::from_str("I(Mm)6"), Err(Error::InvalidInversion),
            "explicit seventh quality cannot use triad inversions"
        );

        assert_eq!(
            RomanChord::from_str("V(Mm)6/4"), Err(Error::InvalidInversion),
            "explicit seventh quality cannot use triad inversions"
        );

        assert_eq!(
            RomanChord::from_str("iiø6"), Err(Error::InvalidInversion),
            "half-diminished cannot use triad inversions"
        );

        assert_eq!(
            RomanChord::from_str("viiø6/4"), Err(Error::InvalidInversion),
            "half-diminished cannot use triad inversions"
        );
    }
}