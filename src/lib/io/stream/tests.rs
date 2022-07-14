#[cfg(test)]
pub mod tests {
    use super::super::*;
    use std::path::PathBuf;

    #[test]
    fn test_fileinstream() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/test_fileinstream");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }

    #[test]
    fn test_fileoutstream() {
        let path = "/tmp/test";

        std::fs::remove_file(path);

        {
            let mut stream = FileByteOutStream::new(String::from(path));

            stream.write(0);
            stream.write(1);
            stream.write(2);
        } // The contents are now flushed

        let mut stream = FileByteInStream::new(String::from(path));

        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }

    #[test]
    fn test_vecinstream() {
        let mut stream = VecByteStream::new(vec![0, 1, 2]);

        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }

    #[test]
    fn test_vecoutstream() {
        let mut stream = VecByteStream::new(Vec::new());

        stream.write(0);
        stream.write(1);
        stream.write(2);
        
        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }
}
