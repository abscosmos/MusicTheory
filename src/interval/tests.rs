use super::*;
use crate::pitch::{Letter, AccidentalSign};
use std::num::NonZeroU16;

use Interval as I;
use IntervalQuality as IQ;
use IntervalNumber as IN;

const FOUR: NonZeroU16 = NonZeroU16::new(4).expect("nonzero");
const SIX: NonZeroU16 = NonZeroU16::new(6).expect("nonzero");

// helper fns
fn semi(ivl: I) -> i16 {
    ivl.semitones().0
}

fn ivl(q: IQ, sz: i16) -> I {
    I::new(q, IN::new(sz).expect("nonzero")).expect("valid interval")
}

#[test]
fn new() {
    for num in 1..25 {
        let num = IN::new(num).expect("nonzero");

        assert!(I::new(IQ::DIMINISHED, num).is_some());
        assert!(I::new(IQ::AUGMENTED, num).is_some());

        for adj in 1..12 {
            assert!(I::new(IQ::Diminished(NonZeroU16::new(adj).expect("nonzero")), num).is_some());
            assert!(I::new(IQ::Augmented(NonZeroU16::new(adj).expect("nonzero")), num).is_some());
        }
    }

    assert!(I::new(IQ::Major, IN::THIRD).is_some());
    assert!(I::new(IQ::Major, IN::THIRTEENTH).is_some());
    assert!(I::new(IQ::Major, IN::SECOND).is_some());

    assert!(I::new(IQ::Major, IN::FOURTH).is_none());
    assert!(I::new(IQ::Major, IN::TWELFTH).is_none());
    assert!(I::new(IQ::Major, IN::OCTAVE).is_none());

    assert!(I::new(IQ::Minor, IN::SIXTH).is_some());
    assert!(I::new(IQ::Minor, IN::NINTH).is_some());

    assert!(I::new(IQ::Minor, IN::ELEVENTH).is_none());
    assert!(I::new(IQ::Minor, IN::UNISON).is_none());

    assert!(I::new(IQ::Perfect, IN::FOURTH).is_some());
    assert!(I::new(IQ::Perfect, IN::FIFTEENTH).is_some());

    assert!(I::new(IQ::Perfect, IN::SECOND).is_none());
    assert!(I::new(IQ::Perfect, IN::SEVENTH).is_none());
}

#[test]
fn from_str() {
    assert_eq!("P1".parse(), Ok(I::PERFECT_UNISON));
    assert_eq!("-M7".parse(), Ok(-I::MAJOR_SEVENTH));
    assert_eq!("m-13".parse(), Ok(-I::MINOR_THIRTEENTH));
    assert_eq!("A6".parse(), Ok(I::AUGMENTED_SIXTH));
    assert_eq!("d15".parse(), Ok(I::DIMINISHED_FIFTEENTH));

    assert_eq!("dddd-5".parse(), Ok(I::new(IQ::Diminished(FOUR), -IN::FIFTH).expect("valid interval")));
    assert_eq!("-AAAAAA2".parse(), Ok(I::new(IQ::Augmented(SIX), -IN::SECOND).expect("valid interval")));

    assert_eq!("1P".parse(), Ok(I::PERFECT_UNISON));
    assert_eq!("-7M".parse(), Ok(-I::MAJOR_SEVENTH));
    assert_eq!("-13m".parse(), Ok(-I::MINOR_THIRTEENTH));
    assert_eq!("A6".parse(), Ok(I::AUGMENTED_SIXTH));
    assert_eq!("d15".parse(), Ok(I::DIMINISHED_FIFTEENTH));

    assert_eq!("-5dddd".parse(), Ok(I::new(IQ::Diminished(FOUR), -IN::FIFTH).expect("valid interval")));
    assert_eq!("-2AAAAAA".parse(), Ok(I::new(IQ::Augmented(SIX), -IN::SECOND).expect("valid interval")));

    assert_eq!("".parse::<I>(), Err(ParseIntervalError::InvalidFormat));
    assert_eq!("P3".parse::<I>(), Err(ParseIntervalError::InvalidInterval));
    assert_eq!("q3".parse::<I>(), Err(ParseIntervalError::QualityErr(ParseIntervalQualityErr)));
    assert!(matches!("m0".parse::<I>(), Err(ParseIntervalError::NumberErr(..))));
}

