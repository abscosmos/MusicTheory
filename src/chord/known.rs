use crate::Interval;
use Interval as I;

struct KnownChordData {
    name: &'static str,
    intervals: &'static [Interval],
}

macro_rules! define_known_chords {
    ($( $variant:ident { name: $name:expr, intervals: [$($ivl:expr),+ $(,)?] } ),* $(,)?) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
        #[non_exhaustive]
        #[repr(u16)]
        pub enum KnownChord { $($variant),* }

        static CHORD_DATA: &[KnownChordData] = &[
            $( KnownChordData { name: $name, intervals: &[$($ivl),+] } ),*
        ];
    };
}

define_known_chords! {
    MajorTriad   { name: "major triad",   intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH]                   },
    MinorTriad   { name: "minor triad",   intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH]                   },
    MajorSeventh { name: "major seventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH] },
}

impl KnownChord {
    const fn data(self) -> &'static KnownChordData {
        &CHORD_DATA[self as usize]
    }

    pub(super) const fn name(self) -> &'static str {
        self.data().name
    }

    pub(super) const fn intervals(self) -> &'static [Interval] {
        self.data().intervals
    }
}
