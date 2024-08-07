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
    for pt in Pitch::iter() {
        for ty in ChordType::iter() {
            let c = Chord::from_type(ty, pt, 0).expect("valid inversion");

            if let Some(p) = c.pitches() {
                println!("{pt:?} {ty:?} = {p:?}");
            } else {
                // println!("Can't make a {pt:?} {ty:?}, since it requires more than a double flat/sharp");
            }
        }

        println!();
    }
}