#[test] // TODO: make tests better, test descending intervals
fn subzero() {
    assert!(I::strict_non_subzero(IQ::DIMINISHED, IN::UNISON).is_none());
    assert!(I::new(IQ::DIMINISHED, IN::UNISON).expect("valid interval").inverted().inverted_strict_non_subzero().is_none());

    for num in 2..15 {
        let num = IN::new(num).expect("nonzero");
        assert!(I::strict_non_subzero(IQ::DIMINISHED, num).is_some());
        assert!(I::strict_non_subzero(IQ::DIMINISHED, num).expect("non subzero").inverted_strict_non_subzero().is_some());
    }

    let doubly_diminished = IQ::Diminished(NonZeroU16::new(2).expect("nonzero"));

    assert!(I::strict_non_subzero(doubly_diminished, IN::UNISON).is_none());
    assert!(I::new(doubly_diminished, IN::UNISON).expect("valid interval").inverted().inverted_strict_non_subzero().is_none());
    assert!(I::strict_non_subzero(doubly_diminished, IN::SECOND).is_none());
    assert!(I::new(doubly_diminished, IN::SECOND).expect("valid interval").inverted().inverted_strict_non_subzero().is_none());

    for num in 3..15 {
        let num = IN::new(num).expect("nonzero");
        assert!(I::strict_non_subzero(doubly_diminished, num).is_some());
        assert!(I::strict_non_subzero(doubly_diminished, num).expect("non subzero").inverted_strict_non_subzero().is_some());
    }

    assert!(I::new(IQ::DIMINISHED, IN::UNISON).expect("valid quality").is_subzero());
    assert!(!I::new(IQ::DIMINISHED, IN::SECOND).expect("valid quality").is_subzero());
}

#[test]
fn semitones_constants() {
    assert_eq!(semi(I::PERFECT_UNISON), 0);
    assert_eq!(semi(I::DIMINISHED_SECOND), 0);

    assert_eq!(semi(I::MINOR_SECOND), 1);
    assert_eq!(semi(I::AUGMENTED_UNISON), 1);

    assert_eq!(semi(I::MAJOR_SECOND), 2);
    assert_eq!(semi(I::DIMINISHED_THIRD), 2);

    assert_eq!(semi(I::MINOR_THIRD), 3);
    assert_eq!(semi(I::AUGMENTED_SECOND), 3);

    assert_eq!(semi(I::MAJOR_THIRD), 4);
    assert_eq!(semi(I::DIMINISHED_FOURTH), 4);

    assert_eq!(semi(I::PERFECT_FOURTH), 5);
    assert_eq!(semi(I::AUGMENTED_THIRD), 5);

    assert_eq!(semi(I::DIMINISHED_FIFTH), 6);
    assert_eq!(semi(I::AUGMENTED_FOURTH), 6);

    assert_eq!(semi(I::PERFECT_FIFTH), 7);
    assert_eq!(semi(I::DIMINISHED_SIXTH), 7);

    assert_eq!(semi(I::MINOR_SIXTH), 8);
    assert_eq!(semi(I::AUGMENTED_FIFTH), 8);

    assert_eq!(semi(I::MAJOR_SIXTH), 9);
    assert_eq!(semi(I::DIMINISHED_SEVENTH), 9);

    assert_eq!(semi(I::MINOR_SEVENTH), 10);
    assert_eq!(semi(I::AUGMENTED_SIXTH), 10);

    assert_eq!(semi(I::MAJOR_SEVENTH), 11);
    assert_eq!(semi(I::DIMINISHED_OCTAVE), 11);

    assert_eq!(semi(I::PERFECT_OCTAVE), 12);
    assert_eq!(semi(I::AUGMENTED_SEVENTH), 12);
    assert_eq!(semi(I::DIMINISHED_NINTH), 12);

    assert_eq!(semi(I::MINOR_NINTH), 13);
    assert_eq!(semi(I::AUGMENTED_OCTAVE), 13);

    assert_eq!(semi(I::MAJOR_NINTH), 14);
    assert_eq!(semi(I::DIMINISHED_TENTH), 14);

    assert_eq!(semi(I::MINOR_TENTH), 15);
    assert_eq!(semi(I::AUGMENTED_NINTH), 15);

    assert_eq!(semi(I::MAJOR_TENTH), 16);
    assert_eq!(semi(I::DIMINISHED_ELEVENTH), 16);

    assert_eq!(semi(I::PERFECT_ELEVENTH), 17);
    assert_eq!(semi(I::AUGMENTED_TENTH), 17);

    assert_eq!(semi(I::DIMINISHED_TWELFTH), 18);
    assert_eq!(semi(I::AUGMENTED_ELEVENTH), 18);

    assert_eq!(semi(I::PERFECT_TWELFTH), 19);
    assert_eq!(semi(I::DIMINISHED_THIRTEENTH), 19);

    assert_eq!(semi(I::MINOR_THIRTEENTH), 20);
    assert_eq!(semi(I::AUGMENTED_TWELFTH), 20);

    assert_eq!(semi(I::MAJOR_THIRTEENTH), 21);
    assert_eq!(semi(I::DIMINISHED_FOURTEENTH), 21);

    assert_eq!(semi(I::MINOR_FOURTEENTH), 22);
    assert_eq!(semi(I::AUGMENTED_THIRTEENTH), 22);

    assert_eq!(semi(I::MAJOR_FOURTEENTH), 23);
    assert_eq!(semi(I::DIMINISHED_FIFTEENTH), 23);

    assert_eq!(semi(I::PERFECT_FIFTEENTH), 24);
    assert_eq!(semi(I::AUGMENTED_FOURTEENTH), 24);
}

