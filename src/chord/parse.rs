use std::slice;
use crate::accidental::AccidentalSign;
use crate::interval::Interval;
use crate::interval::number::IntervalNumber;
use crate::interval::quality::IntervalQuality;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum ChordTk {
    Maj, // maj, M
    Min, // min, -
    Dim, // dim, °
    Aug, // aug, +, -> (#5)
    Sus2, // sus2
    Sus4, // sus4, sus
    Five, // 5
    Six, // 6 & m6
    Seven, // 7
    Nine,
    Eleven,
    Thirteen,
    Add(u8),
    Omit(u8), // (omit5)
    Alt(AccidentalSign, u8),
}

macro_rules! ensure {
    { $cond:expr } => {
        if !$cond {
            return None;
        }
    };
}

fn upper_chord_ext(cursor: &mut TkCursor, seventh_quality: IntervalQuality, intervals: &mut Vec<Interval>) -> Option<()> {
    use ChordTk as T;
    use Interval as I;

    let curr = cursor.curr()?;

    if !matches!(curr, T::Seven | T::Nine | T::Eleven | T::Thirteen) {
        return None;
    }

    assert!(
        matches!(
            seventh_quality,
            IntervalQuality::Minor | IntervalQuality::Major | IntervalQuality::DIMINISHED
        ),
        "invalid quality for 7th degree"
    );

    let seventh = I::new(seventh_quality, IntervalNumber::SEVENTH)
        .expect("assert above should ensure that this seventh must be valid");

    const NINTH: Interval = I::MAJOR_NINTH;
    const ELEVENTH: Interval = I::PERFECT_ELEVENTH;
    const THIRTEENTH: Interval = I::MAJOR_THIRTEENTH;

    match curr {
        ChordTk::Seven => intervals.push(seventh),
        ChordTk::Nine => intervals.extend([seventh, NINTH]),
        ChordTk::Eleven => intervals.extend([seventh, NINTH, ELEVENTH]),
        ChordTk::Thirteen => intervals.extend([seventh, NINTH, ELEVENTH, THIRTEENTH]),
        _ => return None,
    }

    cursor.consume(1);

    Some(())
}

fn interpret(tokens: &[ChordTk]) -> Option<Vec<Interval>> {
    use ChordTk as T;
    use Interval as I;

    if tokens.is_empty() {
        return Some(vec![I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH]);
    }

    let mut intervals = Vec::new(); // TODO: guess capacity
    let mut cursor = TkCursor::new(tokens);

    while let Some(tk) = cursor.curr() {
        match tk {
            T::Min => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH]);

                cursor.consume(1);

                if let Some(T::Maj) = cursor.curr() {
                    cursor.consume(1);

                    upper_chord_ext(&mut cursor, IntervalQuality::Major, &mut intervals)?; // ? since the next token MUST be a number
                } else if let Some(T::Six) = cursor.curr() {
                    intervals.push(I::MAJOR_SIXTH);
                    cursor.consume(1);
                } else {
                    // ignore option result since next token doesn't NEED to be a number
                    let _consumed_number = upper_chord_ext(&mut cursor, IntervalQuality::Minor, &mut intervals).is_some();
                }
            }
            T::Maj => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH]);

                cursor.consume(1);

                if let Some(T::Six) = cursor.curr() {
                    intervals.push(I::MAJOR_SIXTH);
                    cursor.consume(1);
                } else {
                    // ignore option result since next token doesn't NEED to be a number
                    let _consumed_number = upper_chord_ext(&mut cursor, IntervalQuality::Minor, &mut intervals).is_some();
                }
            }
            T::Dim => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH]);

                cursor.consume(1);

                let _consumed_number = upper_chord_ext(&mut cursor, IntervalQuality::DIMINISHED, &mut intervals).is_some();
            }
            T::Aug => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH]);

                cursor.consume(1);

                if let Some(T::Maj) = cursor.curr() {
                    cursor.consume(1);

                    upper_chord_ext(&mut cursor, IntervalQuality::Major, &mut intervals)?; // ? since the next token MUST be a number
                } else {
                    // ignore option result since next token doesn't NEED to be a number
                    let _consumed_number = upper_chord_ext(&mut cursor, IntervalQuality::Minor, &mut intervals).is_some();
                }
            }
            T::Sus2 => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::MAJOR_SECOND, I::PERFECT_FIFTH]);

                cursor.consume(1);
            }
            T::Sus4 => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::PERFECT_FOURTH, I::PERFECT_FIFTH]);

                cursor.consume(1);
            }
            T::Five => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::PERFECT_FIFTH]);

                cursor.consume(1);
            }
            T::Six => {
                ensure! { cursor.consumed() == 0 }

                intervals.extend([I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH]);

                cursor.consume(1);
            }
            T::Seven | T:: Nine | T::Eleven | T::Thirteen => {
                intervals.extend([I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH]);
                
                upper_chord_ext(&mut cursor, IntervalQuality::Minor, &mut intervals)
                    .expect("token must be number due to match")
            }
            _ => todo!()
        }
    }

    Some(intervals)
}

