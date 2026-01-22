use crate::harmony::{DiatonicMode, Key};
use crate::Note;

use super::roman_chord::RomanChord;
use super::{Voice, Voicing};

const BASE_WIDTH: usize = 160;
const WIDTH_PER_CHORD: usize = 60;
const MAX_WIDTH: usize = 1200;
const HEIGHT: usize = 295;
const STAVE_X: usize = 25;
const TREBLE_Y: usize = 15;
const BASS_Y: usize = 115;
const SYMBOL_Y: usize = 240;

/// Displays a chord progression with optional voicings in evcxr/Jupyter.
///
/// Renders the progression as SATB (four-part harmony) notation using VexFlow,
/// with key signature, chord symbols, and quarter notes (or rests if voicing is None).
///
/// # Arguments
///
/// - `progression` - The chord progression as Roman numerals
/// - `key` - The musical key for the progression
/// - `voicings` - Optional slice of voicings; if None or shorter than progression,
///   quarter rests are displayed for missing voicings
/// ```
pub fn display_voicings(progression: &[RomanChord], key: Key, voicings: Option<&[Voicing]>) {
    let html = render_html(progression, key, voicings);
    println!("EVCXR_BEGIN_CONTENT text/html\n{html}\nEVCXR_END_CONTENT");
}

pub fn render_html(progression: &[RomanChord], key: Key, voicings: Option<&[Voicing]>) -> String {
    let id = generate_unique_id();
    let width = (BASE_WIDTH + progression.len() * WIDTH_PER_CHORD).min(MAX_WIDTH);
    let stave_width = width - STAVE_X - 5;
    let key_sig = key_to_vexflow(key);
    let chords_js = generate_chords_js(progression, key, voicings);

    format!(
        r##"<div id="{id}"></div>
<script src="https://cdn.jsdelivr.net/npm/vexflow@4.2.5/build/cjs/vexflow.js"></script>
<script>
(function() {{
    const {{ Renderer, Stave, StaveConnector, Voice, Formatter, StaveNote, Accidental }} = Vex.Flow;

    const div = document.getElementById("{id}");
    const renderer = new Renderer(div, Renderer.Backends.SVG);
    renderer.resize({width}, {HEIGHT});
    const context = renderer.getContext();

    const chords = {chords_js};
    const numChords = chords.length;
    const staveWidth = {stave_width};

    // Create and draw staves
    const trebleStave = new Stave({STAVE_X}, {TREBLE_Y}, staveWidth);
    trebleStave.addClef("treble").addKeySignature("{key_sig}");
    trebleStave.setContext(context).draw();

    const bassStave = new Stave({STAVE_X}, {BASS_Y}, staveWidth);
    bassStave.addClef("bass").addKeySignature("{key_sig}");
    bassStave.setContext(context).draw();

    // Connect staves
    [StaveConnector.type.BRACE, StaveConnector.type.SINGLE_LEFT, StaveConnector.type.SINGLE_RIGHT]
        .forEach(type => {{
            const connector = new StaveConnector(trebleStave, bassStave);
            connector.setType(type);
            connector.setContext(context).draw();
        }});

    // Build note arrays for each voice
    const notes = {{ soprano: [], alto: [], tenor: [], bass: [] }};

    chords.forEach(chord => {{
        if (chord.isRest) {{
            notes.soprano.push(new StaveNote({{ keys: ["b/4"], duration: "qr", clef: "treble" }}));
            notes.alto.push(new StaveNote({{ keys: ["b/4"], duration: "qr", clef: "treble" }}));
            notes.tenor.push(new StaveNote({{ keys: ["d/3"], duration: "qr", clef: "bass" }}));
            notes.bass.push(new StaveNote({{ keys: ["d/3"], duration: "qr", clef: "bass" }}));
        }} else {{
            const createNote = (key, acc, clef, stemDir) => {{
                const note = new StaveNote({{ keys: [key], duration: "q", stem_direction: stemDir, clef }});
                if (acc) note.addModifier(new Accidental(acc));
                return note;
            }};

            notes.soprano.push(createNote(chord.soprano, chord.sopranoAcc, "treble", 1));
            notes.alto.push(createNote(chord.alto, chord.altoAcc, "treble", -1));
            notes.tenor.push(createNote(chord.tenor, chord.tenorAcc, "bass", 1));
            notes.bass.push(createNote(chord.bass, chord.bassAcc, "bass", -1));
        }}
    }});

    // Create and format voices
    const createVoice = noteArray => new Voice({{ num_beats: numChords, beat_value: 4 }}).addTickables(noteArray);

    const sopranoVoice = createVoice(notes.soprano);
    const altoVoice = createVoice(notes.alto);
    const tenorVoice = createVoice(notes.tenor);
    const bassVoice = createVoice(notes.bass);

    const formatWidth = staveWidth - 100;

    // Use single formatter for all voices to ensure alignment across staves
    const formatter = new Formatter();
    formatter.joinVoices([sopranoVoice, altoVoice]);
    formatter.joinVoices([tenorVoice, bassVoice]);
    formatter.format([sopranoVoice, altoVoice, tenorVoice, bassVoice], formatWidth);

    sopranoVoice.draw(context, trebleStave);
    altoVoice.draw(context, trebleStave);
    tenorVoice.draw(context, bassStave);
    bassVoice.draw(context, bassStave);

    // Draw chord symbols centered under each note
    context.setFont("Times New Roman", 12, "normal");
    context.setFillStyle("#000000");

    notes.bass.forEach((note, idx) => {{
        const x = note.getAbsoluteX();
        const symbol = chords[idx].symbol;
        const textWidth = context.measureText(symbol).width;
        context.fillText(symbol, x - textWidth / 2 + 5, {SYMBOL_Y});
    }});
}})();
</script>"##
    )
}

