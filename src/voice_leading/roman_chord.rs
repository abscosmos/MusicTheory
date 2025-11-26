use std::fmt;
use crate::interval::{Interval, IntervalQuality};
use crate::key::Key;
use crate::pitch::Pitch;
use crate::scales::heptatonic::DiatonicMode;
use strum_macros::FromRepr;

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
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromRepr)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
}


#[derive(Debug, thiserror::Error, Clone, Eq, PartialEq)]
#[error("Invalid inversion for chord type")]
pub struct InvalidInversionError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    fn mode_has_raised_leading_tone(mode: DiatonicMode) -> bool {
        matches!(mode, DiatonicMode::Aeolian | DiatonicMode::Dorian)
    }

    pub fn root_in_key(&self, key: Key) -> Pitch {
        let degree_index = (self.degree as u8 - 1) as usize;
        // TODO: this scale function is experimental
        let scale = key.scale().build_default();
        let mut root = scale[degree_index];

        // TODO: maybe this function is overkill?
        if self.should_raise_leading_tone(key).unwrap_or(false) {
            root = root.transpose(Interval::AUGMENTED_UNISON);
        }

        root
    }

    fn should_raise_leading_tone(&self, key: Key) -> Option<bool> {
        if !matches!(self.degree, ScaleDegree::V | ScaleDegree::VII) && Self::mode_has_raised_leading_tone(key.mode) {
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
            (Q::Major, Some(Q::Minor)) => { /* none */ }, // dominant
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

        match (self.seventh_quality().is_some(), self.inversion) {
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

#[cfg(test)]
mod tests {
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
}