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
    fn test_read_u32() {
        let mut stream1 = VecByteStream::new(vec![0x12, 0x34, 0x56, 0x78]);
        let mut stream2 = VecByteStream::new(vec![0x12, 0x34, 0x56, 0x78, 0x99, 1, 0]);

        assert_eq!(*u32::read(&mut stream1).unwrap(), 0x12345678);
        assert_eq!(*u32::read(&mut stream2).unwrap(), 0x12345678);
    }

    #[test]
    fn test_write_u32() {
        let mut stream = VecByteStream::new(Vec::new());

        (0x12345678 as u32).write(&mut stream).unwrap();
        assert_eq!(*u32::read(&mut stream).unwrap(), 0x12345678);
    }

    #[test]
    fn test_write_u16() {
        let mut stream = VecByteStream::new(Vec::new());

        (0x1234 as u16).write(&mut stream).unwrap();
        assert_eq!(*u16::read(&mut stream).unwrap(), 0x1234);
    }

    #[test]
    fn test_read_u16() {
        let mut stream = VecByteStream::new(vec![0x12, 0x34]);

        assert_eq!(*u16::read(&mut stream).unwrap(), 0x1234);
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
        let mut stream = VecByteStream::new(Vec::new());

        (*VarLen::read(&mut VecByteStream::new(vec![0x00])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0x00]);


        let mut stream = VecByteStream::new(Vec::new());
        (*VarLen::read(&mut VecByteStream::new(vec![0x40])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0x40]);

        let mut stream = VecByteStream::new(Vec::new());
        (*VarLen::read(&mut VecByteStream::new(vec![0x7F])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0x7F]);

        let mut stream = VecByteStream::new(Vec::new());
        (*VarLen::read(&mut VecByteStream::new(vec![0x81, 0x00])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0x81, 0x00]);

        let mut stream = VecByteStream::new(Vec::new());
        (*VarLen::read(&mut VecByteStream::new(vec![0xC0, 0x00])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0xC0, 0x00]);

        let mut stream = VecByteStream::new(Vec::new());
        (*VarLen::read(&mut VecByteStream::new(vec![0xFF, 0x7F])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0xFF, 0x7F]);

        let mut stream = VecByteStream::new(Vec::new());
        (*VarLen::read(&mut VecByteStream::new(vec![0x81, 0x80, 0x00])).unwrap()).write(&mut stream).unwrap();
        assert!(stream.to_vec() == vec![0x81, 0x80, 0x00]);
    }
}
