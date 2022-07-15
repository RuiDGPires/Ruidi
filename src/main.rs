use midi::{io::stream::{FileByteInStream, FileByteOutStream, Sourceable}, MidiObj};
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_files/wing2003.mid");
    
    let stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());
    
    let mut obj = MidiObj::from_stream(stream).unwrap();
    obj.add_track();

    {
        let stream = FileByteOutStream::new(String::from("/tmp/testing.mid"));
        obj.to_stream(stream).unwrap();
    }
}
