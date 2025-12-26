use crate::note::{Note, NoteLetter};
use std::sync::OnceLock;

const DEFAULT_MAPPING: &str = "qwertyuiop[]asdfghjkl{},";
const LETTERS: [NoteLetter; 12] = [
    NoteLetter::A,
    NoteLetter::B,
    NoteLetter::C,
    NoteLetter::D,
    NoteLetter::E,
    NoteLetter::F,
    NoteLetter::G,
    NoteLetter::H,
    NoteLetter::I,
    NoteLetter::J,
    NoteLetter::K,
    NoteLetter::L,
];

static KEYBOARD_MAPPING: OnceLock<String> = OnceLock::new();

pub fn set_keyboard_mapping(mapping: String) -> Result<(), String> {
    if mapping.len() != 24 {
        return Err(format!("Keyboard mapping must be exactly 24 characters, got {}", mapping.len()));
    }

    if !mapping.is_ascii() {
        return Err("Keyboard mapping must contain only ASCII characters".to_string());
    }

    let mut seen = [false; 256];
    for c in mapping.chars() {
        let idx = c as usize;
        if idx >= 256 || seen[idx] {
            return Err(format!("Keyboard mapping contains duplicate or invalid character: '{}'", c));
        }
        seen[idx] = true;
    }

    KEYBOARD_MAPPING.get_or_init(|| mapping);
    Ok(())
}

pub fn get_mapping() -> &'static str {
    KEYBOARD_MAPPING.get().map(|s| s.as_str()).unwrap_or(DEFAULT_MAPPING)
}

pub fn is_valid_key(c: char) -> bool {
    get_mapping().contains(c)
}

pub fn key_to_note(key: char, octave_offset: i32) -> Option<Note> {
    get_mapping().find(key).map(|idx| idx_to_note(idx, octave_offset))
}

pub fn get_key_at_index(idx: usize) -> char {
    get_mapping().as_bytes()[idx] as char
}

fn idx_to_note(idx: usize, octave_offset: i32) -> Note {
    let base_octave = 1 + (idx / 12) as i32;
    let letter = LETTERS[idx % 12];
    Note::new(letter, base_octave + octave_offset)
}
