use std::path::Path;
use musicxml::datatypes::{AccidentalValue, ClefSign, Fifths, Id, IdRef, Mode, Octave, PositiveDivisions, Semitones, StaffLinePosition, Step, Token};
use musicxml::elements::{AudibleType, MeasureElement, NormalInfo, Note, NoteContents, NoteType, PartName, Pitch, PitchContents, ScorePart, ScorePartAttributes, ScorePartContents, ScorePartwise, ScorePartwiseContents, Octave as OctaveEl, Step as StepEl, Key as KeyEl, Duration, Accidental, Rest, RestContents, Measure, MeasureAttributes, Part, PartAttributes, PartElement, Attributes, AttributesContents, KeyContents, ExplicitKeyContents, Fifths as FifthsCt, Mode as ModeEl, Clef, ClefContents, Sign, Line, PartList, PartListContents, PartListElement, Divisions, Alter};
use crate::accidental::AccidentalSign;
use crate::containers::{AccidentalDisplay, ContainerElement, Freeform, Offset};
use crate::letter::Letter;
use crate::duration::{Duration as MtDuration};
use crate::scales::heptatonic::DiatonicMode;

#[derive(Debug, thiserror::Error)]
pub enum FreeformToMxlError {
    #[error("No key was found")]
    NoKey,
    #[error("No clef was found")]
    NoClef,
    #[error("Freeform contains {0}, which cannot be represented by musicxml or isn't implemented by converter")]
    UnrepresentableAccidental(AccidentalSign),
    #[error("Score has {divisions} divisions, which cannot represent a duration of {duration:?}")]
    IndivisibleDuration {
        divisions: u32,
        duration: MtDuration,
    }
}

pub fn save_mxl(score: &ScorePartwise, path: impl AsRef<Path>) -> Result<(), String> {
    let path = path.as_ref();
    
    let compressed = match path.extension() {
        Some(ext) if ext == "mxl" => true,
        Some(ext) if ext == "musicxml" => false,
        _ => return Err("Unrecognized file extension".to_owned()),
    };

    musicxml::write_partwise_score(&path.to_string_lossy(), score, compressed, false)
}

