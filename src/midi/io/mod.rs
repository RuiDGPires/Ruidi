pub mod stream;
mod tests;
mod util;

use crate::{MidiObj, Note};
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
                let _delta_time = varlen.val;

            }
        }

        Ok(Box::new(Self::new()))
    }

    fn to_stream<T: stream::OutStream<u8>>(&self, mut stream: T) -> Result<(), String> {
        use stream::OutStream;
        stream.write('M' as u8)?;
        stream.write('T' as u8)?;
        stream.write('h' as u8)?;
        stream.write('d' as u8)?;

        (6 as u32).write(&mut stream)?;
        (0 as u16).write(&mut stream)?;
        (self.tracks.len() as u16).write(&mut stream)?;
        (96 as u16).write(&mut stream)?;

        for track in &self.tracks {
            let mut track_stream = stream::VecByteStream::new(Vec::new());
            stream.write('M' as u8)?;
            stream.write('T' as u8)?;
            stream.write('r' as u8)?;
            stream.write('k' as u8)?;
            
            util::VarLen::new(0).write(&mut track_stream)?;
            (0xFF_58 as u16).write(&mut track_stream)?; // Time signature
            track_stream.write(0x04)?;
            track_stream.write(0x04)?;
            track_stream.write(0x02)?;
            track_stream.write(0x18)?;
            track_stream.write(0x08)?;

            util::VarLen::new(0).write(&mut track_stream)?;
            (0xFF_51 as u16).write(&mut track_stream)?; // Tempo
            track_stream.write(0x03)?;

            let tempo: u32 = 60_000_000 / self.tempo;

            for i in 0..3 {
                track_stream.write((tempo >> (2 - i)*8) as u8)?;
            }

            util::VarLen::new(0).write(&mut track_stream)?;
            track_stream.write(0xC0)?; // Program change
            track_stream.write(self.instrument as u8)?;
            
            for i in 0..track.i {
                let note : &Note = track.notes.get(&i).unwrap();

                util::VarLen::new(0).write(&mut track_stream)?;
                track_stream.write(0x90)?;
                track_stream.write(note.note)?; // Note on
                track_stream.write(note.vel)?;

                util::VarLen::new(note.duration).write(&mut track_stream)?;
                track_stream.write(0x80)?;
                track_stream.write(note.note)?; // Note off
                track_stream.write(0x0)?;
            }

            util::VarLen::new(0).write(&mut track_stream)?;
            track_stream.write(0xFF)?; // EOT
            track_stream.write(0x2F)?;
            track_stream.write(0)?;

            (track_stream.size() as u32).write(&mut stream)?;
            track_stream.into_filestream(&mut stream)?;
        }
         
        Ok(())
    } 
}
