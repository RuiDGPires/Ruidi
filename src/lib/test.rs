#[cfg(test)]
mod tests{
    use super::super::{MidiObj, Voice, Note, util::VarLen, stream::FileByteInStream, stream::InStream};

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

    #[test]
    fn varlen_read() {
        assert!(VarLen::read(&[0x00]).val == 0x00);
        assert!(VarLen::read(&[0x40]).val == 0x40);
        assert!(VarLen::read(&[0x7F]).val == 0x7F);
        assert!(VarLen::read(&[0x81, 0x00]).val == 0x80);
        assert!(VarLen::read(&[0xC0, 0x00, 0x01, 0x7F]).val == 0x2000);
        assert!(VarLen::read(&[0xFF, 0x7F, 0xFF]).val == 0x3FFF);
        assert!(VarLen::read(&[0x81, 0x80, 0x00]).val == 0x4000);
    }

    #[test]
    fn varlen_write() {
        let bytes1 = &[0x00];
        assert!(&VarLen::read(bytes1).write()[..] == bytes1);
        let bytes2 = &[0x40];
        assert!(&VarLen::read(bytes2).write()[..] == bytes2);
        let bytes3 = &[0x7F];
        assert!(&VarLen::read(bytes3).write()[..] == bytes3);
        let bytes4 = &[0x81, 0x00];
        assert!(&VarLen::read(bytes4).write()[..] == bytes4);
        let bytes5 = &[0xC0, 0x00];
        assert!(&VarLen::read(bytes5).write()[..] == bytes5);
        let bytes6 = &[0xFF, 0x7F];
        assert!(&VarLen::read(bytes6).write()[..] == bytes6);
        let bytes7 = &[0x81, 0x80, 0x00];
        assert!(&VarLen::read(bytes7).write()[..] == bytes7);
    }
}

