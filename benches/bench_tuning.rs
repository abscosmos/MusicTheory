use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use music_theory::note::Note;
use music_theory::tuning::{RatioBasedTuning, Tuning, TwelveToneEqualTemperament};

fn twelve_tet_tuning(c: &mut Criterion) {
    let mut g = c.benchmark_group("12TET, note to freq");
    g.sample_size(300);

    let notes = (u8::MIN..=u8::MAX).map(Note::from_midi).collect::<Vec<_>>();

    let tuning_eq_temp = TwelveToneEqualTemperament::A4_440;
    let tuning_ratios = RatioBasedTuning::from_twelve_tet(tuning_eq_temp);

    g.bench_function("original", |b| b.iter(|| {
        for note in &notes {
            let _hz = black_box(tuning_eq_temp.note_to_freq_hz(black_box(*note)));
        }
    }));

    g.bench_function("ratios", |b| b.iter(|| {
        for note in &notes {
            let _hz = black_box(tuning_ratios.note_to_freq_hz(black_box(*note)));
        }
    }));

    drop(g);

    let freq_original = notes.iter().map(|&n| tuning_eq_temp.note_to_freq_hz(n).expect("valid notes")).collect::<Vec<_>>();
    let freq_ratios = notes.iter().map(|&n| tuning_ratios.note_to_freq_hz(n).expect("valid notes")).collect::<Vec<_>>();

    let mut g = c.benchmark_group("12TET, freq to note");
    g.sample_size(300);

    g.bench_function("original", |b| b.iter(|| {
        for freq in &freq_original {
            let _ret = black_box(tuning_eq_temp.freq_to_note(black_box(*freq)));
        }
    }));

    g.bench_function("ratios", |b| b.iter(|| {
        for freq in &freq_ratios {
            let _ret = black_box(tuning_eq_temp.freq_to_note(black_box(*freq)));
        }
    }));
}

criterion_group!(benches, twelve_tet_tuning);

criterion_main!(benches);

