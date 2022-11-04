// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
use std::fmt::Display;

#[derive(Debug)]
pub struct Error;

pub struct Scale {
    tonic: Note,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        unimplemented!(
            "Construct a new scale with tonic {} and intervals {}",
            tonic,
            intervals
        )
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Result::Ok(Scale { tonic: Note::from(tonic) })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let display_type = self.tonic.display_type();
        let notes = Note::as_array();
        let start_pos = notes.iter().position(|n| *n == self.tonic).unwrap();
        let mut scale: Vec<String> = Vec::new();
        for index in 0..13 {
            let mut note_index = start_pos + index;
            if note_index >= notes.len() {
                note_index = note_index - notes.len()
            }
            scale.push(notes[note_index].as_str(display_type).to_string());
        }
        scale
    }
}

#[derive(Clone, Copy)]
enum NoteDisplayType {
    Sharp,
    Flat
}

#[derive(Debug, PartialEq, Eq)]
enum Note {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp
}

impl Note {
    fn as_array() -> [Note; 12] {
        [
            Note::A,
            Note::ASharp,
            Note::B,
            Note::C,
            Note::CSharp,
            Note::D,
            Note::DSharp,
            Note::E,
            Note::F,
            Note::FSharp,
            Note::G,
            Note::GSharp
        ]
    }

    fn as_str(&self, disp: NoteDisplayType) -> &str {
        match self {
            Note::A => "A",
            Note::ASharp => match disp {
                NoteDisplayType::Sharp => "A#",
                NoteDisplayType::Flat => "Bb",
            },
            Note::B => "B",
            Note::C => "C",
            Note::CSharp => match disp {
                NoteDisplayType::Sharp => "C#",
                NoteDisplayType::Flat => "Db",
            },
            Note::D => "D",
            Note::DSharp => match disp {
                NoteDisplayType::Sharp => "D#",
                NoteDisplayType::Flat => "Eb",
            },
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => match disp {
                NoteDisplayType::Sharp => "F#",
                NoteDisplayType::Flat => "Gb",
            },
            Note::G => "G",
            Note::GSharp  => match disp {
                NoteDisplayType::Sharp => "G#",
                NoteDisplayType::Flat => "Ab",
            },
        }
    }

    fn display_type(&self) -> NoteDisplayType {
        match self {
            Note::C => NoteDisplayType::Sharp,
            Note::F => NoteDisplayType::Flat, 
            _ => unimplemented!("Unknown note to display type association: {:?}", self)
        }
    } 
}

impl From<&str> for Note {
    fn from(note: &str) -> Self {
        match note {
            "A" => Note::A,
            "B" => Note::B,
            "C" => Note::C,
            "D" => Note::D,
            "E" => Note::E,
            "F" => Note::F,
            "G" => Note::G,
            _ => unimplemented!("Unknown note: {:?}", note)
        }
    }
}