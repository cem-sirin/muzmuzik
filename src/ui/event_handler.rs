use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

use crate::audio::AudioEngine;

pub fn handle_key_event(engine: &AudioEngine, key_code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind) -> bool {
    match kind {
        KeyEventKind::Press => {
            if key_code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
                return true;
            }
            if let KeyCode::Char(c) = key_code {
                match c {
                    'n' => engine.change_octave(-1),
                    'm' => engine.change_octave(1),
                    _ if crate::piano::is_valid_key(c) => {
                        let octave = engine.get_octave();
                        if let Some(note) = crate::piano::key_to_note(c, octave) {
                            engine.note_on(note);
                        }
                    }
                    _ => {}
                }
            }
        }
        KeyEventKind::Release => {
            if let KeyCode::Char(c) = key_code
                && crate::piano::is_valid_key(c) {
                let octave = engine.get_octave();
                if let Some(note) = crate::piano::key_to_note(c, octave) {
                    engine.note_off(note);
                }
            }
        }
        _ => {}
    }
    false
}