pub fn export_to_musicxml(freeform: &Freeform) -> Result<ScorePartwise, FreeformToMxlError> {
    // TODO: handle durations that aren't powers of two
    let max_denom = freeform.elements()
        .iter()
        .flat_map(|(offset, el)| [*offset, el.duration().unwrap_or(MtDuration::ZERO)])
        .map(|dur| *dur.ratio().denom())
        .max()
        .unwrap_or(1);
    
    let divisions = (max_denom / 4).max(1);
    
    let start = freeform.elements()
        .iter()
        .filter_map(|(o, el)| (*o == Offset::ZERO).then_some(el));
    
    // TODO: ensure invariant is upheld of no two clefs/keys at same offset
    
    let clef = *start.clone()
        .filter_map(|el| match el {
            ContainerElement::Clef(clef) => Some(clef),
            _ => None,
        })
        .next()
        .ok_or(FreeformToMxlError::NoKey)?;

    let key = *start.clone()
        .filter_map(|el| match el {
            ContainerElement::KeySignature(key) => Some(key),
            _ => None,
        })
        .next()
        .ok_or(FreeformToMxlError::NoKey)?;
    
    let mut measures = vec![(Offset::ZERO, Some(clef), (key, true), Vec::new())];
    
    for (offset, el) in freeform.elements() {
        match el {
            ContainerElement::Note { note, duration, accidental } => {
                let note = Note {
                    attributes: Default::default(),
                    content: NoteContents {
                        info: NoteType::Normal(NormalInfo {
                            chord: None,
                            audible: AudibleType::Pitch(Pitch {
                                attributes: (),
                                content: PitchContents {
                                    step: StepEl {
                                        attributes: (),
                                        content: match note.letter() {
                                            Letter::C => Step::C,
                                            Letter::D => Step::D,
                                            Letter::E => Step::E,
                                            Letter::F => Step::F,
                                            Letter::G => Step::G,
                                            Letter::A => Step::A,
                                            Letter::B => Step::B,
                                        }
                                    },
                                    alter: Some(Alter {
                                        attributes: (),
                                        content:
                                        Semitones(note.accidental().offset),
                                    }),
                                    octave: OctaveEl {
                                        attributes: (),
                                        content: Octave(note.octave.clamp(u8::MIN as _, u8::MAX as _) as _),
                                    },
                                },
                            }),
                            duration: duration_to_musicxml(*duration, divisions)?,
                            tie: vec![],
                        }),
                        instrument: vec![],
                        footnote: None,
                        level: None,
                        voice: None,
                        r#type: None,
                        dot: vec![],
                        accidental: match note.accidental() {
                            acc if acc == measures.last()
                                .expect("shouldn't be empty")
                                .2.0
                                .accidental_of(note.letter())
                                && *accidental != AccidentalDisplay::Courtesy
                            => None,
                            acc => Some(Accidental {
                                attributes: Default::default(),
                                content: match acc {
                                    AccidentalSign::NATURAL => AccidentalValue::Natural,
                                    AccidentalSign::SHARP => AccidentalValue::Sharp,
                                    AccidentalSign::DOUBLE_SHARP => AccidentalValue::DoubleSharp,
                                    AccidentalSign::FLAT => AccidentalValue::Flat,
                                    AccidentalSign::DOUBLE_FLAT => AccidentalValue::FlatFlat,
                                    _ => return Err(FreeformToMxlError::UnrepresentableAccidental(acc))
                                },
                            }),
                        },
                        time_modification: None,
                        stem: None,
                        notehead: None,
                        notehead_text: None,
                        staff: None,
                        beam: vec![],
                        notations: vec![],
                        lyric: vec![],
                        play: None,
                        listen: None,
                    },
                };

                measures.last_mut()
                    .expect("shouldn't be empty")
                    .3
                    .push(MeasureElement::Note(note));
            }
            ContainerElement::Rest { duration, .. } => measures.last_mut()
                .expect("shouldn't be empty")
                .3
                .push(MeasureElement::Note(
                    Note {
                        attributes: Default::default(),
                        content: NoteContents {
                            info: NoteType::Normal(NormalInfo {
                                chord: None,
                                audible: AudibleType::Rest(Rest {
                                    attributes: Default::default(),
                                    content: RestContents {
                                        display_step: None,
                                        display_octave: None,
                                    },
                                }),
                                duration: duration_to_musicxml(*duration, divisions)?,
                                tie: vec![],
                            }),
                            instrument: vec![],
                            footnote: None,
                            level: None,
                            voice: None,
                            r#type: None,
                            dot: vec![],
                            accidental: None,
                            time_modification: None,
                            stem: None,
                            notehead: None,
                            notehead_text: None,
                            staff: None,
                            beam: vec![],
                            notations: vec![],
                            lyric: vec![],
                            play: None,
                            listen: None,
                        },
                    }
                )),
            ContainerElement::KeySignature(key) => {
                let last = measures.last().expect("shouldn't be empty");
                
                if *offset != last.0 {
                    // create new measure
                    measures.push((*offset, None, (*key, true), Vec::new()));
                } else {
                    measures.last_mut()
                        .expect("shouldn't be empty")
                        .2 = (*key, true);
                }
            }
            ContainerElement::Clef(clef) => {
                let last = measures.last().expect("shouldn't be empty");

                if *offset != last.0 {
                    // create new measure
                    let key = last.2.0;
                    measures.push((*offset, Some(*clef), (key, false), Vec::new()));
                } else {
                    measures.last_mut()
                        .expect("shouldn't be empty")
                        .1 = Some(*clef);
                }
            }
        }
    }
    
    let mut part_content = Vec::new();
    
    for (i, (_, clef, (key, include_key), mut measure_elements)) in measures.into_iter().enumerate() {
        let attr = MeasureElement::Attributes(Attributes {
            attributes: (),
            content: AttributesContents {
                footnote: None,
                level: None,
                divisions: Some(Divisions {
                    attributes: (),
                    content: PositiveDivisions(divisions)
                }),
                key: if include_key {
                    vec![KeyEl {
                        attributes: Default::default(),
                        content: KeyContents::Explicit(ExplicitKeyContents {
                            cancel: None,
                            fifths: FifthsCt {
                                attributes: (),
                                content: Fifths(key.sharps() as _),
                            },
                            mode: Some(ModeEl {
                                attributes: (),
                                content: match key.mode {
                                    DiatonicMode::MAJOR => Mode::Major,
                                    DiatonicMode::Dorian => Mode::Dorian,
                                    DiatonicMode::Phrygian => Mode::Phrygian,
                                    DiatonicMode::Lydian => Mode::Lydian,
                                    DiatonicMode::Mixolydian => Mode::Mixolydian,
                                    DiatonicMode::NATURAL_MINOR => Mode::Minor,
                                    DiatonicMode::Locrian => Mode::Locrian,
                                },
                            }),
                            key_octave: vec![],
                        }),
                    }]
                } else {
                    vec![]
                },
                time: vec![],
                staves: None,
                part_symbol: None,
                instruments: None,
                clef: match clef {
                    None => vec![],
                    Some(clef) => vec![Clef {
                        attributes: Default::default(),
                        content: ClefContents {
                            sign: Sign {
                                attributes: (),
                                content: match clef.anchor().letter {
                                    Letter::C => ClefSign::C,
                                    Letter::F => ClefSign::F,
                                    Letter::G => ClefSign::G,
                                    _ => unreachable!("clef sign can only be in [C, F, G]")
                                }
                            },
                            line: Some(Line {
                                attributes: (),
                                content: StaffLinePosition(clef.staff_line().get() as _),
                            }),
                            clef_octave_change: None, // FIXME: clef octave change
                        },
                    }]
                },
                staff_details: vec![],
                transpose: vec![],
                for_part: vec![],
                directive: vec![],
                measure_style: vec![],
            },
        });

        if include_key || clef.is_some() {
            measure_elements.insert(0, attr);
        }

        let measure = Measure {
            attributes: MeasureAttributes {
                number: Token((i + 1).to_string()),
                id: None,
                implicit: None,
                non_controlling: None,
                text: None,
                width: None,
            },
            content: measure_elements,
        };
        
        part_content.push(PartElement::Measure(measure));
    }

    let part_id = "P1";
    
    Ok(ScorePartwise {
        attributes: Default::default(),
        content: ScorePartwiseContents {
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credit: vec![],
            part_list: PartList {
                attributes: (),
                content: PartListContents {
                    content: vec![PartListElement::ScorePart(
                        ScorePart {
                            attributes: ScorePartAttributes { id: Id(part_id.to_owned()) },
                            content: ScorePartContents {
                                identification: None,
                                part_link: vec![],
                                part_name: PartName {
                                    attributes: Default::default(),
                                    content: "Freeform Part".to_owned(),
                                },
                                part_name_display: None,
                                part_abbreviation: None,
                                part_abbreviation_display: None,
                                group: vec![],
                                score_instrument: vec![],
                                player: vec![],
                                midi_device: vec![],
                                midi_instrument: vec![],
                            },
                        }
                    )],
                },
            },
            part: vec![Part {
                attributes: PartAttributes { id: IdRef(part_id.to_owned()) },
                content: part_content,
            }],
        },
    })
}

// TODO: this should decompose duration
fn duration_to_musicxml(duration: MtDuration, divisions: u32) -> Result<Duration, FreeformToMxlError> {
    Ok(Duration {
        attributes: (),
        content: PositiveDivisions({
            let divs = duration.ratio() * divisions * 4;

            if !divs.is_integer() {
                return Err(FreeformToMxlError::IndivisibleDuration { divisions, duration });
            }

            *divs.numer()
        }),
    })
}