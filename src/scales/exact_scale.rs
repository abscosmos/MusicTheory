use crate::Interval;
use crate::scales::ScaleDefinition;
use crate::scales::sized_scale::SizedScale;
use crate::scales::typed_scale::TypedScale;

// TODO(generic_const_exprs): N should eventually become an assoc constant
pub trait ExactScale<const N: usize>: Default {
    type Scale: ScaleDefinition<N>;
    
    fn as_typed(&self) -> TypedScale<Self::Scale, N>;
}

impl<const N: usize, S: ScaleDefinition<N>, E: ExactScale<N, Scale= S>> SizedScale<N> for E {
    fn relative_intervals(&self) -> [Interval; N] {
        self.as_typed().relative_intervals()
    }
}