#[test]
fn semitones_negative() {
    assert_eq!(semi(-I::PERFECT_UNISON), 0);
    assert_eq!(semi(-I::DIMINISHED_SECOND), 0);

    assert_eq!(semi(-I::MINOR_SECOND), -1);
    assert_eq!(semi(-I::AUGMENTED_UNISON), -1);

    assert_eq!(semi(-I::MAJOR_SEVENTH), -11);
    assert_eq!(semi(-I::DIMINISHED_OCTAVE), -11);

    assert_eq!(semi(-I::PERFECT_OCTAVE), -12);
    assert_eq!(semi(-I::AUGMENTED_SEVENTH), -12);
    assert_eq!(semi(-I::DIMINISHED_NINTH), -12);

    assert_eq!(semi(-I::MAJOR_FOURTEENTH), -23);
    assert_eq!(semi(-I::DIMINISHED_FIFTEENTH), -23);

    assert_eq!(semi(-I::PERFECT_FIFTEENTH), -24);
    assert_eq!(semi(-I::AUGMENTED_FOURTEENTH), -24);
}

#[test]
fn semitones_aug_dim() {
    fn dim(adj: u16, sz: i16) -> I {
        I::new(
            IQ::Diminished(NonZeroU16::new(adj).expect("nonzero")),
            IN::new(sz).expect("nonzero")
        ).expect("valid interval")
    }

    fn aug(adj: u16, sz: i16) -> I {
        I::new(
            IQ::Augmented(NonZeroU16::new(adj).expect("nonzero")),
            IN::new(sz).expect("nonzero")
        ).expect("valid interval")
    }

    assert_eq!(semi(dim(7, -80)), -128);
    assert_eq!(semi(dim(6, 4)), -1); // subzero
    assert_eq!(semi(dim(5, -45)), -70);
    assert_eq!(semi(dim(4, 30)), 45);
    assert_eq!(semi(dim(3, -75)), -124);
    assert_eq!(semi(dim(2, 6)), 6);

    assert_eq!(semi(aug(2, -38)), -66);
    assert_eq!(semi(aug(3, -11)), -20);
    assert_eq!(semi(aug(4, 59)), 104);
    assert_eq!(semi(aug(5, 25)), 46);
    assert_eq!(semi(aug(6, -53)), -95);
    assert_eq!(semi(aug(7, 34)), 64);
}

#[test]
fn semitones_general() {
    assert_eq!(semi(ivl(IQ::Perfect, -39)), -65);
    assert_eq!(semi(ivl(IQ::Major, 31)), 52);
    assert_eq!(semi(ivl(IQ::Minor, -76)), -128);
    assert_eq!(semi(ivl(IQ::Perfect, 40)), 67);
    assert_eq!(semi(ivl(IQ::Major, 17)), 28);
    assert_eq!(semi(ivl(IQ::Minor, -77)), -130);
    assert_eq!(semi(ivl(IQ::Perfect, -19)), -31);
    assert_eq!(semi(ivl(IQ::Major, 48)), 81);
    assert_eq!(semi(ivl(IQ::Minor, 21)), 34);
}

