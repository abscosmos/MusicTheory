use super::*;
use strum::IntoEnumIterator;

#[test]
fn simplify() {
    for offset in -5..5 {
        for letter in Letter::iter() {
            let acc = AccidentalSign { offset };

            let pitch = Pitch::from_letter_and_accidental(letter, acc);

            let simplified = pitch.simplified();

            assert!(
                pitch.eq_enharmonic(&simplified),
                "{pitch:?} ({:?}), {simplified:?} ({:?})",
                pitch.as_pitch_class(),
                simplified.as_pitch_class()
            );
        }
    }
}

#[test]
fn semitones_offset_from_c() {
    for pitch in Pitch::ALL_CONSTS {
        assert_eq!(
            pitch.semitones_offset_from_c().0.rem_euclid(12),
            Pitch::C.semitones_to(*pitch).0,
            "pitch={pitch}"
        );
    }
}

#[test]
fn test_respell_with() {
    let cases = [
        // respelling with flats
        (Pitch::C_SHARP, Spelling::Flats, Pitch::D_FLAT),
        (Pitch::D_SHARP, Spelling::Flats, Pitch::E_FLAT),
        (Pitch::E_DOUBLE_SHARP, Spelling::Flats, Pitch::G_FLAT),

        // respelling with sharps
        (Pitch::C_DOUBLE_FLAT, Spelling::Sharps, Pitch::A_SHARP),
        (Pitch::E_FLAT, Spelling::Sharps, Pitch::D_SHARP),
        (Pitch::G_FLAT, Spelling::Sharps, Pitch::F_SHARP),

        // naturals stay natural
        (Pitch::C, Spelling::Sharps, Pitch::C),
        (Pitch::C, Spelling::Flats, Pitch::C),
        (Pitch::D, Spelling::Sharps, Pitch::D),
        (Pitch::D, Spelling::Flats, Pitch::D),

        // already spelled correctly
        (Pitch::C_SHARP, Spelling::Sharps, Pitch::C_SHARP),
        (Pitch::D_FLAT, Spelling::Flats, Pitch::D_FLAT),
        (Pitch::G_DOUBLE_SHARP, Spelling::Sharps, Pitch::G_DOUBLE_SHARP),
        (Pitch::A_DOUBLE_FLAT, Spelling::Flats, Pitch::A_DOUBLE_FLAT),

        // double accidentals
        (Pitch::C_DOUBLE_SHARP, Spelling::Flats, Pitch::D),
        (Pitch::D_DOUBLE_FLAT, Spelling::Sharps, Pitch::C),

        // boundary without black keys
        (Pitch::C_FLAT, Spelling::Sharps, Pitch::B),
        (Pitch::C_FLAT, Spelling::Flats, Pitch::C_FLAT),
        (Pitch::F_FLAT, Spelling::Sharps, Pitch::E),
        (Pitch::E_SHARP, Spelling::Flats, Pitch::F),
    ];

    for (input, spelling, expected) in cases {
        assert_eq!(
            input.respell_with(spelling),
            expected,
            "{input} respelled with {spelling:?} should be {expected}",
        );
    }
}

#[test]
fn test_simplified() {
    let cases = [
        // single accidentals
        (Pitch::C_SHARP, Pitch::C_SHARP),
        (Pitch::D_FLAT, Pitch::D_FLAT),
        (Pitch::F_SHARP, Pitch::F_SHARP),
        (Pitch::A_FLAT, Pitch::A_FLAT),

        // naturals stay natural
        (Pitch::C, Pitch::C),
        (Pitch::D, Pitch::D),
        (Pitch::E, Pitch::E),
        (Pitch::F, Pitch::F),

        // double accidentals
        (Pitch::F_DOUBLE_SHARP, Pitch::G),
        (Pitch::G_DOUBLE_SHARP, Pitch::A),
        (Pitch::D_DOUBLE_FLAT, Pitch::C),
        (Pitch::E_DOUBLE_FLAT, Pitch::D),
        (Pitch::E_DOUBLE_SHARP, Pitch::F_SHARP),
        (Pitch::C_DOUBLE_FLAT, Pitch::B_FLAT),

        // boundaries without black keys
        (Pitch::C_FLAT, Pitch::B),
        (Pitch::B_SHARP, Pitch::C),
        (Pitch::F_FLAT, Pitch::E),
        (Pitch::E_SHARP, Pitch::F),
    ];

    for (input, expected) in cases {
        assert_eq!(
            input.simplified(),
            expected,
            "{input} simplified should be {expected}"
        );
    }
}

