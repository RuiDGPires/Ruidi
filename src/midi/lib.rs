mod tests;
pub mod io;
pub mod instruments;

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
}

impl Track {
    pub fn new() -> Track {
        Track { notes: HashMap::new(), i: 0 }
    }

    pub fn add_note(&mut self, note: Note) -> &mut Track {
        self.notes.insert(self.i, note);
        self.i += 1;
        self
    }

    pub fn get_notes(&self) -> &HashMap<usize, Note> {
        &self.notes
    }
}

pub struct MidiObj{
    pub tracks: Vec<Track>,
    pub tempo: u32,
    //pub signature: u32,
    pub instrument: Instrument,
}

impl MidiObj {
    pub fn new() -> MidiObj {
        MidiObj { tracks: Vec::new(), tempo: 120, instrument: Instrument::AcousticGrandPiano }
    }

    pub fn new_sized(size: usize) -> MidiObj {
        let mut obj = MidiObj::new();

        for _ in 0 .. size {
            obj.add_track();
        }
         
       obj 
    }

    pub fn add_track(&mut self) -> &mut MidiObj {
        self.tracks.push(Track::new()); 
        self
    }

    pub fn add_note(&mut self, track: usize, note: Note) -> &mut MidiObj {
        self.tracks[track].add_note(note);
        self
    }

    pub fn set_tempo(&mut self, tempo: u32) -> &mut MidiObj {
        self.tempo = tempo;
        self
    }

    pub fn set_instrument(&mut self, intrument: Instrument) -> &mut MidiObj{
        self.instrument = intrument;
        self
    }
}
