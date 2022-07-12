#[cfg(test)]
mod tests{
    use midi::stream::*;
    use midi::MidiObj;
    use std::path::PathBuf;

    #[test]
    fn test_filestream() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/files/test_fileinstream");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }

    #[test]
    fn test_midi_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/files/wing2003.mid");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert!(!MidiObj::from_file(stream).is_err());
    }

    #[test]
    fn test_invalid_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/files/test_fileinstream");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert!(MidiObj::from_file(stream).is_err());
    }
}
