pub mod stream;
mod tests;
mod util;

use crate::MidiObj;
use util::Streamable;

impl stream::Sourceable<u8> for MidiObj {
    fn from_stream<T: stream::InStream<u8>>(mut stream: T) -> Result<Box<Self>, String> {
        if !util::check_str(&mut stream, "MThd") {
            return Err(String::from("Invalid Midi file"));
        }

        let header_length: u32 = *u32::read(&mut stream)?;
        let format:        u16 = *u16::read(&mut stream)?;
        let ntrks:         u16 = *u16::read(&mut stream)?;
        let division:      u16 = *u16::read(&mut stream)?; // TODO bit 15 checking

        println!("Length: {}", header_length);
        println!("Format: {}", format);
        println!("Number of tracks: {}", ntrks);
        println!("Division: {}", division);

        loop {
            if !util::check_str(&mut stream, "MTrk") {
                break;
            }
            let length: u32 = *u32::read(&mut stream)?;

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

    fn to_stream<T: stream::OutStream<u8>>(&self, mut stream: T) -> Result<(), String> {
        stream.write('M' as u8)?;
        stream.write('T' as u8)?;
        stream.write('h' as u8)?;
        stream.write('d' as u8)?;

        (6 as u32).write(&mut stream)?;
        (0 as u16).write(&mut stream)?;
        (self.tracks.len() as u16).write(&mut stream)?;
        (96 as u16).write(&mut stream)?;
         
        Ok(())
    } 
}
