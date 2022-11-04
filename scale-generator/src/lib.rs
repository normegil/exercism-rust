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
#[derive(Debug)]
pub struct Error;

pub struct Scale {
    tonic: Note,
    display: NoteDisplayType,
    intervals: Vec<Step>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let intervals = Step::from(intervals);
        let (note, display) = Note::from(tonic);
        Result::Ok(Scale { tonic: note, display: display, intervals: intervals })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let chromatic_interval = "mmmmmmmmmmmm";
        Scale::new(tonic, chromatic_interval)
    }

    pub fn enumerate(&self) -> Vec<String> {
        let notes = Note::as_array();
        let mut current_pos = notes.iter().position(|n| *n == self.tonic).unwrap();
        let mut scale: Vec<String> = Vec::new();
        for interval in self.intervals.iter() {
            scale.push(notes[current_pos].as_str(self.display).to_string());
            match interval {
                Step::Major => current_pos += 2,
                Step::Minor => current_pos += 1,
                Step::Harmonic => current_pos += 3,
            }
            if current_pos >= notes.len() {
                current_pos = current_pos - notes.len();
            }
        }
        scale.push(notes[current_pos].as_str(self.display).to_string());
        scale
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoteDisplayType {
    Sharp,
    Flat
}

enum Step {
    Major,
    Minor,
    Harmonic
}

impl Step {
    fn from(intervals: &str) -> Vec<Self> {
        intervals.chars().map(|c| {
            match c {
                'M' => Step::Major,
                'm' => Step::Minor,
                'A' => Step::Harmonic,
                _ => unimplemented!("Unsupported char {}", c),
            }
        }).collect()
    }
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

    fn from(note: &str) -> (Self, NoteDisplayType) {
        match note {
            "A" | "a" => (Note::A, NoteDisplayType::Sharp),
            "bb" => (Note::ASharp, NoteDisplayType::Flat),
            "B" => (Note::B, NoteDisplayType::Sharp),
            "C" => (Note::C, NoteDisplayType::Sharp),
            "Db" => (Note::CSharp, NoteDisplayType::Flat),
            "D" => (Note::D, NoteDisplayType::Sharp),
            "d" => (Note::D, NoteDisplayType::Flat),
            "Eb" => (Note::DSharp, NoteDisplayType::Flat),
            "E" | "e" => (Note::E, NoteDisplayType::Sharp),
            "F" => (Note::F, NoteDisplayType::Flat),
            "f#" => (Note::FSharp, NoteDisplayType::Sharp),
            "G" => (Note::G, NoteDisplayType::Sharp),
            "g" => (Note::G, NoteDisplayType::Flat),
            _ => unimplemented!("Unknown note: {:?}", note)
        }
    }
}