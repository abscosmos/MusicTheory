use crate::pitch::Pitch;
use crate::scales::heptatonic::DiatonicMode;

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
}