struct TkCursor<'a> { // TODO: maybe this can just be replaced with a peekable iterator?
    consumed: usize,
    curr: Option<ChordTk>,
    tks: slice::Iter<'a, ChordTk>,
}

impl<'a> TkCursor<'a> {
    pub fn new(input: &'a [ChordTk]) -> Self {
        let mut tks = input.iter();

        let curr = tks.next().copied();

        Self { consumed: 0, curr, tks }
    }

    pub fn curr(&self) -> Option<ChordTk> {
        self.curr
    }

    pub fn peek(&self, n: usize) -> Option<ChordTk> {
        self.tks
            .clone()
            .nth(n)
            .copied()
    }

    pub fn consume(&mut self, n: usize) {
        self.consumed += n.min(self.tks.clone().len() + 1);

        self.curr = self.tks
            .nth(n.checked_sub(1).expect("to consume 1 token, use 1 not 0"))
            .copied()
    }

    pub fn consumed(&self) -> usize {
        self.consumed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ChordTk as T;
    use Interval as I;

    macro_rules! test_interpret {
        ( $($name:expr => $tks:expr),*; $ivls:expr) => {
            let intervals = ( $ivls ).split(',')
                .map(|s| s.trim().parse::<I>().expect("valid intervals"))
                .collect::<Vec<_>>();

            $(
                assert_eq!(
                    interpret(&( $tks )).as_ref(),
                    Some(&intervals),
                    concat!("failed to interpret '", $name, "' (", stringify!( $tks ), ")")
                );
            )*
        };
    }

    #[test]
    fn cursor() {
        use ChordTk as T;

        let tks = [T::Min, T::Seven, T::Nine, T::Eleven, T::Thirteen];

        let mut cursor = TkCursor::new(&tks);

        assert_eq!(cursor.curr(), Some(T::Min));

        assert_eq!(cursor.peek(0), Some(T::Seven));
        assert_eq!(cursor.peek(1), Some(T::Nine));
        assert_eq!(cursor.peek(2), Some(T::Eleven));

        cursor.consume(1);

        assert_eq!(cursor.curr(), Some(T::Seven));
        assert_eq!(cursor.peek(2), Some(T::Thirteen));
        assert_eq!(cursor.peek(3), None);

        cursor.consume(2);

        assert_eq!(cursor.curr(), Some(T::Eleven));

        assert_eq!(cursor.consumed(), 3);

        cursor.consume(2);

        assert_eq!(cursor.curr(), None);

        assert_eq!(cursor.consumed(), 5);
    }

    #[test]
    fn interpret_successful_no_special() {
        test_interpret!("C" => [], "Cmaj" => [T::Maj]; "P1, M3, P5");

        test_interpret!("Cmin / Cm" => [T::Min]; "P1, m3, P5");

        test_interpret!("Caug / C+" => [T::Aug]; "P1, M3, A5");

        test_interpret!("Cdim" => [T::Dim]; "P1, m3, d5");

        test_interpret!("Csus2" => [T::Sus2]; "P1, M2, P5");

        test_interpret!("Csus4" => [T::Sus4]; "P1, P4, P5");

        test_interpret!("C5" => [T::Five]; "P1, P5");

        test_interpret!("C6" => [T::Six], "Cmaj6" => [T::Maj, T::Six]; "P1, M3, P5, M6");

        test_interpret!("Cmin6" => [T::Min, T::Six]; "P1, m3, P5, M6");

        test_interpret!("C7" => [T::Seven]; "P1, M3, P5, m7");

        test_interpret!("Cdim7" => [T::Dim, T::Seven]; "P1, m3, d5, d7");

        test_interpret!("Cmin7" => [T::Min, T::Seven]; "P1, m3, P5, m7");

        test_interpret!("Cmin(maj7)" => [T::Min, T::Maj, T::Seven]; "P1, m3, P5, M7");

        test_interpret!("Cmaj7" => [T::Maj, T::Seven]; "P1, M3, P5, M7");

        test_interpret!("Caug7" => [T::Aug, T::Seven]; "P1, M3, a5, m7");

        test_interpret!("Caug(maj7)" => [T::Aug, T::Maj, T::Seven]; "P1, M3, a5, M7");

        test_interpret!("C9" => [T::Nine]; "P1, M3, P5, m7, M9");

        test_interpret!("Cmin9" => [T::Nine]; "P1, m3, P5, m7, M9");

        test_interpret!("C11" => [T::Eleven]; "P1, M3, P5, m7, M9, P11");

        test_interpret!("C13" => [T::Eleven]; "P1, M3, P5, m7, M9, P11, M13");
    }
}