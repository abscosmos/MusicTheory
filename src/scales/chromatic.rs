use crate::interval::Interval;
use crate::scales::define::define_scale;
use crate::scales::exact_scale::ExactScale;
use crate::scales::typed_scale::TypedScale;
use super::S;

const A1: Interval = Interval::AUGMENTED_UNISON;

define_scale!(
    name = Chromatic,
    size = 12,
    intervals = [A1, S, A1, S, S, A1, S, A1, S, A1, S, S],
    mode = []
);

#[derive(Default)]
pub struct ChromaticScale;

impl ExactScale<12> for ChromaticScale {
    type Scale = ChromaticScaleDef;
    
    fn as_typed(&self) -> TypedScale<Self::Scale, 12> {
        TypedScale::new(ChromaticMode)
    }
}