#[test]
fn test_enharmonic() {
    let cases = [
        // sharps -> flats
        (Pitch::F_SHARP, Pitch::G_FLAT),
        (Pitch::G_SHARP, Pitch::A_FLAT),

        // flats -> sharps
        (Pitch::D_FLAT, Pitch::C_SHARP),
        (Pitch::B_FLAT, Pitch::A_SHARP),

        // naturals
        (Pitch::E, Pitch::E),
        (Pitch::A, Pitch::A),

        // double accidentals -> naturals
        (Pitch::C_DOUBLE_SHARP, Pitch::D),
        (Pitch::D_DOUBLE_FLAT, Pitch::C),
        (Pitch::G_DOUBLE_FLAT, Pitch::F),

        // ... unless it needs an accidental
        (Pitch::E_DOUBLE_SHARP, Pitch::G_FLAT),
        (Pitch::C_DOUBLE_FLAT, Pitch::A_SHARP),

        // boundaries without black keys
        (Pitch::C_FLAT, Pitch::B),
        (Pitch::B_SHARP, Pitch::C),
        (Pitch::F_FLAT, Pitch::E),
        (Pitch::E_SHARP, Pitch::F),
    ];

    for (input, expected) in cases {
        assert_eq!(
            input.enharmonic(),
            expected,
            "{input} enharmonic should be {expected}"
        );
    }
}

#[test]
fn test_respell_in_key() {
    let cs_maj = Key::major(Pitch::C_SHARP);
    let cb_maj = Key::major(Pitch::C_FLAT);
    let f_maj = Key::major(Pitch::F);
    let d_maj = Key::major(Pitch::D);
    let e_min = Key::minor(Pitch::E);
    let c_maj = Key::major(Pitch::C);
    let gx_maj = Key::major(Pitch::G_DOUBLE_SHARP);

    let cases = [
        // C# major: C#, D#, E#, F#, G#, A#, B#
        (Pitch::C, cs_maj, Pitch::B_SHARP),
        (Pitch::F, cs_maj, Pitch::E_SHARP),
        (Pitch::C_SHARP, cs_maj, Pitch::C_SHARP),
        (Pitch::D_SHARP, cs_maj, Pitch::D_SHARP),

        // Cb major: Cb, Db, Eb, Fb, Gb, Ab, Bb
        (Pitch::B, cb_maj, Pitch::C_FLAT),
        (Pitch::E, cb_maj, Pitch::F_FLAT),
        (Pitch::C_FLAT, cb_maj, Pitch::C_FLAT),
        (Pitch::D_FLAT, cb_maj, Pitch::D_FLAT),

        // F major: F, G, A, Bb, C, D, E
        (Pitch::A_SHARP, f_maj, Pitch::B_FLAT),
        (Pitch::B_FLAT, f_maj, Pitch::B_FLAT),
        (Pitch::F, f_maj, Pitch::F),

        // chromatic notes preserve original spelling
        (Pitch::C_SHARP, f_maj, Pitch::C_SHARP),
        (Pitch::D_FLAT, f_maj, Pitch::D_FLAT),
        (Pitch::E_FLAT, d_maj, Pitch::E_FLAT),

        // E minor: E, F#, G, A, B, C, D
        (Pitch::G_FLAT, e_min, Pitch::F_SHARP),
        (Pitch::C, e_min, Pitch::C),

        // C major: naturals
        (Pitch::C, c_maj, Pitch::C),
        (Pitch::D, c_maj, Pitch::D),
        (Pitch::E, c_maj, Pitch::E),

        // keys with double accidentals
        (Pitch::A, gx_maj, Pitch::G_DOUBLE_SHARP),
        (Pitch::B, gx_maj, Pitch::A_DOUBLE_SHARP),
        (Pitch::C_SHARP, gx_maj, Pitch::B_DOUBLE_SHARP),
    ];

    for (input, key, expected) in cases {
        assert_eq!(
            input.respell_in_key(key),
            expected,
            "{input} respelled in {key:?} should be {expected}"
        );
    }
}