fn generate_chords_js(progression: &[RomanChord], key: Key, voicings: Option<&[Voicing]>) -> String {
    let chords: Vec<_> = progression
        .iter()
        .enumerate()
        .map(|(i, chord)| chord_to_js(chord, voicings.and_then(|v| v.get(i)), key))
        .collect();

    format!("[{}]", chords.join(", "))
}

fn chord_to_js(chord: &RomanChord, voicing: Option<&Voicing>, key: Key) -> String {
    let symbol = escape_js_string(&chord.to_string());

    match voicing {
        Some(v) => {
            let notes: Vec<_> = [Voice::Soprano, Voice::Alto, Voice::Tenor, Voice::Bass]
                .into_iter()
                .map(|voice| {
                    let note = v[voice];
                    (note_to_vexflow(note), accidental_to_vexflow(note, key))
                })
                .collect();

            format!(
                r#"{{ isRest: false, symbol: "{symbol}", soprano: "{}", alto: "{}", tenor: "{}", bass: "{}", sopranoAcc: {}, altoAcc: {}, tenorAcc: {}, bassAcc: {} }}"#,
                notes[0].0, notes[1].0, notes[2].0, notes[3].0,
                notes[0].1, notes[1].1, notes[2].1, notes[3].1,
            )
        }
        None => format!(r#"{{ isRest: true, symbol: "{symbol}" }}"#),
    }
}

/// Converts a Note to VexFlow key format (e.g., "c/4", "f/5").
fn note_to_vexflow(note: Note) -> String {
    format!(
        "{}/{}",
        note.pitch.letter().to_string().to_lowercase(),
        note.octave
    )
}

/// Converts a Note's accidental to VexFlow format, accounting for key signature.
/// Returns "null" if the accidental matches the key signature.
fn accidental_to_vexflow(note: Note, key: Key) -> String {
    let note_acc = note.pitch.accidental();
    let key_acc = key.accidental_of(note.pitch.letter());

    if note_acc == key_acc {
        return "null".into();
    }

    let acc_str = match note_acc.offset {
        0 => "n",
        1 => "#",
        2 => "##",
        -1 => "b",
        -2 => "bb",
        n if n > 2 => return format!(r#""{}""#, "#".repeat(n as usize)),
        n if n < -2 => return format!(r#""{}""#, "b".repeat((-n) as usize)),
        _ => return "null".into(),
    };

    format!(r#""{acc_str}""#)
}

/// Converts a Key to VexFlow key signature string (uses relative major tonic).
fn key_to_vexflow(key: Key) -> String {
    let tonic = key.relative(DiatonicMode::MAJOR).tonic;
    let letter = tonic.letter().to_string();

    match tonic.accidental().offset {
        0 => letter,
        n if n > 0 => format!("{letter}{}", "#".repeat(n as usize)),
        n => format!("{letter}{}", "b".repeat((-n) as usize)),
    }
}

fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn generate_unique_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    format!("vexflow-debug-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voice_leading::roman_chord::{Quality, ScaleDegree};
    use crate::Pitch;

    #[test]
    fn test_key_to_vexflow() {
        assert_eq!(key_to_vexflow(Key::major(Pitch::C)), "C");
        assert_eq!(key_to_vexflow(Key::major(Pitch::G)), "G");
        assert_eq!(key_to_vexflow(Key::major(Pitch::F)), "F");
        assert_eq!(key_to_vexflow(Key::major(Pitch::D)), "D");
        assert_eq!(key_to_vexflow(Key::major(Pitch::B_FLAT)), "Bb");
        assert_eq!(key_to_vexflow(Key::minor(Pitch::A)), "C");
        assert_eq!(key_to_vexflow(Key::minor(Pitch::E)), "G");
    }

    #[test]
    fn test_accidental_with_key() {
        let g_major = Key::major(Pitch::G);

        assert_eq!(accidental_to_vexflow(Note::new(Pitch::F_SHARP, 4), g_major), "null");

        assert_eq!(accidental_to_vexflow(Note::new(Pitch::F, 4), g_major), "\"n\"");

        assert_eq!(accidental_to_vexflow(Note::new(Pitch::C, 4), g_major), "null");
    }

    #[test]
    fn test_render_html_generates_valid_structure() {
        let progression = [
            RomanChord::triad(ScaleDegree::I, Quality::Major),
            RomanChord::triad(ScaleDegree::V, Quality::Major),
        ];

        let html = render_html(&progression, Key::major(Pitch::C), None);

        assert!(html.contains("vexflow"));
        assert!(html.contains("treble"));
        assert!(html.contains("bass"));
        assert!(html.contains("addKeySignature"));
        assert!(html.contains("isRest: true"));
    }
}
