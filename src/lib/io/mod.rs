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
        let format:        u16 = util::get_u16(&mut stream)?;
        let ntrks:         u16 = util::get_u16(&mut stream)?;
        let division:      u16 = util::get_u16(&mut stream)?; // TODO bit 15 checking

        println!("Length: {}", header_length);
        println!("Format: {}", format);
        println!("Number of tracks: {}", ntrks);
        println!("Division: {}", division);

        loop {
            if !util::check_str(&mut stream, "MTrk") {
                break;
            }
            let length: u32 = util::get_u32(&mut stream)?;

            let mut i = 0;
            while i < length{
                // Parse events
                let varlen: util::VarLen = *util::VarLen::read(&mut stream)?;
                i += varlen.size as u32;
                let delta_time = varlen.val;

            }
        }

        Ok(Box::new(Self::new()))
    }
}
