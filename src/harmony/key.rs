use crate::interval::Interval;
use crate::pitch::{Pitch, Letter, AccidentalSign, Spelling};
use crate::harmony::mode::DiatonicMode;
use crate::harmony::ScaleDegree;
use crate::scales::definition::heptatonic::{DiatonicMode as DiatonicModeExperimental, DiatonicScale};
use crate::scales::rooted::RootedSizedScale;
use crate::scales::ScaleMode as _;
use crate::scales::sized_scale::SizedScale as _;

/// A musical key with a tonic pitch and mode.
///
/// A `Key` is built on a [tonic](Self::tonic), and can have any of the seven [diatonic modes](DiatonicMode).
///
/// For convenience, `Key` provides constructors for major and minor keys using
/// [`Key::major`] and [`Key::minor`].
///
/// # Examples
///
/// You can create a key from a tonic and mode:
/// ```
/// # use music_theory::prelude::*;
/// let d_minor = Key::new(Pitch::D, DiatonicMode::NATURAL_MINOR);
/// ```
///
/// Or use the convenience constructors:
/// ```
/// # use music_theory::prelude::*;
/// let g_major = Key::major(Pitch::G);
/// let e_minor = Key::minor(Pitch::E);
/// ```
///
/// You can create keys from their key signature:
/// ```
/// # use music_theory::prelude::*;
/// // D major has 2 sharps
/// let d_major = Key::from_sharps(2, DiatonicMode::MAJOR);
/// assert_eq!(d_major.tonic, Pitch::D);
/// ```
///
/// # Key Signatures
///
/// Keys can tell you their key signature (number of sharps or flats):
/// ```
/// # use music_theory::prelude::*;
/// // G major has 1 sharp (F#)
/// assert_eq!(Key::major(Pitch::G).sharps(), 1);
///
/// // F major has 1 flat (Bb)
/// assert_eq!(Key::major(Pitch::F).sharps(), -1);
///
/// // C major has no sharps or flats
/// assert_eq!(Key::major(Pitch::C).sharps(), 0);
/// ```
///
/// # Relative and Parallel Keys
///
/// Relative keys share the same key signature:
/// ```
/// # use music_theory::prelude::*;
/// let c_major = Key::major(Pitch::C);
/// let a_minor = c_major.relative(DiatonicMode::NATURAL_MINOR);
///
/// assert_eq!(a_minor, Key::minor(Pitch::A));
/// assert_eq!(c_major.sharps(), a_minor.sharps());
/// ```
///
/// Parallel keys share the same tonic:
/// ```
/// # use music_theory::prelude::*;
/// let c_major = Key::major(Pitch::C);
/// let c_minor = c_major.parallel(DiatonicMode::NATURAL_MINOR);
///
/// assert_eq!(c_minor, Key::minor(Pitch::C));
/// assert_eq!(c_major.tonic, c_minor.tonic);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key {
    /// The tonic (root) pitch of the key.
    pub tonic: Pitch,
    /// The mode of the key (major, minor, dorian, etc.).
    pub mode: DiatonicMode,
}

impl Key {
    /// Creates a new key from a tonic pitch and mode.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let d_dorian = Key::new(Pitch::D, DiatonicMode::Dorian);
    /// ```
    pub fn new(tonic: Pitch, mode: DiatonicMode) -> Self {
        Self { tonic, mode }
    }

    /// Creates a major key with the given tonic.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let g_major = Key::major(Pitch::G);
    /// assert_eq!(g_major.sharps(), 1);
    /// ```
    pub fn major(tonic: Pitch) -> Self {
        Self::new(tonic, DiatonicMode::MAJOR)
    }

    /// Creates a natural minor key with the given tonic.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let a_minor = Key::minor(Pitch::F);
    /// assert_eq!(a_minor.sharps(), -4);
    /// ```
    pub fn minor(tonic: Pitch) -> Self {
        Self::new(tonic, DiatonicMode::NATURAL_MINOR)
    }

