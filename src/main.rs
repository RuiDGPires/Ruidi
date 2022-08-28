use midi::{io::stream::{FileByteInStream, FileByteOutStream, Sourceable}, MidiObj, Track, Note, note};
use midi::instruments::Instrument;
use midi::pitch::*;
use midi::dynamics;
use midi::durations;

use std::env;

const DEFAULT_OUTPUT_FILE: &str = "/tmp/default.mid";

struct Conf {
    outfile: String,
    infile:  String
}

impl Conf {
    pub fn from_args(argv: &Vec<String>) -> Self {
        let mut i: usize = 1;
        let mut infile:  String = String::from("");
        let mut outfile: String = String::from(DEFAULT_OUTPUT_FILE);
        let argc = argv.len();

        while i < argc {
            if argv[i] == "-i" {
                i += 1;
                infile = argv[i].clone();
            }else if argv[i] == "-o" {
                i += 1;
                outfile = argv[i].clone();
            }

            i += 1;
        }

        Self {outfile: outfile, infile: infile}
    }
}

fn demo() -> MidiObj{
    let mut obj = MidiObj::new();

    obj.set_tempo(120)
        .set_time_signature((4, 4))
        .set_key_signature((0, false))
        ;

    let mut track1 = Track::new();
    let mut track2 = Track::new();

    track1.set_instrument(Instrument::TenorSax)
        .set_dynamics(dynamics::FORTE);

    for i in 0..8 {
        track1.add_note(note!(dynamics::AUTO, durations::QUARTER, octave(C + i, 4)));
    }

    track2.set_instrument(Instrument::AltoSax)
        .set_dynamics(dynamics::FORTE);

    for i in 0..16 {
        track2.add_note(note!(dynamics::AUTO, durations::EIGHTH, octave(C + i, 4), octave(C + i - 4, 4)));
    }

    obj.add_track(track1)
        .add_track(track2);

    obj
}

fn main() {
    let conf: Conf = Conf::from_args(&env::args().collect());
    
    {
        if conf.infile != "" {
            *MidiObj::from_stream(FileByteInStream::new(conf.infile)).unwrap()
        } else {
            demo()
        }

    }.to_stream(FileByteOutStream::new(conf.outfile)).unwrap();
}
