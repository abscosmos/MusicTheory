use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use music_theory::key::Key;
use music_theory::note::Note;
use music_theory::pitch::Pitch;
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
        b.iter(|| generate_voice_leadings(&progression[2..], key, None))
    );

    group.finish();
}

fn backtracking(c: &mut Criterion) {
    let (key, progression) = progression();

    let starting_chord = Voicing([
        Note::new(Pitch::B_FLAT, 4),
        Note::new(Pitch::E_FLAT, 4),
        Note::new(Pitch::G, 3),
        Note::new(Pitch::E_FLAT, 3),
    ]);

    c.bench_function("VL Solver: backtracking solver, full & starting chord", |b|
        b.iter(|| generate_voice_leadings(&progression, key, Some(starting_chord)))
    );
}


criterion_group!(benches, compare_with_brute_force, backtracking);

criterion_main!(benches);