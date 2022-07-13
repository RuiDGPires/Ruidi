#[cfg(test)]
mod tests{
    use crate::MidiObj;
    use super::super::stream::*;
    use std::path::PathBuf;

    #[test]
    fn test_midi_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/wing2003.mid");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert!(!MidiObj::from_stream(stream).is_err());
    }

    #[test]
    fn test_invalid_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/test_fileinstream");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert!(MidiObj::from_stream(stream).is_err());
    }
}
