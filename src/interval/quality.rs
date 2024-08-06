use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Diminished,
    Augmented,
    DoublyDiminished,
    DoublyAugmented,
}