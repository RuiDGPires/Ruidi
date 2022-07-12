mod test;
mod util;
pub mod stream;

pub struct Note {
    vel: u32,
    note: u32,
    duration: u32,
}

impl Note {
    pub fn new(vel: u32, note: u32, duration: u32) -> Note { 
        Note {vel: vel, note: note, duration: duration} 
    }

    pub fn pause(duration: u32) -> Note {
        Note {vel: 0, note: 0, duration: duration}
    }
}

pub struct Voice{
    notes: Vec<Note>
}

impl Voice {
    pub fn new() -> Voice {
        Voice { notes: Vec::new() }
    }

    pub fn add_note(&mut self, note: Note) -> &mut Voice {
        self.notes.push(note);
        self
    }
}

pub struct MidiObj{
    pub voices: Vec<Voice>
}

impl MidiObj {
    pub fn new() -> MidiObj {
        MidiObj { voices: Vec::new() }
    }

    pub fn new_sized(size: usize) -> MidiObj {
        let mut obj = MidiObj::new();

        for _ in 0 .. size {
            obj.add_voice();
        }
         
       obj 
    }

    pub fn add_voice(&mut self) -> &mut MidiObj {
        self.voices.push(Voice::new()); 
        self
    }

    pub fn add_note(&mut self, voice: usize, note: Note) -> &mut MidiObj {
        self.voices[voice].add_note(note);
        self
    }
}

fn check_str<T: stream::InStream<u8>>(stream: &mut T, string: &str) -> bool {
    for c in string.chars() {
        if stream.read() != Some(&(c as u8)) {
            return false;
        }
    }
    true
}

impl stream::Sourceable<u8> for MidiObj {
    fn from_file<T: stream::InStream<u8>>(mut stream: T) -> Result<Box<Self>, String> {
        if check_str(&mut stream, "MThd") {
            Ok(Box::new(Self::new()))
        } else {
            Err(String::from("Invalid Midi file"))
        }
    }
}
