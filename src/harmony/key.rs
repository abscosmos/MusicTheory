use crate::interval::Interval;
use crate::pitch::{Pitch, Letter, AccidentalSign, Spelling};
use crate::harmony::mode::DiatonicMode;
use crate::scales::definition::heptatonic::{DiatonicMode as DiatonicModeExperimental, DiatonicScale};
use crate::scales::rooted::RootedSizedScale;
use crate::scales::{Numeral7, ScaleMode as _};
use crate::scales::sized_scale::SizedScale as _;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key {
    pub tonic: Pitch,
    pub mode: DiatonicMode,
}

impl Key {
    pub fn new(tonic: Pitch, mode: DiatonicMode) -> Self {
        Self { tonic, mode }
    }
    
    pub fn major(tonic: Pitch) -> Self {
        Self::new(tonic, DiatonicMode::MAJOR)
    }

    pub fn minor(tonic: Pitch) -> Self {
        Self::new(tonic, DiatonicMode::NATURAL_MINOR)
    }

    pub fn with_tonic(self, tonic: Pitch) -> Self {
        Self { tonic, .. self }
    }

    pub fn from_sharps(sharps: i16, mode: DiatonicMode) -> Self {
        let offset = Letter::from_step(mode as u8 - 1)
            .expect("mode is in [1, 7], so subtracting 1 should be in range")
            .fifths_from_c();
        
        Self::new(Pitch::from_fifths_from_c(sharps + offset), mode)
    }
    
    pub fn sharps(self) -> i16 {
        let offset = Letter::from_step(self.mode as u8 - 1)
            .expect("mode is in [1, 7], so subtracting 1 should be in range")
            .fifths_from_c();
        
        self.tonic.as_fifths_from_c() - offset
    }

    /// Returns the spelling preference (sharps or flats) for this key.
    ///
    /// In other words, does this key use sharps or flats? Since keys like C major use neither
    /// sharps nor flats, this method can return `None`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Sharp keys prefer sharp spelling
    /// assert_eq!(Key::major(Pitch::G).spelling(), Some(Spelling::Sharps));
    /// assert_eq!(Key::minor(Pitch::E).spelling(), Some(Spelling::Sharps));
    ///
    /// // Flat keys prefer flat spelling
    /// assert_eq!(Key::major(Pitch::F).spelling(), Some(Spelling::Flats));
    /// assert_eq!(Key::minor(Pitch::D).spelling(), Some(Spelling::Flats));
    ///
    /// // C major and A minor have no preference
    /// assert_eq!(Key::major(Pitch::C).spelling(), None);
    /// assert_eq!(Key::minor(Pitch::A).spelling(), None);
    /// ```
    pub fn spelling(self) -> Option<Spelling> {
        match self.sharps() {
            ..0 => Some(Spelling::Flats),
            0 => None,
            1.. => Some(Spelling::Sharps),
        }
    }
    
    pub fn try_from_sharps_tonic(sharps: i16, tonic: Pitch) -> Option<Self> {
        let major_tonic = Pitch::from_fifths_from_c(sharps);

        let pos = DiatonicScale::new(DiatonicMode::MAJOR.as_experimental())
            .build_from(major_tonic)
            .iter()
            .position(|p| *p == tonic)?;

        let mode = DiatonicModeExperimental::from_num((pos + 1) as _)
            .expect("should be within [1,7]");
        
        Some(Self::new(tonic, DiatonicMode::from_experimental(mode)))
    }

    pub fn parallel(self, mode: DiatonicMode) -> Self {
        Self { mode, .. self }
    }

    pub fn relative(self, mode: DiatonicMode) -> Self {
        // what key has all white keys in this mode?
        let lhs_ref = Letter::from_step(self.mode as u8 - 1).expect("mode enum should be same size as letter enum");
        let rhs_ref = Letter::from_step(mode as u8 - 1).expect("mode enum should be same size as letter enum");

        let diff_fifths = Pitch::from(rhs_ref).as_fifths_from_c() - Pitch::from(lhs_ref).as_fifths_from_c();
        
        let new_tonic = self.tonic.transpose_fifths(diff_fifths);
        
        self.with_tonic(new_tonic).parallel(mode)
    }
    
