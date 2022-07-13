pub mod stream;
mod tests;
mod util;

use crate::MidiObj;

impl stream::Sourceable<u8> for MidiObj {
    fn from_stream<T: stream::InStream<u8>>(mut stream: T) -> Result<Box<Self>, String> {
        if !util::check_str(&mut stream, "MThd") {
            return Err(String::from("Invalid Midi file"));
        }

        let header_length: u32 = util::get_u32(&mut stream)?;

        Ok(Box::new(Self::new()))
    }
}
