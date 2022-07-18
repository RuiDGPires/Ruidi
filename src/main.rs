use midi::{io::stream::{FileByteOutStream, Sourceable}, MidiObj, Track, Note};
use midi::instruments::Instrument;

fn main() {
    
    let mut obj = MidiObj::new();

    //obj.set_timesignature(0, 4, 4);
    let mut track = Track::new();
    track.set_instrument(Instrument::TenorSax)
            .add_note(Note::new(0x7F, 50, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            .add_note(Note::new(0x7F, 52, 96))
            ;

    obj.set_tempo(120).add_track(track);

    {
        let stream = FileByteOutStream::new(String::from("/tmp/testing.mid"));
        obj.to_stream(stream).unwrap();
    }
}
