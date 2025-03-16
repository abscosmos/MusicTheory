use crate::interval::Interval;

pub mod heptatonic;

const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;

#[cfg(test)]
mod tests {
    use crate::pitch::Pitch;
    use crate::scales::heptatonic::{HeptatoniaPrimaMode, HeptatonicScaleModes};

    #[test]
    fn intervals() {
        let ivls = HeptatoniaPrimaMode::LOCRIAN.intervals();
        
        assert_eq!(ivls, HeptatoniaPrimaMode::LOCRIAN.build_from(Pitch::A).map(|p| Pitch::A.distance_to(&p)));
        
        assert_eq!(HeptatoniaPrimaMode::LOCRIAN.build_from(Pitch::A), ivls.map(|i| Pitch::A + i))
    }
}
