use crate::chord::quality::ChordQuality;
use crate::chord::size::ChordSize;
use crate::interval::Interval;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::note::pitch::Pitch;

pub mod quality;
pub mod size;

struct ChordInfo {
    root: Pitch,
    inversion: u8,
}

enum ChordType {
    Regular {
        quality: ChordQuality,
        size: ChordSize,
    },
    Alternate(AlternateChordType)
}

pub enum AlternateChordType {
    JazzSus,
}

pub struct Chord {
    ty: ChordType,
    info: ChordInfo,
}

macro_rules! ivl {
    ($( $quality:ident $size:ident ),*) => {
        vec![
            $(
                Interval::from_quality_and_size(
                    IntervalQuality::$quality,
                    IntervalSize::$size
                ).expect("valid interval")
            ),*
        ]
    };
}




impl ChordType {
    fn intervals(&self) -> Vec<Interval> {
        use ChordQuality as Q;
        use ChordSize as S;

        use AlternateChordType as Alt;

        let ivl = |q, s| Interval::from_quality_and_size(q, s).expect("valid interval");

        // TODO: just make an enum of every type :/ ie Type::MajorTriad
        match self {
            ChordType::Regular { quality, size } => match (quality, size) {
                (Q::Major, S::Triad) => ivl!(Major Third, Perfect Fifth),
                (Q::Minor, S::Triad) => ivl!(Minor Third, Perfect Fifth),
                (Q::Augmented, S::Triad) => ivl!(Major Third, Augmented Fifth),
                (Q::Diminished, S::Triad) => ivl!(Minor Third, Diminished Fifth),
                (Q::Suspended2, S::Triad) => ivl!(Major Second, Perfect Fifth),
                (Q::Suspended4, S::Triad) => ivl!(Perfect Fourth, Perfect Fifth),

                (Q::Major, S::Sixth) => ivl!(Major Third, Perfect Fifth, Major Sixth),
                (Q::Minor, S::Sixth) => ivl!(Minor Third, Perfect Fifth, Major Sixth),
                (Q::MinorFlat, S::Sixth) => ivl!(Minor Third, Perfect Fifth, Minor Sixth),
                (Q::Augmented, S::Sixth) => ivl!(Major Third, Perfect Fifth, Augmented Sixth),

                (Q::MinorFlat, S::Sixth) => ivl!(Minor Third, Perfect Fifth, Minor Sixth),

                (_, _) => todo!(),
            }
            ChordType::Alternate(alt) => match alt {
                Alt::JazzSus => vec![ ivl(Q::Perfect, S::Fourth), ivl(Q::Perfect, S::Fifth), ivl(Q::Minor, S::Seventh), ivl(Q::Major, S::Ninth) ],
                _ => todo!(),
            },
        }
    }

    fn from_quality_and_size(quality: ChordQuality, size: ChordSize) -> Option<Self> {
        use ChordQuality as Q;
        use ChordSize as S;

        let unchecked = Self::Regular { quality, size };

        match (quality, size) {
            (Q::Major, S::Triad) | (Q::Minor, S::Triad) | (Q::Diminished, S::Triad) |
            (Q::Augmented, S::Triad) | (Q::Suspended2, S::Triad) | (Q::Suspended4, S::Triad) |

            (Q::Major, S::Sixth) | (Q::Minor, S::Sixth) | (Q::MinorFlat, S::Sixth) => Some(unchecked),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;
    use crate::chord::quality::ChordQuality;
    use crate::chord::size::ChordSize;
    use super::*;

    #[test]
    fn all_regular_chord_types_implemented() {
        use std::panic;

        for quality in ChordQuality::iter() {
            for size in ChordSize::iter() {

                match ChordType::from_quality_and_size(quality, size) {
                    Some(c) => {
                        assert!(panic::catch_unwind(|| { c.intervals() }).is_ok())
                    }
                    None => {
                        assert!(
                            panic::catch_unwind(|| { ChordType::Regular { quality, size }.intervals() }).is_err()
                        )
                    }
                }
            }
        }
    }
}