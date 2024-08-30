use crate::chord::Chord;
use crate::enharmonic::EnharmonicEq;
use crate::note::pitch::Pitch;

impl Chord {
    // TODO: is there a more efficient way of doing this?
    fn eq_helper<C: Eq, T: Ord>(&self, rhs: &Self, map: &impl Fn(Pitch) -> C, mut sort_by_key: &mut impl FnMut(&C) -> T) -> bool {
        if self.intervals.len() != rhs.intervals().len() {
            return false;
        }

        let mut lhs = self.pitches()
            .into_iter()
            .map(&map)
            .collect::<Vec<_>>();

        let mut rhs = rhs.pitches()
            .into_iter()
            .map(&map)
            .collect::<Vec<_>>();

        lhs.sort_unstable_by_key(&mut sort_by_key);
        rhs.sort_unstable_by_key(&mut sort_by_key);

        rhs == lhs
    }

    pub fn eq_enharmonic_strict(&self, rhs: &Self) -> bool {
        self.eq_helper(
            rhs,
            &std::convert::identity,
            &mut Pitch::as_fifths_from_c
        )
    }
}

impl EnharmonicEq for Chord {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.eq_helper(
            rhs,
            &|p| p.as_pitch_class(),
            &mut |pc| *pc as u8
        )
    }
}