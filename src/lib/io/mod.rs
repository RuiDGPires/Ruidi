pub mod stream;
mod tests;

use crate::MidiObj;

fn check_str<T: stream::InStream<u8>>(stream: &mut T, string: &str) -> bool {
    for c in string.chars() {
        if stream.read() != Some(&(c as u8)) {
            return false;
        }
    }
    true
}

fn get_u32<T: stream::InStream<u8>>(stream: &mut T) -> Result<u32, String> {
    let mut val = 0;

    for i in 0..4 {
        match stream.read() {
            Some(v) => {val |= (*v as u32) << (3 - i)*8;}
            None   => return Err(String::from("Unexpected end of file")),
        }
    }

    Ok(val)
} 

impl stream::Sourceable<u8> for MidiObj {
    fn from_stream<T: stream::InStream<u8>>(mut stream: T) -> Result<Box<Self>, String> {
        if !check_str(&mut stream, "MThd") {
            return Err(String::from("Invalid Midi file"));
        }

        let header_length: u32 = get_u32(&mut stream)?;

        Ok(Box::new(Self::new()))
    }
}
