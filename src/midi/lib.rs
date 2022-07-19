mod tests;
pub mod io;
pub mod instruments;
pub mod pitch;

pub mod dynamics {
    pub const AUTO:               u8 = 0xFF;

    pub const FORTISSIMO:         u8 = 101;
    pub const FORTE:              u8 = 88;
    pub const MEZZO_FORTE:        u8 = 75;
    pub const MEZZO_PIANO:        u8 = 62;
    pub const PIANO:              u8 = 49;
    pub const PIANISSIMO:         u8 = 36;
}

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
    // American names
    pub const QUARTER:   u32 = 96;
    pub const HALF:      u32 = Self::QUARTER*2;
    pub const WHOLE:     u32 = Self::HALF*2;
    pub const EIGHTH:    u32 = Self::QUARTER / 2;
    pub const SIXTEENTH: u32 = Self::QUARTER / 2;

    // British names
    pub const SEMIBREVE:     u32 = Self::WHOLE;
    pub const CROTCHET:      u32 = Self::QUARTER;
    pub const QUAVER:        u32 = Self::EIGHTH;
    pub const SEMI_QUAVER:   u32 = Self::SIXTEENTH;
    pub const MINIM:         u32 = Self::HALF;

    pub fn doted(val: u32) -> u32 {
        (val as f32 * 1.5) as u32
    }

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
