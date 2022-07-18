use midi::{io::stream::{FileByteOutStream, Sourceable}, MidiObj, Track, Note};
use midi::instruments::Instrument;
use midi::pitch::*;

fn main() {
    
    let mut obj = MidiObj::new();
    obj.set_tempo(120);

    //obj.set_timesignature(0, 4, 4);
    {
        let mut track = Track::new();

        track.set_instrument(Instrument::TenorSax);
        for i in 0..8 {
            track.add_note(Note::new(0x7F, octave(C + i, 4), Note::QUARTER));
        }
        obj.add_track(track);
    }
    {
        let mut track = Track::new();

        track.set_instrument(Instrument::AltoSax);
        for i in 0..16 {
            track.add_note(Note::new(0x7F, octave(C + i, 4), Note::EIGHT));
        }
        obj.add_track(track);
    }

    let stream = FileByteOutStream::new(String::from("/tmp/testing.mid"));
    obj.to_stream(stream).unwrap();
}
