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

impl IntervalQuality {
    pub fn shorthand(&self) -> &'static str {
        use IntervalQuality as Q;

        match self {
            Q::Perfect => "P",
            Q::Major => "M",
            Q::Minor => "m",
            Q::Diminished => "d",
            Q::Augmented => "A",
            Q::DoublyDiminished => "dd",
            Q::DoublyAugmented => "AA",
        }
    }
}