use midi::{io::stream::{FileByteOutStream, Sourceable}, MidiObj, Track, Note, note};
use midi::instruments::Instrument;
use midi::pitch::*;
use midi::dynamics;
use midi::durations;

use std::env;

const USAGE: &str = "./Ruidi <midi_file>";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output_file = String::from("/tmp/default.mid");

    match args.len() {
        2 => {output_file = args[1].clone();}
        1 => {}
        _ => {panic!("Invalid number of command line arguments\nUsage: \n\t{}", USAGE);}
    }
    
    let mut obj = MidiObj::new();

    obj.set_tempo(120)
        .set_time_signature((4, 4))
        .set_key_signature((7, false))
        ;

    let mut track1 = Track::new();

    track1.set_instrument(Instrument::TenorSax)
        .set_dynamics(dynamics::FORTE)
        ;

    for i in 0..8 {
        track1.add_note(note!(dynamics::AUTO, durations::QUARTER, octave(C + i, 4)));
    }
    obj.add_track(track1);

    let mut track2 = Track::new();

    track2.set_instrument(Instrument::AltoSax)
        .set_dynamics(dynamics::FORTE)
        ;

    for i in 0..16 {
        track2.add_note(note!(dynamics::AUTO, durations::EIGHTH, octave(C + i, 4), octave(C + i - 4, 4)));
    }
    obj.add_track(track2);

    obj.to_stream(FileByteOutStream::new(output_file)).unwrap();
}