    /// Returns a new key with the same mode but a different tonic.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let c_major = Key::major(Pitch::C);
    /// let d_major = c_major.with_tonic(Pitch::D);
    ///
    /// assert_eq!(d_major, Key::major(Pitch::D));
    /// ```
    pub fn with_tonic(self, tonic: Pitch) -> Self {
        Self { tonic, .. self }
    }

    /// Creates a key from a key signature (number of sharps or flats) and mode.
    ///
    /// Positive values represent sharps, and negative values represent flats.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // D major has 2 sharps (F# and C#)
    /// assert_eq!(
    ///     Key::from_sharps(2, DiatonicMode::MAJOR),
    ///     Key::major(Pitch::D)
    /// );
    ///
    /// // C minor has 3 flats (Bb, Eb, and Ab)
    /// assert_eq!(
    ///     Key::from_sharps(-3, DiatonicMode::NATURAL_MINOR),
    ///     Key::minor(Pitch::C)
    /// );
    ///
    /// // Works with all diatonic modes
    /// assert_eq!(
    ///     Key::from_sharps(1, DiatonicMode::Dorian),
    ///     Key::new(Pitch::A, DiatonicMode::Dorian)
    /// );
    /// ```
    pub fn from_sharps(sharps: i16, mode: DiatonicMode) -> Self {
        let offset = Letter::from_step(mode as u8 - 1)
            .expect("mode is in [1, 7], so subtracting 1 should be in range")
            .fifths_from_c();
        
        Self::new(Pitch::from_fifths_from_c(sharps + offset), mode)
    }

    /// Returns the key signature as the number of sharps (positive) or flats (negative).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // G major has 1 sharp
    /// assert_eq!(Key::major(Pitch::G).sharps(), 1);
    ///
    /// // F major has 1 flat
    /// assert_eq!(Key::major(Pitch::F).sharps(), -1);
    ///
    /// // C major has no sharps or flats
    /// assert_eq!(Key::major(Pitch::C).sharps(), 0);
    ///
    /// // F phrygian has 5 flats
    /// assert_eq!(Key::new(Pitch::F, DiatonicMode::Phrygian).sharps(), -5);
    /// ```
    pub fn sharps(self) -> i16 {
        let offset = Letter::from_step(self.mode as u8 - 1)
            .expect("mode is in [1, 7], so subtracting 1 should be in range")
            .fifths_from_c();
        
        self.tonic.as_fifths_from_c() - offset
    }

    /// Returns the spelling preference (sharps or flats) for this key.
    ///
    /// In other words, does this key use sharps or flats? Since keys like C major use neither
    /// sharps nor flats, this method can return `None`.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// assert_eq!(Key::major(Pitch::G).spelling(), Some(Spelling::Sharps));
    /// assert_eq!(Key::major(Pitch::F).spelling(), Some(Spelling::Flats));
    /// assert_eq!(Key::minor(Pitch::A).spelling(), None);
    /// ```
    pub fn spelling(self) -> Option<Spelling> {
        match self.sharps() {
            ..0 => Some(Spelling::Flats),
            0 => None,
            1.. => Some(Spelling::Sharps),
        }
    }
    
    /// Attempts to create a key from a key signature and tonic pitch.
    ///
    /// Returns `None` if the tonic is not diatonic to any mode with the given key signature.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // D *major* has 2 sharps
    /// assert_eq!(
    ///     Key::try_from_sharps_tonic(2, Pitch::D),
    ///     Some(Key::major(Pitch::D))
    /// );
    ///
    /// // Bb *mixolydian* has 3 flats
    /// assert_eq!(
    ///     Key::try_from_sharps_tonic(-3, Pitch::B_FLAT),
    ///     Some(Key::new(Pitch::B_FLAT, DiatonicMode::Mixolydian))
    /// );
    ///
    /// // C is not diatonic to any mode with 2 sharps
    /// assert_eq!(
    ///     Key::try_from_sharps_tonic(2, Pitch::C),
    ///     None
    /// );
    /// ```
    pub fn try_from_sharps_tonic(sharps: i16, tonic: Pitch) -> Option<Self> {
        let major_tonic = Pitch::from_fifths_from_c(sharps);

        let pos = DiatonicScale::new(DiatonicMode::MAJOR.as_experimental())
            .build_from(major_tonic)
            .iter()
            .position(|p| *p == tonic)?;

        let mode = DiatonicModeExperimental::from_num((pos + 1) as _)
            .expect("should be within [1,7]");
        
        Some(Self::new(tonic, DiatonicMode::from_experimental(mode)))
    }

