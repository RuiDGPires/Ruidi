mod tests;
pub mod io;
pub mod instruments;
pub mod pitch;

use std::collections::HashMap;
use instruments::Instrument;

trait Event {
}

pub struct Note {
    pub vel: u8,
    pub note: u8,
    pub duration: u32,
}

impl Event for Note{}

impl Note {
    // Common note durations
    pub const QUARTER: u32 = 96;
    pub const HALF: u32 = Self::QUARTER*2;
    pub const WHOLE: u32 = Self::HALF*2;
    pub const EIGHT: u32 = Self::QUARTER / 2;

    pub fn new(vel: u8, note: u8, duration: u32) -> Note { 
        Note {vel: vel, note: note, duration: duration} 
    }

    pub fn pause(duration: u32) -> Note {
        Note {vel: 0, note: 0, duration: duration}
    }
}

pub struct Track{
    notes: HashMap<usize, Note>,
    i: usize,
    pub instrument: Instrument,
}

impl Track {
    pub fn new() -> Track {
        Track { notes: HashMap::new(), i: 0, instrument: Instrument::AcousticGrandPiano}
    }

    pub fn add_note(&mut self, note: Note) -> &mut Track {
        self.notes.insert(self.i, note);
        self.i += 1;
        self
    }

    pub fn get_notes(&self) -> &HashMap<usize, Note> {
        &self.notes
    }

    pub fn set_instrument(&mut self, intrument: Instrument) -> &mut Track{
        self.instrument = intrument;
        self
    }
}

pub struct MidiObj{
    pub tracks: Vec<Track>,
    pub tempo: u32,
    pub time_signature: (u8, u8),
}

impl MidiObj {
    pub fn new() -> MidiObj {
        MidiObj { tracks: Vec::new(), tempo: 120, time_signature: (4, 4) }
    }

    pub fn add_track(&mut self, track: Track) -> &mut MidiObj {
        self.tracks.push(track); 
        self
    }

    pub fn set_tempo(&mut self, tempo: u32) -> &mut MidiObj {
        self.tempo = tempo;
        self
    }

    pub fn set_time_signature(&mut self, time_signature: (u8, u8)) -> &mut MidiObj {
        self.time_signature = time_signature;
        self
    }
}
