mod tests;
pub mod io;
use std::collections::HashMap;

trait Event {
}

pub struct Note {
    vel: u32,
    note: u32,
    duration: u32,
}

impl Event for Note{}

impl Note {
    pub fn new(vel: u32, note: u32, duration: u32) -> Note { 
        Note {vel: vel, note: note, duration: duration} 
    }

    pub fn pause(duration: u32) -> Note {
        Note {vel: 0, note: 0, duration: duration}
    }
}

pub struct Track{
    notes: HashMap<usize, Note>
}

impl Track {
    pub fn new() -> Track {
        Track { notes: HashMap::new() }
    }

    pub fn add_note(&mut self, time: usize, note: Note) -> &mut Track {
        self.notes.insert(time, note);
        self
    }
}

pub struct MidiObj{
    pub tracks: Vec<Track>
}

impl MidiObj {
    pub fn new() -> MidiObj {
        MidiObj { tracks: Vec::new() }
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

    pub fn add_note(&mut self, track: usize, time: usize, note: Note) -> &mut MidiObj {
        self.tracks[track].add_note(time, note);
        self
    }
}
