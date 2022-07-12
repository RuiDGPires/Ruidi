use midi::{MidiObj, stream::*};
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("wing2003.mid");

    let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

    match MidiObj::from_file(stream) {
        Ok(obj) => {
            let mut midiObj: MidiObj = *obj;

            midiObj.add_voice();
            midiObj.add_note(0, midi::Note::pause(2));
        }
        Err(e) => println!("Error: {}", e)
    }
}
