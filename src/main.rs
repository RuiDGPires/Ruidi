use midi::{MidiObj, io::stream::FileByteInStream, io::stream::Sourceable};
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_files/wing2003.mid");

    let stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

    let _obj: MidiObj = *MidiObj::from_stream(stream).expect("oi");
}
