pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;
pub mod chord;

use strum::IntoEnumIterator;
use crate::chord::Chord;
use crate::chord::types::ChordType;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::note::pitch::Pitch;

fn main() {
    let cmaj6 = Chord::from_type(ChordType::MajorSixth, Pitch::C, 0).unwrap();
    let am7 = Chord::from_type(ChordType::DiminishedSeventh, Pitch::A, 0).unwrap();

    println!("cmaj6 {:?}", cmaj6.pitches().unwrap());
    println!("Am7 {:?}", am7.pitches().unwrap());

    println!("{:?}", cmaj6.eq_enharmonic(&am7));
}
