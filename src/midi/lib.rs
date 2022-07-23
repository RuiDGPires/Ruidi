mod tests;
pub mod io;
pub mod instruments;
pub mod pitch;

pub mod dynamics {
    pub const AUTO:               u8 = 0xFF; // TODO

    pub const FORTISSIMO:         u8 = 101;
    pub const FORTE:              u8 = 88;
    pub const MEZZO_FORTE:        u8 = 75;
    pub const MEZZO_PIANO:        u8 = 62;
    pub const PIANO:              u8 = 49;
    pub const PIANISSIMO:         u8 = 36;
}

pub mod durations {
    // Common note durations
    // American names
    pub const QUARTER:   u32 = 96;
    pub const HALF:      u32 = QUARTER*2;
    pub const WHOLE:     u32 = HALF*2;
    pub const EIGHTH:    u32 = QUARTER / 2;
    pub const SIXTEENTH: u32 = QUARTER / 2;

    // British names
    pub const SEMIBREVE:     u32 = WHOLE;
    pub const CROTCHET:      u32 = QUARTER;
    pub const QUAVER:        u32 = EIGHTH;
    pub const SEMI_QUAVER:   u32 = SIXTEENTH;
    pub const MINIM:         u32 = HALF;
}

use std::collections::HashMap;
use instruments::Instrument;

pub struct Note {
    pub vel: u8,
    pub notes: Vec<u8>,
    pub duration: u32,
}

#[macro_export]
macro_rules! note {
    ($vel:expr, $duration:expr, $note:expr) => (
        Note::new($vel, $duration, vec![$note])   
        );

    ($vel:expr, $duration:expr, $note:expr, $($n:expr),+) => (
        Note::new($vel, $duration, vec![$note, $($n),+])
    )
}

impl Note {
    pub fn new(vel: u8, duration: u32, notes: Vec<u8>) -> Note { 
        Note {vel: vel, notes: notes, duration: duration} 
    }

    pub fn pause(duration: u32) -> Note {
        Note {vel: 0, notes: Vec::new(), duration: duration}
    }
}

pub struct Track{
    notes: HashMap<usize, Note>,
    i: usize,
    pub instrument: Instrument,
    pub dynamics: u8,

    pub tick: u32,
}

impl Track {
    pub fn new() -> Track {
        Track { notes: HashMap::new(), i: 0, instrument: Instrument::AcousticGrandPiano, dynamics: dynamics::MEZZO_FORTE, tick: 0}
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

    pub fn set_dynamics(&mut self, dynamics: u8) -> &mut Track {
        self.dynamics = dynamics;    
        self
    }
}

pub struct MidiObj{
    pub tracks: Vec<Track>,
    pub tempo: u32,
    pub time_signature: (u8, u8),
    pub key_signature: (u8, bool),
}

impl MidiObj {
    pub fn new() -> MidiObj {
        MidiObj { tracks: Vec::new(), tempo: 120, time_signature: (4, 4), key_signature: (0, false) }
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

    pub fn set_key_signature(&mut self, key_signature: (u8, bool)) -> &mut MidiObj {
        self.key_signature = key_signature;
        self
    }
}