#[test]
fn from_semitones() {
    assert_eq!(
        (0..=12)
            .map(|s| Interval::from_semitones_preferred(Semitones(s)))
            .collect::<Vec<_>>(),
        [
            I::PERFECT_UNISON, I::MINOR_SECOND, I::MAJOR_SECOND,
            I::MINOR_THIRD, I::MAJOR_THIRD, I::PERFECT_FOURTH,
            I::DIMINISHED_FIFTH, I::PERFECT_FIFTH, I::MINOR_SIXTH,
            I::MAJOR_SIXTH, I::MINOR_SEVENTH, I::MAJOR_SEVENTH,
            I::PERFECT_OCTAVE
        ]
    );

    assert_eq!(Interval::from_semitones_preferred(Semitones(76)), ivl(IQ::Major, 45));
    assert_eq!(Interval::from_semitones_preferred(Semitones(21)), ivl(IQ::Major, 13));
    assert_eq!(Interval::from_semitones_preferred(Semitones(-31)), ivl(IQ::Perfect, -19));
    assert_eq!(Interval::from_semitones_preferred(Semitones(58)), ivl(IQ::Minor, 35));
    assert_eq!(Interval::from_semitones_preferred(Semitones(14)), ivl(IQ::Major, 9));
    assert_eq!(Interval::from_semitones_preferred(Semitones(-27)), ivl(IQ::Minor, -17));
    assert_eq!(Interval::from_semitones_preferred(Semitones(-17)), ivl(IQ::Perfect, -11));
    assert_eq!(Interval::from_semitones_preferred(Semitones(16)), ivl(IQ::Major, 10));
    assert_eq!(Interval::from_semitones_preferred(Semitones(-66)), ivl(IQ::DIMINISHED, -40));
    assert_eq!(Interval::from_semitones_preferred(Semitones(72)), ivl(IQ::Perfect, 43));
}

#[test]
fn to_from_semitones_inverse() {
    for semis in -75..75 {
        assert_eq!(semi(I::from_semitones_preferred(Semitones(semis))), semis);
    }
}

#[test]
fn shorthand_display() {
    assert_eq!(I::PERFECT_FIFTEENTH.shorthand(), "P15");
    assert_eq!(I::PERFECT_FIFTEENTH.to_string(), "P15");
}

#[test]
fn inverted() {
    assert_eq!(I::PERFECT_UNISON.inverted(), I::PERFECT_UNISON);
    assert_eq!(I::DIMINISHED_SECOND.inverted(), I::AUGMENTED_SEVENTH);
    assert_eq!(-I::MINOR_THIRD.inverted(), -I::MAJOR_SIXTH);
    assert_eq!(I::DIMINISHED_FOURTH.inverted(), I::AUGMENTED_FIFTH);
    assert_eq!(-I::PERFECT_FIFTH.inverted(), -I::PERFECT_FOURTH);
    assert_eq!(I::AUGMENTED_SIXTH.inverted(), I::DIMINISHED_THIRD);
    assert_eq!(I::MAJOR_SEVENTH.inverted(), I::MINOR_SECOND);
    assert_eq!(-I::DIMINISHED_OCTAVE.inverted(), -I::AUGMENTED_OCTAVE);
    assert_eq!(I::PERFECT_OCTAVE.inverted(), I::PERFECT_OCTAVE);

    assert_eq!(-I::MINOR_TENTH.inverted(), -I::MAJOR_THIRTEENTH);
    assert_eq!(I::AUGMENTED_ELEVENTH.inverted(), I::DIMINISHED_TWELFTH);
    assert_eq!(I::PERFECT_TWELFTH.inverted(), I::PERFECT_ELEVENTH);
    assert_eq!(-I::DIMINISHED_THIRTEENTH.inverted(), -I::AUGMENTED_TENTH);
    assert_eq!(-I::MAJOR_FOURTEENTH.inverted(), -I::MINOR_NINTH);
    assert_eq!(I::PERFECT_FIFTEENTH.inverted(), I::PERFECT_FIFTEENTH);

    assert_eq!(ivl(IQ::Perfect, -39).inverted(), ivl(IQ::Perfect, -40));
    assert_eq!(ivl(IQ::Major, 31).inverted(), ivl(IQ::Minor, 34));
    assert_eq!(ivl(IQ::Minor, -76).inverted(), ivl(IQ::Major, -73));
    assert_eq!(ivl(IQ::Perfect, 40).inverted(), ivl(IQ::Perfect, 39));
    assert_eq!(ivl(IQ::Major, 17).inverted(), ivl(IQ::Minor, 20));
    assert_eq!(ivl(IQ::Minor, -77).inverted(), ivl(IQ::Major, -72));
    assert_eq!(ivl(IQ::Perfect, -19).inverted(), ivl(IQ::Perfect, -18));
    assert_eq!(ivl(IQ::Major, 48).inverted(), ivl(IQ::Minor, 45));
    assert_eq!(ivl(IQ::Minor, 21).inverted(), ivl(IQ::Major, 16));
}

