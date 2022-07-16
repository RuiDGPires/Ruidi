use midi::{io::stream::{FileByteOutStream, Sourceable}, MidiObj, Note};
use midi::instruments::Instrument;

fn main() {
    
    let mut obj = MidiObj::new();
    obj.add_track();

    //obj.set_timesignature(0, 4, 4);
    obj.set_tempo(120)
        .set_instrument(Instrument::TenorSax)
        .add_note(0, Note::new(0x7F, 50, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        .add_note(0, Note::new(0x7F, 52, 96))
        ;

    {
        let stream = FileByteOutStream::new(String::from("/tmp/testing.mid"));
        obj.to_stream(stream).unwrap();
    }
}
