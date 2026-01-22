use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use music_theory::harmony::Key;
use music_theory::{Pitch, Note};
use music_theory::voice_leading::roman_chord::{RomanChord, ScaleDegree};
use music_theory::voice_leading::{solve::brute_force_search, Voicing};
use music_theory::voice_leading::solve::generate_voice_leadings;

fn progression() -> (Key, [RomanChord; 6]) {
    use ScaleDegree as D;

    let key = Key::major(Pitch::E_FLAT);

    let progression = [
        RomanChord::diatonic_in_key(D::I, key, false),
        RomanChord::diatonic_in_key(D::V, key, false).with_inversion(1).expect("valid inversion"),
        RomanChord::diatonic_in_key(D::I, key, false),
        RomanChord::diatonic_in_key(D::IV, key, false),
        RomanChord::diatonic_in_key(D::V, key, true),
        RomanChord::diatonic_in_key(D::I, key, false),
    ];

    (key, progression)
}

fn compare_with_brute_force(c: &mut Criterion) {
    let (key, progression) = progression();

    let mut group = c.benchmark_group("VL Solver, medium");

    group.bench_function("Brute force solver", |b|
        b.iter(|| brute_force_search(&progression[2..], key, None))
    );

    group.bench_function("Backtracking solver", |b|
        b.iter(|| generate_voice_leadings(&progression[2..], key, None).expect("no starting voicing, so should not error"))
    );

    group.finish();
}

fn backtracking(c: &mut Criterion) {
    let (key, progression) = progression();

    let chord_i = Voicing([
        Note::new(Pitch::B_FLAT, 4),
        Note::new(Pitch::E_FLAT, 4),
        Note::new(Pitch::G, 3),
        Note::new(Pitch::E_FLAT, 3),
    ]);

    let chord_v6 = Voicing([
        Note::new(Pitch::B_FLAT, 4),
        Note::new(Pitch::F, 4),
        Note::new(Pitch::F, 3),
        Note::new(Pitch::D, 3),
    ]);

    let mut group = c.benchmark_group("VL Solver");

    let inputs: [Option<&[Voicing]>; _] = [
        None,
        Some(&[chord_i]),
        Some(&[chord_i, chord_v6]),
        Some(&[chord_i, chord_v6, chord_i]),
    ];

    for starting_chord in inputs {
        let id = format!("VL Solver: backtracking solver, {} starting chords", starting_chord.map_or(0, <[_]>::len));

        group.bench_with_input(id, &starting_chord, |b, &starting_chord| {
            b.iter(||
                generate_voice_leadings(black_box(&progression), black_box(key), black_box(starting_chord))
                    .expect("starting voicing should be valid, if present")
            )
        });
    }

    group.finish();
}


criterion_group!(benches, compare_with_brute_force, backtracking);

criterion_main!(benches);