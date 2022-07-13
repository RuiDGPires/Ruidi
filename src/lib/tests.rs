#[cfg(test)]
mod tests{
    use super::super::{MidiObj, Voice, Note};

    #[test]
    fn create_empty_midi_obj() {
        let obj: MidiObj = MidiObj::new(); 
        assert!(obj.voices.is_empty());
    }

    #[test]
    fn create_empty_voice_obj() {
        let voice: Voice = Voice::new(); 
        assert!(voice.notes.is_empty());
    }

    #[test]
    fn add_voice_to_midiobj() {
        let mut obj: MidiObj = MidiObj::new(); 
        obj.add_voice();
        assert!(!obj.voices.is_empty());
        assert!(obj.voices[0].notes.is_empty());
    }

    #[test]
    fn add_note() {
        let mut obj: MidiObj = MidiObj::new_sized(1); 
        assert!(!obj.voices.is_empty());
        obj.add_note(0, Note::new(10, 15, 20));
        obj.add_note(0, Note::pause(2));
        obj.add_note(0, Note::new(2, 2, 3));
        
        assert!(obj.voices[0].notes[0].vel == 10);
        assert!(obj.voices[0].notes[0].note == 15);
        assert!(obj.voices[0].notes[0].duration == 20);

        assert!(obj.voices[0].notes[1].vel == 0);
        assert!(obj.voices[0].notes[1].note == 0);
        assert!(obj.voices[0].notes[1].duration == 2);

        assert!(obj.voices[0].notes[2].duration == 3);
    }
}