    pub fn from_pitch_degree(degree: Numeral7, pitch: Pitch, mode: DiatonicMode) -> Self {
        let offset = degree as u8 - 1;
        
        let letter_step = (pitch.letter().step() + 7 - offset) % 7;
        
        let letter = Letter::from_step(letter_step)
            .expect("must be in range of [0,6]");

        let expect = RootedSizedScale { root: Pitch::from(letter), scale: DiatonicScale::new(mode.as_experimental()) }.get(degree);
        
        assert_eq!(
            pitch.letter(), expect.letter(),
            "both letters should be the same if we're comparing accidentals"
        );
        
        let offset = pitch.accidental().offset - expect.accidental().offset;
        
        Self {
            tonic: Pitch::from_letter_and_accidental(letter, AccidentalSign { offset }),
            mode,
        }
    }
    
    pub fn transpose(&self, interval: Interval) -> Self {
        self.with_tonic(self.tonic.transpose(interval))
    }
    
    pub fn alterations(&self) -> Vec<Pitch> {
        let mut accidentals = self.scale_experimental()
            .build_default()
            .into_iter()
            .filter(|a| a.accidental() != AccidentalSign::NATURAL)
            .collect::<Vec<_>>();
        
        accidentals.sort_unstable_by_key(|p| p.as_fifths_from_c());

        assert_eq!(
            accidentals.iter().map(|p| p.accidental().offset).sum::<i16>(), self.sharps(),
            "total accidentals should equal sharps of key"
        );
        
        accidentals
    }
    
    pub fn accidental_of(&self, letter: Letter) -> AccidentalSign {
        let degree = Numeral7::from_num(self.tonic.letter().offset_between(letter) + 1)
            .expect("offset should be in range");

        let pitch = self.scale_experimental().get(degree);

        assert_eq!(
            pitch.letter(), letter,
            "should have gotten the correct letter"
        );
        
        pitch.accidental()
    }

    pub fn relative_pitch(self, degree: Numeral7) -> Pitch {
        self.scale_experimental().get(degree)
    }
    
    // this is pub(crate) since it's reliant on 'experimental-scales', so it shouldn't be public
    // if the feature is enabled, the 'scale' method should be used instead 
    pub(crate) fn scale_experimental(&self) -> RootedSizedScale<Pitch, 7, DiatonicScale> {
        RootedSizedScale {
            root: self.tonic,
            scale: DiatonicScale::new(self.mode.as_experimental()),
        }
    }

    #[cfg(feature = "experimental-scales")]
    pub fn scale(&self) -> RootedSizedScale<Pitch, 7, DiatonicScale> {
        self.scale_experimental()
    }

    #[cfg(feature = "experimental-scales")]
    pub fn chord_scales(&self) -> [RootedSizedScale<Pitch, 7, DiatonicScale>; 7] {
        use std::array;

        let scale = self.scale_experimental().build_default();
        
        array::from_fn(|i| {
            let mode = DiatonicModeExperimental::from_num((self.mode.as_experimental().as_num() - 1 + i as u8) % 7 + 1)
                .expect("should be in [1, 7]");
            
            let root = *scale.get(i).expect("scale and ret array should be the same size");
            
            RootedSizedScale { root, scale: DiatonicScale::new(mode) }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::harmony::{DiatonicMode, Key};

    #[test]
    fn relative() {
        let modes = (1..8).map(|n|DiatonicMode::from_repr(n).expect("in range"));

        for sharps in -10..=10 {
            for mode in modes.clone() {
                let key = Key::from_sharps(sharps, mode);

                for mode in modes.clone() {
                    let relative = key.relative(mode);

                    assert_eq!(
                        relative.mode, mode,
                        "mode should match requested relative",
                    );

                    assert_eq!(
                        relative.sharps(), key.sharps(),
                        "relative mode should have same amount of sharps/flats",
                    );
                }
            }
        }


    }
}