#[test]
fn double_inversion() {
    for &ivl in I::ALL_CONSTS {
        assert_eq!(ivl.inverted().inverted(), ivl);
    }
}

#[test]
fn direction() {
    assert!(I::MAJOR_NINTH.is_ascending());
    assert!(I::DIMINISHED_TWELFTH.is_ascending());

    assert!(!(-I::MINOR_SEVENTH).is_ascending());
    assert!(!(-I::AUGMENTED_FOURTEENTH).is_ascending());

    assert_eq!(I::MINOR_SEVENTH.with_direction(true), I::MINOR_SEVENTH);
    assert_eq!(I::MAJOR_SECOND.with_direction(false), -I::MAJOR_SECOND);

    assert_eq!((-I::AUGMENTED_FIFTH).with_direction(true), I::AUGMENTED_FIFTH);
    assert_eq!((-I::PERFECT_ELEVENTH).with_direction(false), -I::PERFECT_ELEVENTH);
}

#[test]
fn eq_ord_enharmonic() {
    assert!(I::MAJOR_SIXTH.eq_enharmonic(&I::DIMINISHED_SEVENTH));
    assert!(I::AUGMENTED_THIRTEENTH.eq_enharmonic(&I::MINOR_FOURTEENTH));

    assert!(!I::MINOR_THIRD.eq_enharmonic(&I::DIMINISHED_FOURTH));
    assert!(!I::PERFECT_TWELFTH.eq_enharmonic(&I::AUGMENTED_TWELFTH));

    assert_eq!(I::AUGMENTED_FOURTH.cmp_enharmonic(&I::PERFECT_FIFTH), Ordering::Less);
    assert_eq!(I::MAJOR_NINTH.cmp_enharmonic(&I::DIMINISHED_TENTH), Ordering::Equal);
    assert_eq!(I::PERFECT_FIFTEENTH.cmp_enharmonic(&I::DIMINISHED_FIFTEENTH), Ordering::Greater);
}

#[test]
fn add_subtract() {
    use IntervalQuality as IQ;
    use IntervalNumber as IN;

    let mut qualities = vec![IQ::Perfect, IQ::Major, IQ::Minor];
    qualities.extend((1..=4).map(|n| IQ::Diminished(NonZeroU16::new(n).expect("nonzero"))));
    qualities.extend((1..=4).map(|n| IQ::Augmented(NonZeroU16::new(n).expect("nonzero"))));

    let mut numbers = Vec::with_capacity(100);
    numbers.extend((1..=24).map(|n| IN::new(n).expect("nonzero")));
    numbers.extend((-24..=-1).map(|n| IN::new(n).expect("nonzero")));

    let intervals = qualities.iter()
        .flat_map(|iq|
            numbers.iter().filter_map(
                |num| I::new(*iq, *num)
            ))
        .collect::<Vec<_>>();

    for &lhs in &intervals {
        for &rhs in &intervals {
            let add = lhs.add(rhs);
            let sub = lhs.sub(rhs);

            assert_eq!(Note::MIDDLE_C.transpose(lhs).transpose(rhs), Note::MIDDLE_C.transpose(add), "lhs: {lhs}, rhs: {rhs} add: {add}");
            assert_eq!(Note::MIDDLE_C.transpose(lhs).transpose(-rhs), Note::MIDDLE_C.transpose(sub), "lhs: {lhs}, rhs: {rhs} add: {sub}");
        }
    }
}

#[test]
fn neg() {
    assert_eq!((-I::DIMINISHED_FOURTEENTH).shorthand(), "d-14");
    assert_eq!(-(-I::MAJOR_SEVENTH), I::MAJOR_SEVENTH);
}

