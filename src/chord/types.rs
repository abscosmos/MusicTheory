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
        use crate::intervals_vec as ivls;
        use ChordType as C;

        match self {
            C::MajorTriad => ivls!(Perfect Unison, Major Third, Perfect Fifth),
            C::MinorTriad => ivls!(Perfect Unison, Minor Third, Perfect Fifth),
            C::AugmentedTriad => ivls!(Perfect Unison, Major Third, Augmented Fifth),
            C::DiminishedTriad => ivls!(Perfect Unison, Minor Third, Diminished Fifth),
            C::Suspended2Triad => ivls!(Perfect Unison, Major Second, Perfect Fifth),
            C::Suspended4Triad => ivls!(Perfect Unison, Perfect Fourth, Perfect Fifth),

            C::Fifth => ivls!(Perfect Unison, Perfect Fifth),

            C::MajorSixth | C::AddSixth => ivls!(Perfect Unison, Major Third, Perfect Fifth, Major Sixth),
            C::MinorSixth => ivls!(Perfect Unison, Minor Third, Perfect Fifth, Major Sixth),
            C::MinorFlatSixth => ivls!(Perfect Unison, Minor Third, Perfect Fifth, Minor Sixth),

            C::DiminishedSeventh => ivls!(Perfect Unison, Minor Third, Diminished Fifth, Diminished Seventh),
            C::HalfDiminishedSeventh => ivls!(Perfect Unison, Minor Third, Diminished Fifth, Minor Seventh),
            C::MinorSeventh => ivls!(Perfect Unison, Minor Third, Perfect Fifth, Minor Seventh),
            C::MinorMajorSeventh => ivls!(Perfect Unison, Minor Third, Perfect Fifth, Major Seventh),
            C::DominantSeventh => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh),
            C::MajorSeventh => ivls!(Perfect Unison, Major Third, Perfect Fifth, Major Seventh),
            C::AugmentedSeventh | C::SeventhAugmentedFifth => ivls!(Perfect Unison, Major Third, Augmented Fifth, Minor Seventh),
            C::AugmentedMajorSeventh => ivls!(Perfect Unison, Major Third, Augmented Fifth, Major Seventh),
            C::DominantSeventhFlatFive => ivls!(Perfect Unison, Major Third, Diminished Fifth, Minor Seventh),

            C::DominantNinth => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Major Ninth),
            C::DominantEleventh => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Major Ninth, Perfect Eleventh),
            C::DominantThirteenth => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Major Ninth, Perfect Eleventh, Major Thirteenth),

            C::Lydian => ivls!(Perfect Unison, Major Third, Perfect Fifth, Major Seventh, Augmented Eleventh),
            C::LydianAugmented => ivls!(Perfect Unison, Major Third, Augmented Fifth, Minor Seventh, Augmented Eleventh),

            C::SeventhMinorNinth => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Minor Ninth),
            C::SeventhSharpNinth => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Augmented Ninth),
            C::SeventhAugmentedEleventh => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Major Ninth, Augmented Eleventh),
            C::SeventhDiminishedThirteenth => ivls!(Perfect Unison, Major Third, Perfect Fifth, Minor Seventh, Major Ninth, Perfect Eleventh, Minor Thirteenth),
            C::AddTwo => ivls!(Perfect Unison, Major Second, Major Third, Perfect Fifth),
            C::AddFourth => ivls!(Perfect Unison, Major Third, Perfect Fourth, Perfect Fifth),
            C::AddNine => ivls!(Perfect Unison, Major Third, Perfect Fifth, Major Ninth),
            C::SixNine => ivls!(Perfect Unison, Major Third, Perfect Fifth, Major Sixth, Major Ninth),
            C::SevenSix => ivls!(Perfect Unison, Major Third, Perfect Fifth, Major Sixth, Minor Seventh),
            C::MixedThird => ivls!(Perfect Unison, Minor Third, Major Third, Perfect Fifth),
            C::JazzSus => ivls!(Perfect Unison, Perfect Fourth, Perfect Fifth, Minor Seventh, Major Ninth),
        }
    }
}

