#[cfg(test)]
mod tests{
    use crate::MidiObj;
    use super::super::stream::*;
    use super::super::*;
    use std::path::PathBuf;

    #[test]
    fn test_midi_from_file() {
        let mut pathOK = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        pathOK.push("test_files/wing2003.mid");
        let mut pathERR = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        pathERR.push("test_files/test_fileinstream");

        let mut streamOK = FileByteInStream::new(pathOK.into_os_string().into_string().unwrap());
        let mut streamERR = FileByteInStream::new(pathERR.into_os_string().into_string().unwrap());

        assert!(!MidiObj::from_stream(streamOK).is_err());
        assert!(MidiObj::from_stream(streamERR).is_err());
    }

    #[test]
    fn test_check_str() {
        let mut stream1 = VecByteStream::new("MThd".chars().map(|x| x as u8).collect());
        let mut stream2 = VecByteStream::new("MTdh".chars().map(|x| x as u8).collect());

        assert!(check_str(&mut stream1, "MThd"));
        assert!(!check_str(&mut stream2, "MThd"));
    }

    #[test]
    fn test_get_u32() {
        let mut stream = VecByteStream::new(vec![0x12, 0x34, 0x56, 0x78]);

        assert_eq!(get_u32(&mut stream).unwrap(), 0x12345678);
    }
}
