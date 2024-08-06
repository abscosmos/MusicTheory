pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;

use strum::IntoEnumIterator;
use crate::enharmonic::EnharmonicEq;
use crate::interval::Interval;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;

fn main() {
    for q in IntervalQuality::iter() {
        for s in IntervalSize::iter() {

            match Interval::from_size_and_quality(s, q) {
                None => println!("you can't have a {q:?} {s:?}!"),
                Some(i) => println!("A {q:?} {s:?} has {:?} semitones!", i.semitones().0)
            }
        }
    }
}
