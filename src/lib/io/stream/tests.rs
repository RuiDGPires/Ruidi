#[cfg(test)]
pub mod tests {
    use super::super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filestream() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/test_fileinstream");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }

    #[test]
    fn test_vecstream() {
        let mut stream = VecByteStream::new(vec![0, 1, 2]);

        assert_eq!(stream.read(), Some(&0));
        assert_eq!(stream.read(), Some(&1));
        assert_eq!(stream.read(), Some(&2));
    }
}
