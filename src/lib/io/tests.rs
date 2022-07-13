#[cfg(test)]
mod tests{
    use crate::MidiObj;
    use super::super::stream::*;
    use super::super::util::*;
    use std::path::PathBuf;

    #[test]
    fn test_midi_from_file() {
        let mut path_ok = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_ok.push("test_files/wing2003.mid");
        let mut path_err = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_err.push("test_files/test_fileinstream");

        let stream_ok = FileByteInStream::new(path_ok.into_os_string().into_string().unwrap());
        let stream_err = FileByteInStream::new(path_err.into_os_string().into_string().unwrap());

        assert!(!MidiObj::from_stream(stream_ok).is_err());
        assert!(MidiObj::from_stream(stream_err).is_err());
    }

    #[test]
    fn test_check_str() {
        let mut stream_ok = VecByteStream::new("MThd".chars().map(|x| x as u8).collect());
        let mut stream_err = VecByteStream::new("MTdh".chars().map(|x| x as u8).collect());

        assert!(check_str(&mut stream_ok, "MThd"));
        assert!(!check_str(&mut stream_err, "MThd"));
    }

    #[test]
    fn test_get_u32() {
        let mut stream = VecByteStream::new(vec![0x12, 0x34, 0x56, 0x78]);

        assert_eq!(get_u32(&mut stream).unwrap(), 0x12345678);
    }

    #[test]
    fn varlen_read() {
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0x00])).unwrap()).val == 0x00);
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0x40])).unwrap()).val == 0x40);
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0x7F])).unwrap()).val == 0x7F);
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0x81, 0x00])).unwrap()).val == 0x80);
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0xC0, 0x00, 0x01, 0x7F])).unwrap()).val == 0x2000);
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0xFF, 0x7F, 0xFF])).unwrap()).val == 0x3FFF);
        assert!((*VarLen::read(&mut VecByteStream::new(vec![0x81, 0x80, 0x00])).unwrap()).val == 0x4000);
    }

    #[test]
    fn varlen_write() {
    // TODO
    //    let bytes1 = &[0x00];
    //    assert!(&VarLen::read(bytes1).write()[..] == bytes1);
    //    let bytes2 = &[0x40];
    //    assert!(&VarLen::read(bytes2).write()[..] == bytes2);
    //    let bytes3 = &[0x7F];
    //    assert!(&VarLen::read(bytes3).write()[..] == bytes3);
    //    let bytes4 = &[0x81, 0x00];
    //    assert!(&VarLen::read(bytes4).write()[..] == bytes4);
    //    let bytes5 = &[0xC0, 0x00];
    //    assert!(&VarLen::read(bytes5).write()[..] == bytes5);
    //    let bytes6 = &[0xFF, 0x7F];
    //    assert!(&VarLen::read(bytes6).write()[..] == bytes6);
    //    let bytes7 = &[0x81, 0x80, 0x00];
    //    assert!(&VarLen::read(bytes7).write()[..] == bytes7);
    }
}
