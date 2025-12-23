use std::array;
use serde::{Deserialize, Serialize};
use crate::accidental::AccidentalSign;
use crate::interval::Interval;
use crate::letter::Letter;
use crate::pitch::Pitch;
use crate::scales::heptatonic::{DiatonicMode, DiatonicScale};
use crate::scales::rooted::RootedSizedScale;
use crate::scales::{Numeral7, ScaleMode};
use crate::scales::sized_scale::SizedScale;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

    pub fn parallel(self, mode: DiatonicMode) -> Self {
        Self { mode, .. self }
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
    
    pub fn try_from_sharps_tonic(sharps: i16, tonic: Pitch) -> Option<Self> {
        let major_tonic = Pitch::from_fifths_from_c(sharps);

        let pos = DiatonicScale::new(DiatonicMode::MAJOR)
            .build_from(major_tonic)
            .iter()
            .position(|p| *p == tonic)?;

        let mode = DiatonicMode::from_num((pos + 1) as _)
            .expect("should be within [1,7]");
        
        Some(Self::new(tonic, mode))
    }

    pub fn relative(self, mode: DiatonicMode) -> Self {
        Self::from_sharps(self.sharps(), mode)
    }
    
    pub fn from_pitch_degree(degree: Numeral7, pitch: Pitch, mode: DiatonicMode) -> Self {
        let offset = degree as u8 - 1;
        
        let letter_step = (pitch.letter().step() + 7 - offset) % 7;
        
        let letter = Letter::from_step(letter_step)
            .expect("must be in range of [0,6]");

        let expect = RootedSizedScale { root: Pitch::from(letter), scale: DiatonicScale::new(mode) }.get(degree);
        
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
        let mut accidentals = self.scale()
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

        let pitch = self.scale().get(degree);

        assert_eq!(
            pitch.letter(), letter,
            "should have gotten the correct letter"
        );
        
        pitch.accidental()
    }
    
    pub fn relative_pitch(self, degree: Numeral7) -> Pitch {
        self.scale().get(degree)
    }

    pub fn scale(&self) -> RootedSizedScale<Pitch, 7, DiatonicScale> {
        RootedSizedScale {
            root: self.tonic,
            scale: DiatonicScale::new(self.mode),
        }
    }
    
    pub fn chord_scales(&self) -> [RootedSizedScale<Pitch, 7, DiatonicScale>; 7] {
        let scale = self.scale().build_default();
        
        array::from_fn(|i| {
            let mode = DiatonicMode::from_num((self.mode.as_num() - 1 + i as u8) % 7 + 1)
                .expect("should be in [1, 7]");
            
            let root = *scale.get(i).expect("scale and ret array should be the same size");
            
            RootedSizedScale { root, scale: DiatonicScale::new(mode) }
        })
    }
}