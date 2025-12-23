use std::fmt;

/// Base frequency for `A1` in this notation system.
///
/// This project uses a 12-letter system (A-L) for 12-TET.
pub const A1_FREQ_HZ: f32 = 130.815;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteLetter {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
}

impl NoteLetter {
    pub fn index(self) -> i32 {
        self as u8 as i32
    }

    pub fn as_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::H => 'H',
            Self::I => 'I',
            Self::J => 'J',
            Self::K => 'K',
            Self::L => 'L',
        }
    }
}

impl TryFrom<char> for NoteLetter {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            'G' | 'g' => Self::G,
            'H' | 'h' => Self::H,
            'I' | 'i' => Self::I,
            'J' | 'j' => Self::J,
            'K' | 'k' => Self::K,
            'L' | 'l' => Self::L,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for NoteLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
    pub letter: NoteLetter,
    pub octave: i32,
}

impl Note {
    pub fn new(letter: NoteLetter, octave: i32) -> Self {
        Self { letter, octave }
    }

    /// Semitone offset from A1.
    pub fn semitone_offset_from_a1(self) -> i32 {
        (self.octave - 1) * 12 + self.letter.index()
    }

    /// Frequency in Hz, for 12-TET based on `A1_FREQ_HZ`.
    pub fn frequency_hz(self) -> f32 {
        let semitones = self.semitone_offset_from_a1() as f32;
        A1_FREQ_HZ * (2.0_f32).powf(semitones / 12.0)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.letter, self.octave)
    }
}
