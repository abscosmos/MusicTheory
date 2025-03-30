use crate::accidental::AccidentalSign;
use crate::interval::Interval;
use crate::letter::Letter;
use crate::pitch::Pitch;
use crate::scales::heptatonic::{DiatonicMode, HeptatonicScaleModes};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Key {
    pub tonic: Pitch,
    pub mode: DiatonicMode,
}

impl Key {
    pub fn new(tonic: Pitch, mode: DiatonicMode) -> Self {
        Self { tonic, mode }
    }

    pub fn with_tonic(self, tonic: Pitch) -> Self {
        Self { tonic, .. self }
    }

    pub fn with_mode(self, mode: DiatonicMode) -> Self {
        Self { mode, .. self }
    }
    
    pub fn from_sharps(sharps: i16, mode: DiatonicMode) -> Self {
        Self::new(Pitch::from_fifths_from_c(sharps), mode)
    }

    pub fn parallel(self) -> Option<Self> {
        use DiatonicMode as M;

        match self.mode {
            M::MAJOR => Some(self.with_mode(M::NATURAL_MINOR)),
            M::NATURAL_MINOR => Some(self.with_mode(M::MAJOR)),
            _ => None,
        }
    }

    pub fn relative(self) -> Option<Self> {
        use DiatonicMode as M;
        
        let offset_fifths = match self.mode {
            M::MAJOR => 3,
            M::NATURAL_MINOR => -3,
            _ => return None,
        };
        
        let new_tonic = self.tonic.transpose_fifths(offset_fifths); 
        
        Some(
            self.with_tonic(new_tonic)
                .parallel()
                .expect("should be major/minor since we just checked")
        )
    }
    
    pub fn from_pitch_degree(degree: ScaleDegree, pitch: Pitch, mode: DiatonicMode) -> Self {
        let offset = degree as u8 - 1;
        
        let letter_step = (pitch.letter().step() + 7 - offset) % 7;
        
        let letter = Letter::from_step(letter_step)
            .expect("must be in range of [0,6]");
        
        let natural = Pitch::from(letter);
        
        let scale = mode.build_from(natural);
        
        let expect = *scale
            .get(offset as usize)
            .expect("offset must be within [0,7)");
        
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
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, strum_macros::FromRepr)]
pub enum ScaleDegree {
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}