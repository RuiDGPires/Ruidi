#[cfg(test)]
mod tests{
    use super::super::{MidiObj, Track, Note};

    #[test]
    fn create_empty_midi_obj() {
        let obj: MidiObj = MidiObj::new(); 
        assert!(obj.tracks.is_empty());
    }

    #[test]
    fn create_empty_track_obj() {
        let track: Track = Track::new(); 
        assert!(track.notes.is_empty());
    }

    #[test]
    fn add_track_to_midiobj() {
        let mut obj: MidiObj = MidiObj::new(); 
        obj.add_track();
        assert!(!obj.tracks.is_empty());
        assert!(obj.tracks[0].notes.is_empty());
    }

    #[test]
    fn add_note() {
        let mut obj: MidiObj = MidiObj::new_sized(1); 
        assert!(!obj.tracks.is_empty());
        obj.add_note(0, Note::new(10, 15, 20));
        obj.add_note(0, Note::pause(2));
        obj.add_note(0, Note::new(2, 2, 3));
        
        assert!(obj.tracks[0].notes.get(&0).unwrap().vel == 10);
        assert!(obj.tracks[0].notes.get(&0).unwrap().note == 15);
        assert!(obj.tracks[0].notes.get(&0).unwrap().duration == 20);

        assert!(obj.tracks[0].notes.get(&1).unwrap().vel == 0);
        assert!(obj.tracks[0].notes.get(&1).unwrap().note == 0);
        assert!(obj.tracks[0].notes.get(&1).unwrap().duration == 2);

        assert!(obj.tracks[0].notes.get(&2).unwrap().duration == 3);
    }
}