    /// Returns the parallel key in the specified mode.
    ///
    /// Parallel keys share the same tonic but have different modes.
    /// For example, E major and E minor are parallel keys.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// use DiatonicMode as Mode;
    ///
    /// // Get the parallel minor
    /// assert_eq!(
    ///     Key::major(Pitch::E).parallel(Mode::NATURAL_MINOR),
    ///     Key::minor(Pitch::E),
    /// );
    /// // Works with all diatonic modes
    /// assert_eq!(
    ///     Key::new(Pitch::D, Mode::Mixolydian).parallel(Mode::Lydian),
    ///     Key::new(Pitch::D, Mode::Lydian),
    /// );
    /// ```
    pub fn parallel(self, mode: DiatonicMode) -> Self {
        Self { mode, .. self }
    }

    /// Returns the relative key in the specified mode.
    ///
    /// Relative keys share the same key signature (same number of sharps or flats) but have
    /// different tonics. For example, A is the relative minor of C major.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// use DiatonicMode as Mode;
    ///
    /// // Get the relative minor
    /// // (same key signature / number of sharps, different tonic)
    /// assert_eq!(
    ///     Key::major(Pitch::C).relative(Mode::NATURAL_MINOR),
    ///     Key::minor(Pitch::A),
    /// );
    ///
    /// // Works with all diatonic modes
    /// let d_major = Key::major(Pitch::D);
    /// let g_lydian = Key::new(Pitch::G, Mode::Lydian);
    /// assert_eq!(d_major.relative(Mode::Lydian), g_lydian);
    /// assert_eq!(d_major.sharps(), g_lydian.sharps());
    /// ```
    pub fn relative(self, mode: DiatonicMode) -> Self {
        // what key has all white keys in this mode?
        let source_ref = Letter::from_step(self.mode as u8 - 1).expect("mode enum should be same size as letter enum");
        let target_ref = Letter::from_step(mode as u8 - 1).expect("mode enum should be same size as letter enum");

        let diff_fifths = Pitch::from(target_ref).as_fifths_from_c() - Pitch::from(source_ref).as_fifths_from_c();
        
        let new_tonic = self.tonic.transpose_fifths(diff_fifths);
        
        self.with_tonic(new_tonic).parallel(mode)
    }
    
    /// Creates a key from a scale degree, pitch, and mode.
    ///
    /// Given that `pitch` is the specified `degree` in the resulting key,
    /// this function determines what the tonic must be.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // If E is the 3rd degree of a major key, the key is C major
    /// assert_eq!(
    ///     Key::from_pitch_degree(
    ///         ScaleDegree::III,
    ///         Pitch::E,
    ///         DiatonicMode::MAJOR
    ///     ),
    ///     Key::major(Pitch::C)
    /// );
    ///
    /// // If B is the 5th degree of a minor key, the key is E minor
    /// assert_eq!(
    ///     Key::from_pitch_degree(
    ///         ScaleDegree::V,
    ///         Pitch::B,
    ///         DiatonicMode::NATURAL_MINOR
    ///     ),
    ///     Key::minor(Pitch::E)
    /// );
    /// ```
    pub fn from_pitch_degree(degree: ScaleDegree, pitch: Pitch, mode: DiatonicMode) -> Self {
        let offset = degree as u8 - 1;
        
        let letter_step = (pitch.letter().step() + 7 - offset) % 7;
        
        let letter = Letter::from_step(letter_step)
            .expect("must be in range of [0,6]");

        let scale = RootedSizedScale {
            root: Pitch::from(letter),
            scale: DiatonicScale::new(mode.as_experimental())
        };

        let expect = scale.get(degree.as_experimental());
        
        assert_eq!(
            pitch.letter(), expect.letter(),
            "both letters should be the same if we're comparing accidentals"
        );
        
        let offset = pitch.accidental().offset - expect.accidental().offset;
        
        Self {
            tonic: Pitch::from_letter_and_accidental(letter, AccidentalSign { offset }),
            mode,
        }
    }

