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
    let mut possible_chords = Vec::new();

    for pt in Pitch::iter() {
        for ty in ChordType::iter() {
            let c = Chord::from_type(ty, pt, 0).expect("valid inversion");

            if let Some(p) = c.pitches() {
                // println!("{pt:?} {ty:?} = {p:?}");
                possible_chords.push(c);
            }
        }
        // println!();
    }

    for c in &possible_chords {
        println!("The chords equivalent with {:?} {:?} {:?}:", c.root, c.chord_type().unwrap(), c.pitches().unwrap());

        for cc in &possible_chords {
            if c != cc && c.eq_enharmonic_strict(cc) {
                println!("\t{:?} {:?} {:?}", cc.root, cc.chord_type().unwrap(), cc.pitches().unwrap());
            }
        }
        println!();
    }
}
