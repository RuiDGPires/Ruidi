use midi::{io::stream::{FileByteOutStream, Sourceable}, MidiObj, Note};

fn main() {
    
    let mut obj = MidiObj::new();
    obj.add_track();
    obj.add_note(0, Note::new(0x7F, 50, 100));
    obj.add_note(0, Note::pause(100));
    obj.add_note(0, Note::new(0x7F, 50, 100));

    {
        let stream = FileByteOutStream::new(String::from("/tmp/testing.mid"));
        obj.to_stream(stream).unwrap();
    }
}