    /// Transposes the key by the given interval.
    ///
    /// The mode is preserved, and the tonic is transposed.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // Transpose C major up a whole step
    /// assert_eq!(
    ///     Key::major(Pitch::C).transpose(Interval::MAJOR_SECOND),
    ///     Key::major(Pitch::D)
    /// );
    ///
    /// // Transpose G major down a perfect fifth
    /// assert_eq!(
    ///     Key::major(Pitch::G).transpose(-Interval::PERFECT_FIFTH),
    ///     Key::major(Pitch::C)
    /// );
    /// ```
    pub fn transpose(&self, interval: Interval) -> Self {
        self.with_tonic(self.tonic.transpose(interval))
    }

    /// Returns the pitches that are altered (sharped or flattened) in the key signature.
    ///
    /// The returned vector is sorted by the order sharps/flats appear in key signatures.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// // G major has one sharp: F#
    /// assert_eq!(
    ///     Key::major(Pitch::G).alterations(),
    ///     [Pitch::F_SHARP],
    /// );
    ///
    /// // D major has two sharps: F# and C#
    /// assert_eq!(
    ///     Key::major(Pitch::D).alterations(),
    ///     [Pitch::F_SHARP, Pitch::C_SHARP],
    /// );
    ///
    /// // F major has one flat: Bb
    /// assert_eq!(
    ///     Key::major(Pitch::F).alterations(),
    ///     [Pitch::B_FLAT],
    /// );
    ///
    /// // C major has no alterations
    /// assert_eq!(
    ///     Key::major(Pitch::C).alterations(),
    ///     [],
    /// );
    /// ```
    pub fn alterations(&self) -> Vec<Pitch> {
        let mut accidentals = self.scale_experimental()
            .build_default()
            .into_iter()
            .filter(|a| a.accidental() != AccidentalSign::NATURAL)
            .collect::<Vec<_>>();
        
        accidentals.sort_unstable_by_key(|p| p.as_fifths_from_c());

        assert_eq!(
            accidentals.iter().map(|p| p.accidental().offset).sum::<i16>(), self.sharps(),
            "total accidentals should equal sharps of key"
        );
        
        accidentals
    }

    /// Returns the accidental for a given letter in this key's signature.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let g_major = Key::major(Pitch::G);
    ///
    /// // F is sharped in G major
    /// assert_eq!(g_major.accidental_of(Letter::F), AccidentalSign::SHARP);
    ///
    /// // C is natural in G major
    /// assert_eq!(g_major.accidental_of(Letter::C), AccidentalSign::NATURAL);
    /// ```
    pub fn accidental_of(&self, letter: Letter) -> AccidentalSign {
        let degree = ScaleDegree::from_num(self.tonic.letter().offset_between(letter) + 1)
            .expect("offset should be in range");

        let pitch = self.scale_experimental().get(degree.as_experimental());

        assert_eq!(
            pitch.letter(), letter,
            "should have gotten the correct letter"
        );
        
        pitch.accidental()
    }

    /// Returns the pitch at the given scale degree in this key.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::prelude::*;
    /// let g_major = Key::major(Pitch::G);
    ///
    /// // The tonic (1st degree) is G
    /// assert_eq!(g_major.relative_pitch(ScaleDegree::I), Pitch::G);
    ///
    /// // The 3rd degree is B
    /// assert_eq!(g_major.relative_pitch(ScaleDegree::III), Pitch::B);
    ///
    /// // The 7th degree is F#
    /// assert_eq!(g_major.relative_pitch(ScaleDegree::VII), Pitch::F_SHARP);
    /// ```
    pub fn relative_pitch(self, degree: ScaleDegree) -> Pitch {
        self.scale_experimental().get(degree.as_experimental())
    }
    
