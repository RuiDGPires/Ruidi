use midi::{io::stream::{FileByteOutStream, Sourceable}, MidiObj, Track, Note};
use midi::instruments::Instrument;
use midi::pitch::*;
use midi::dynamics;

fn main() {
    
    let mut obj = MidiObj::new();
    obj.set_tempo(120).set_time_signature((4, 4));

    //obj.set_timesignature(0, 4, 4);
    let mut track1 = Track::new();

    track1.set_instrument(Instrument::TenorSax);
    for i in 0..8 {
        track1.add_note(Note::new(dynamics::MEZZO_FORTE, octave(C + i, 4), Note::QUARTER));
    }
    track1.add_note(Note::new(dynamics::PIANO, octave(C, 4), Note::doted(Note::QUARTER)));
    obj.add_track(track1);


    let mut track2 = Track::new();

    track2.set_instrument(Instrument::AltoSax);
    for i in 0..16 {
        track2.add_note(Note::new(0x7F, octave(C + i, 4), Note::EIGHTH));
    }
    obj.add_track(track2);

    let stream = FileByteOutStream::new(String::from("/tmp/testing.mid"));
    obj.to_stream(stream).unwrap();
}
