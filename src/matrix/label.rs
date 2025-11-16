#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum TwelveToneRowForm {
    #[default]
    Prime,
    Retrograde,
    Inversion,
    RetrogradeInversion,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct TwelveToneRowLabel {
    pub form: TwelveToneRowForm,
    // TODO: wrapper struct?
    number: u8,
}

impl TwelveToneRowLabel {
    pub(crate) const COUNT: u8 = 12;

    pub fn new(form: TwelveToneRowForm, number: u8) -> Option<Self> {
        (number < Self::COUNT).then_some(Self { form, number })
    }

    pub fn number(&self) -> u8 {
        self.number
    }
}