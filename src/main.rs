pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;

use strum::IntoEnumIterator;
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::note::pitch::Pitch;

fn main() {
    for s in IntervalSize::iter() {
        for q in IntervalQuality::iter() {

            if let Some(i) = Interval::from_quality_and_size(q, s) {
                let p = Pitch::GSharp;
                let p2 = p.apply_interval(&i);

                let Some(p2) = p2 else {
                    continue;
                };

                println!("{p:?} + {} = {p2:?} ({:?})", i.shorthand(), p2.as_pitch_class());
            }
        }

        println!();
    }

    // let p = Pitch::C;
    // let i = Interval::from_quality_and_size(
    //     IntervalQuality::Augmented,
    //     IntervalSize::Unison,
    // ).unwrap();
    // //
    // let p = p.apply_interval(&i).unwrap();
    //
    // println!("{p:?} ({:?})", p.as_pitch_class());

    // println!("{:?}",
    //     Pitch::C.semitones_between(Pitch::D)
    // )
}