    // this is pub(crate) since it's reliant on 'experimental-scales', so it shouldn't be public
    // if the feature is enabled, the 'scale' method should be used instead 
    pub(crate) fn scale_experimental(&self) -> RootedSizedScale<Pitch, 7, DiatonicScale> {
        RootedSizedScale {
            root: self.tonic,
            scale: DiatonicScale::new(self.mode.as_experimental()),
        }
    }

    /// Returns the diatonic scale for this key.
    ///
    /// This method is only available when the `experimental-scales` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "experimental-scales")]
    /// # {
    /// # use music_theory::prelude::*;
    /// let g_major = Key::major(Pitch::G);
    /// let scale = g_major.scale();
    ///
    /// // Build the scale and check its notes
    /// let notes = scale.build_default();
    /// assert_eq!(notes[0], Pitch::G);
    /// assert_eq!(notes[1], Pitch::A);
    /// assert_eq!(notes[6], Pitch::F_SHARP);
    /// # }
    /// ```
    #[cfg(feature = "experimental-scales")]
    pub fn scale(&self) -> RootedSizedScale<Pitch, 7, DiatonicScale> {
        self.scale_experimental()
    }

    /// Returns the seven diatonic scales built from each degree of this key.
    ///
    /// Each scale uses the corresponding modal rotation, so the first scale uses
    /// the same mode as the key, the second uses the next mode, and so on.
    ///
    /// This method is only available when the `experimental-scales` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "experimental-scales")]
    /// # {
    /// # use music_theory::prelude::*;
    /// # use music_theory::scales::definition::heptatonic::DiatonicMode;
    /// let c_major = Key::major(Pitch::C);
    /// let chord_scales = c_major.chord_scales();
    ///
    /// // First chord scale is rooted on C in Ionian (major) mode
    /// let c_maj = &chord_scales[0];
    /// assert_eq!(c_maj.root, Pitch::C);
    /// // assert_eq!(c_maj.scale.mode, DiatonicMode::Ionian);
    ///
    /// // Second chord scale is rooted on D in Dorian mode
    /// let d_dorian = &chord_scales[1];
    /// assert_eq!(d_dorian.root, Pitch::D);
    /// // assert_eq!(d_dorian.scale.mode, DiatonicMode::Dorian);
    ///
    /// // Seventh chord scale is rooted on B in Locrian mode
    /// let b_locrian = &chord_scales[6];
    /// assert_eq!(b_locrian.root, Pitch::B);
    /// // assert_eq!(b_locrian.scale.mode, DiatonicMode::Locrian);
    /// # }
    /// ```
    #[cfg(feature = "experimental-scales")]
    pub fn chord_scales(&self) -> [RootedSizedScale<Pitch, 7, DiatonicScale>; 7] {
        use std::array;

        let scale = self.scale_experimental().build_default();
        
        array::from_fn(|i| {
            let mode = DiatonicModeExperimental::from_num((self.mode.as_experimental().as_num() - 1 + i as u8) % 7 + 1)
                .expect("should be in [1, 7]");
            
            let root = *scale.get(i).expect("scale and ret array should be the same size");
            
            RootedSizedScale { root, scale: DiatonicScale::new(mode) }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::harmony::{DiatonicMode, Key};

    #[test]
    fn relative_parallel() {
        let modes = (1..8).map(|n|DiatonicMode::from_repr(n).expect("in range"));

        for sharps in -10..=10 {
            for mode in modes.clone() {
                let key = Key::from_sharps(sharps, mode);

                for mode in modes.clone() {
                    let relative = key.relative(mode);

                    assert_eq!(
                        relative.mode, mode,
                        "mode should match requested relative",
                    );

                    assert_eq!(
                        relative.sharps(), key.sharps(),
                        "relative mode should have same amount of sharps/flats",
                    );

                    let parallel = key.parallel(mode);

                    assert_eq!(
                        parallel.mode, mode,
                        "mode should match requested parallel",
                    );

                    assert_eq!(
                        parallel.tonic, key.tonic,
                        "parallel key should have same tonic",
                    );
                }
            }
        }
    }
}