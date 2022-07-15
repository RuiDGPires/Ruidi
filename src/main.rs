use midi::{io::stream::{FileByteInStream, FileByteOutStream, Sourceable}, MidiObj};
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_files/wing2003.mid");
    
    let stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());
    
    let obj = MidiObj::from_stream(stream).unwrap();
    
    {
        let stream = FileByteOutStream::new(String::from("/tmp/testing"));
        obj.to_stream(stream).unwrap();
    }
}
