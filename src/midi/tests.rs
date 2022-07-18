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
        obj.add_track(Track::new());
        assert!(!obj.tracks.is_empty());
        assert!(obj.tracks[0].notes.is_empty());
    }

    #[test]
    fn add_note() {
        let mut obj: MidiObj = MidiObj::new(); 

        let mut track = Track::new();
        track.add_note(Note::new(10, 15, 20))
            .add_note(Note::pause(2))
            .add_note(Note::new(2, 2, 3));
        
        obj.add_track(track);

        assert!(!obj.tracks.is_empty());
        assert!(obj.tracks[0].notes.get(&0).unwrap().vel == 10);
        assert!(obj.tracks[0].notes.get(&0).unwrap().note == 15);
        assert!(obj.tracks[0].notes.get(&0).unwrap().duration == 20);

        assert!(obj.tracks[0].notes.get(&1).unwrap().vel == 0);
        assert!(obj.tracks[0].notes.get(&1).unwrap().note == 0);
        assert!(obj.tracks[0].notes.get(&1).unwrap().duration == 2);

        assert!(obj.tracks[0].notes.get(&2).unwrap().duration == 3);
    }
}