#[test]
fn test_aug_seventh() {
    let between = Interval::between_pitches(Pitch::C, Pitch::B_SHARP);

    assert_eq!(Pitch::C.transpose(between), Pitch::B_SHARP, "{between}");

    let between = Interval::between_pitches(Pitch::G, Pitch::F_DOUBLE_SHARP);

    assert_eq!(Pitch::G.transpose(between), Pitch::F_DOUBLE_SHARP, "{between}");

    let between = Interval::between_pitches(Pitch::G_DOUBLE_FLAT, Pitch::F);

    assert_eq!(Pitch::G_DOUBLE_FLAT.transpose(between), Pitch::F, "{between}");

    let g_quadruple_flat = Pitch::from_letter_and_accidental(Letter::G, AccidentalSign { offset: -4 });

    let between = Interval::between_pitches(g_quadruple_flat, Pitch::F_DOUBLE_FLAT);

    assert_eq!(Pitch::G_DOUBLE_FLAT.transpose(between), Pitch::F, "{between}");
}

#[test]
fn between_pitches_transpose_inverses() {
    for &ivl in &Interval::ALL_CONSTS[..23] {
        for start in Pitch::ALL_CONSTS {
            let end = start.transpose(ivl);

            assert_eq!(
                start.semitones_to(end), ivl.semitones(),
                "{start} -> {end} should span {} semitones", ivl.semitones().0
            );

            let between = Interval::between_pitches(*start, end);

            assert_eq!(
                between, ivl,
                "between_pitches returns {between} instead of applied {ivl}, ({start} -> {end})"
            );

            let neg_between = Interval::between_pitches(end, *start);

            let inv = ivl.inverted().expand_subzero();

            assert_eq!(
                neg_between, inv,
                "neg_between_pitches returns {neg_between} instead of applied {inv}, ({end} -> {start})"
            );
        }
    }
}

#[test]
fn between_notes_transpose_inverses() {
    for &ivl in Interval::ALL_CONSTS {
        for pitch_start in Pitch::ALL_CONSTS {
            for octave in -3..=3 {
                let start = Note { pitch: *pitch_start, octave };

                let end = start.transpose(ivl);

                assert_eq!(
                    start.semitones_to(end), ivl.semitones(),
                    "{start} -> {end} should span {} semitones", ivl.semitones().0
                );

                let between = Interval::between_notes(start, end);

                assert_eq!(
                    between, ivl,
                    "between_notes returns {between} instead of applied {ivl}, ({start} -> {end})"
                );

                // descending

                let descending_ivl = if ivl == I::PERFECT_UNISON { ivl } else { -ivl };

                let end = start.transpose(descending_ivl);

                let between = Interval::between_notes(start, end);

                assert_eq!(
                    between, descending_ivl,
                    "between_notes returns {between} instead of applied {descending_ivl}, ({start} -> {end})"
                );
            }
        }
    }
}

#[test]
fn test_stability() {
    assert_eq!(
        Interval::PERFECT_FOURTH.stability(), None,
        "P5 can be either consonant or dissonant depending on context",
    );

    assert_eq!(
        Interval::AUGMENTED_FOURTH.stability(), Some(Stability::Dissonance),
        "augmented intervals are dissonant",
    );

    assert_eq!(
        Interval::MAJOR_SECOND.stability(), Some(Stability::Dissonance),
        "seconds are dissonant",
    );

    assert_eq!(
        Interval::MAJOR_SIXTH.stability(), Some(Stability::ImperfectConsonance),
        "sixths are imperfect consonances",
    );

    assert_eq!(
        Interval::PERFECT_FIFTH.stability(), Some(Stability::PerfectConsonance),
        "fifths are perfect consonances",
    );

    assert_eq!(
        (Interval::MAJOR_THIRD + Interval::PERFECT_OCTAVE).stability(), Some(Stability::ImperfectConsonance),
        "should be able to check stability of compound intervals",
    );

    for ivl in Interval::ALL_CONSTS {
        // ensure all cases are covered, else would panic on unreachable
        let _ = ivl.stability();
    }

    assert!(
        Stability::PerfectConsonance.is_consonant() && Stability::ImperfectConsonance.is_consonant(),
        "consonances should be consonant",
    );
}