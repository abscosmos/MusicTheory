use strum_macros::EnumIter;
use crate::interval::Interval;

// from https://en.wikipedia.org/wiki/Chord_(music)
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum ChordType {
    MajorTriad,
    MinorTriad,
    AugmentedTriad,
    DiminishedTriad,
    Suspended2Triad,
    Suspended4Triad,

    Fifth,
    MajorSixth,
    MinorSixth,
    MinorFlatSixth,

    DiminishedSeventh,
    HalfDiminishedSeventh,
    MinorSeventh,
    MinorMajorSeventh,
    DominantSeventh,
    MajorSeventh,
    AugmentedSeventh,
    AugmentedMajorSeventh,
    DominantSeventhFlatFive,

    DominantNinth,
    DominantEleventh,
    DominantThirteenth,

    Lydian,
    LydianAugmented,

    SeventhAugmentedFifth,
    SeventhMinorNinth,
    SeventhSharpNinth,
    SeventhAugmentedEleventh,
    SeventhDiminishedThirteenth,

    AddTwo,
    AddFourth,
    AddSixth,
    AddNine,
    SixNine,
    SevenSix,
    MixedThird,

    JazzSus,
}

impl ChordType {
    pub fn intervals(&self) -> Vec<Interval> {
        use ChordType as C;
        use Interval as I;

        match self {
            C::MajorTriad => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH],
            C::MinorTriad => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH],
            C::AugmentedTriad => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH],
            C::DiminishedTriad => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH],
            C::Suspended2Triad => vec![I::PERFECT_UNISON, I::MAJOR_SECOND, I::PERFECT_FIFTH],
            C::Suspended4Triad => vec![I::PERFECT_UNISON, I::PERFECT_FOURTH, I::PERFECT_FIFTH],

            C::Fifth => vec![I::PERFECT_UNISON, I::PERFECT_FIFTH],

            C::MajorSixth | C::AddSixth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH],
            C::MinorSixth => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH],
            C::MinorFlatSixth => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SIXTH],

            C::DiminishedSeventh => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH, I::DIMINISHED_SEVENTH],
            C::HalfDiminishedSeventh => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH, I::MINOR_SEVENTH],
            C::MinorSeventh => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH],
            C::MinorMajorSeventh => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH],
            C::DominantSeventh => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH],
            C::MajorSeventh => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH],
            C::AugmentedSeventh | C::SeventhAugmentedFifth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH, I::MINOR_SEVENTH],
            C::AugmentedMajorSeventh => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH, I::MAJOR_SEVENTH],
            C::DominantSeventhFlatFive => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::DIMINISHED_FIFTH, I::MINOR_SEVENTH],

            C::DominantNinth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH],
            C::DominantEleventh => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH],
            C::DominantThirteenth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH, I::MAJOR_THIRTEENTH],

            C::Lydian => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH, I::AUGMENTED_ELEVENTH],
            C::LydianAugmented => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH, I::MINOR_SEVENTH, I::AUGMENTED_ELEVENTH],

            C::SeventhMinorNinth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MINOR_NINTH],
            C::SeventhSharpNinth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::AUGMENTED_NINTH],
            C::SeventhAugmentedEleventh => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::AUGMENTED_ELEVENTH],
            C::SeventhDiminishedThirteenth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH, I::MINOR_THIRTEENTH],
            C::AddTwo => vec![I::PERFECT_UNISON, I::MAJOR_SECOND, I::MAJOR_THIRD, I::PERFECT_FIFTH],
            C::AddFourth => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FOURTH, I::PERFECT_FIFTH],
            C::AddNine => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_NINTH],
            C::SixNine => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH, I::MAJOR_NINTH],
            C::SevenSix => vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH, I::MINOR_SEVENTH],
            C::MixedThird => vec![I::PERFECT_UNISON, I::MINOR_THIRD, I::MAJOR_THIRD, I::PERFECT_FIFTH],
            C::JazzSus => vec![I::PERFECT_UNISON, I::PERFECT_FOURTH, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH],
        }
    }
}

