use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use music_theory::note::Note;
use music_theory::tuning::{JustIntonation, Tuning, TwelveToneEqualTemperament};

fn twelve_tet_tuning(c: &mut Criterion) {
    let mut g = c.benchmark_group("12TET");

    let notes = (u8::MIN..=u8::MAX).map(Note::from_midi);

    let tuning_eq_temp = TwelveToneEqualTemperament::A4_440;
    let tuning_ratios = JustIntonation::from_twelve_tet(tuning_eq_temp);

    g.bench_function("original", |b| b.iter(|| {
        for note in notes.clone() {
            let _hz = black_box(tuning_eq_temp.note_to_freq_hz(black_box(note)));
        }
    }));

    g.bench_function("ratios", |b| b.iter(|| {
        for note in notes.clone() {
            let _hz = black_box(tuning_ratios.note_to_freq_hz(black_box(note)));
        }
    }));
}

criterion_group!(benches, twelve_tet_tuning);

criterion_main!(benches);

