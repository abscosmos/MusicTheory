pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;
pub mod chord;
pub mod placed;
pub mod pitch_class;
pub mod accidental;
pub mod pitch;
pub mod letter;

use strum::IntoEnumIterator;
use crate::chord::Chord;
use crate::chord::types::ChordType;
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::interval::Interval;
use crate::interval::quality::IntervalQuality;
use crate::interval::size::IntervalSize;
use crate::accidental::AccidentalSign;
use crate::letter::Letter;
use crate::note::Note;
use crate::pitch::Pitch;

fn main() {
    // let mut possible_chords = Vec::new();

    // for pt in Pitch::iter() {
    //     for ty in ChordType::iter() {
    //         let c = Chord::from_type(ty, pt, 0).expect("valid inversion");
    //
    //         if let Some(p) = c.pitches() {
    //             println!("{pt:?} {ty:?} = {p:?}");
    //             possible_chords.push(c);
    //         }
    //     }
    //     // println!();
    // }

    // for c in &possible_chords {
    //     println!("The chords equivalent with {:?} {:?} {:?}:", c.root, c.chord_type().unwrap(), c.pitches().unwrap());
    //
    //     for cc in &possible_chords {
    //         if c != cc && c.eq_enharmonic_strict(cc) {
    //             println!("\t{:?} {:?} {:?}", cc.root, cc.chord_type().unwrap(), cc.pitches().unwrap());
    //         }
    //     }
    //     println!();
    // }

    for p in Pitch::ALL_CONSTS {
        for o in -2..2 {
            let note = Note { base: *p, octave: o };

            for q in IntervalQuality::iter() {
                for s in IntervalSize::iter() {
                    if let Some(i) = Interval::from_quality_and_size(q, s) {
                        let descending_note = note.apply_interval_descending(&i);

                        assert_eq!(
                            note.distance_from(&descending_note),
                            -i.semitones(),
                            "{:?}, {:?}, {:?}",
                            note, i, descending_note
                        );

                        let ascending_note = note.apply_interval_ascending(&i);

                        assert_eq!(
                            note.distance_from(&ascending_note),
                            i.semitones(),
                            "{:?}, {:?}, {:?}",
                            note, i, ascending_note
                        );

                    }
                }
            }
        }
    }

    let a = Note { base: Pitch::C_DOUBLE_FLAT, octave: 4 };
    let interval = Interval::from_quality_and_size(IntervalQuality::Major, IntervalSize::Third).unwrap();

    println!("{:?}", a.apply_interval_ascending(&interval));

    let c = Chord::from_type(ChordType::MinorMajorSeventh, Pitch::B_DOUBLE_SHARP, 0).unwrap();
    let c = c.pitches();

    println!("{c:?}");

    let p = Pitch::from_letter_and_accidental(Letter::C, AccidentalSign { offset: -287 });

    println!("Pitch: \"{p:}\" ({p:?}), Letter: {:?}", p.letter());

    for p in Pitch::ALL_CONSTS {
        println!("Pitch: \"{p:}\" ({p:?}), Letter: {:?}", p.letter());
    }
    // let a = Note { base: Pitch::D, octave: 4 };

    // let bs3 = Note { base: Pitch::BFlat, octave: 3 };
    // let c4 = Note { base: Pitch::CDoubleFlat, octave: 4 };
    //
    // println!("{:?}", c4.base.semitones_offset_from_c());
    //
    // println!("{:?}", bs3.distance_from(&c4));

    //     println!("{p:?}: {:?}", p.set_natural())
    // }

    // let c = Note { base: Pitch::C, octave: 1 };
    // let d = Note { base: Pitch::CSharp, octave: 1 };

    // println!("{:?}", c.distance_from(&d));
}
