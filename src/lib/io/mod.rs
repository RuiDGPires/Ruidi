pub mod stream;
mod tests;

pub mod io {
    use super::stream;
    use crate::MidiObj;

    fn check_str<T: stream::InStream<u8>>(stream: &mut T, string: &str) -> bool {
        for c in string.chars() {
            if stream.read() != Some(&(c as u8)) {
                return false;
            }
        }
        true
    }

    impl stream::Sourceable<u8> for MidiObj {
        fn from_stream<T: stream::InStream<u8>>(mut stream: T) -> Result<Box<Self>, String> {
            if !check_str(&mut stream, "MThd") {
                return Err(String::from("Invalid Midi file"));
            }

            Ok(Box::new(Self::new()))
        }
    }

}
