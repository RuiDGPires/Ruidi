#[derive(Debug)]
pub struct Note {
    vel: u32,
    note: u32,
    duration: u32,
}

pub fn pause(duration: u32) -> Note {
    Note {vel: 0, note: 0, duration: duration}
}

#[derive(Debug)]
pub struct Voice{
    notes: Vec<Note>
}

#[derive(Debug)]
pub struct MidiObj{
    voices: Vec<Voice>
}

pub fn new_midi_object() -> MidiObj {
    MidiObj {voices: Vec::new()}
}
