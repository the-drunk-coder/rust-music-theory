use crate::note::Pitch;
use crate::note::Tuning;
use std::fmt;
use std::fmt::Formatter;

/// A note.
#[derive(Debug, Clone)]
pub struct Note {
    /// The pitch of the note (A, B, C#, etc).
    pub pitch: Pitch,
    /// The octave of the note in standard notation.
    pub octave: u8,
}

impl Note {
    /// Create a new note.
    pub fn new(pitch: Pitch, octave: u8) -> Note {
        Note { pitch, octave }
    }

    pub fn from_string(string: &str) -> Note {
        let mut chars = string.chars();
        let mut pitch = "".to_string();
        let mut oct = "".to_string();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                oct.push(c);
            } else {
                pitch.push(c);
            }
        }
        Note::new(Pitch::from_str(&pitch).unwrap(), oct.parse::<u8>().unwrap())
    }

    pub fn from_note_nr(nr: u8) -> Note {
        println!("nr: {}", nr);
        let pitch_class = Pitch::from_u8(nr % 12);
        let octave = nr / 12;
        Note::new(pitch_class, octave)
    }

    pub fn from_freq(freq: f32, tuning: Tuning) -> Note {
        match tuning {
            Tuning::EqualTemperament => {
                let a440 = Note::new(Pitch::from_str("A").unwrap(), 4).to_note_nr();
                Note::from_note_nr(((12.0 * (freq / 440.0).log2()) as i16 + a440 as i16) as u8)
            }
        }
    }

    pub fn to_note_nr(self) -> u8 {
        self.pitch.into_u8() + 12 * self.octave
    }

    pub fn to_freq(self, tuning: Tuning) -> f32 {
        match tuning {
            Tuning::EqualTemperament => {
                let a440 = Note::new(Pitch::from_str("A").unwrap(), 4).to_note_nr();
                2f32.powf(((self.to_note_nr() as i16 - a440 as i16) as f32) / 12.0) * 440.0
            }
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.pitch)
    }
}

/// A type that can produce a sequence of notes.
pub trait Notes {
    /// Get the sequence of notes.
    fn notes(&self) -> Vec<Note>;

    /// Print the sequence of notes.
    ///
    /// By default this function will print out each notes' index and its pitch class. For example,
    /// printing out C major would look like:
    /// ```text
    /// Notes:
    ///   1: C
    ///   2: E
    ///   3: G
    /// ```
    fn print_notes(&self) {
        let notes = self.notes();

        println!("Notes:");
        for (i, note) in notes.iter().enumerate() {
            println!("  {}: {}", i + 1, note.pitch)
        }
    }
}
