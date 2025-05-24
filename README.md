# Music Theory for Rust
A type safe library for computational music theory, designed to be simple, efficient, and powerful.

```rust
use music_theory::prelude::*;

// Create pitches:
let c_flat = Pitch::C_FLAT;
let a = Pitch::from_letter_and_accidental(Letter::A, AccidentalSign::NATURAL);

// Get distance between pitches
assert_eq!(c_flat.distance_to(a), Interval::AUGMENTED_SIXTH);

// Create & manipulate notes
let a4 = Note::new(a, 4);
let b7 = Note::new(Pitch::B, 7);

// Any size interval can be created, checked for validity
let maj_23 = Interval::new(IntervalQuality::Major, IntervalNumber::new(23).unwrap())
    .expect("should be a valid interval");

// Tranpose notes by intervals
assert_eq!(a4 + maj_23, b7);

// You can make keys
let e_maj = Key::major(Pitch::E);

assert_eq!(e_maj.alterations(), [Pitch::F_SHARP, Pitch::C_SHARP, Pitch::G_SHARP, Pitch::D_SHARP]);

// You can also use a key to turn a letter to a pitch in a key.
assert_eq!(Letter::G.to_pitch_in_key(e_maj), Pitch::G_SHARP);
```
You can find a more in-depth showcase [here](../main/examples/showcase.ipynb).

## Features
- Pitches with arbitrary accidentals
- Arbitrarily sized intervals (double diminished, triple augmented, ..)
- Tranpose pitches & notes by intervals
- Musical keys of any diatonic mode
- WIP implementation of scales
- WIP implementation of chords

## Installation
To install, add the following line to your `Cargo.toml`:
```toml
music_theory = { git = "https://github.com/abscosmos/MusicTheoryLib.git" }
```
**This library is in active development, so expect the APIs (and probably the name) to change.**
