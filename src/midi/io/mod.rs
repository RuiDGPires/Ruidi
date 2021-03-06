pub mod stream;
mod tests;
mod util;
mod events;

use crate::{MidiObj, Note, dynamics};
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
        (1 as u16).write(&mut stream)?; // Format
        (self.tracks.len() as u16 + 1).write(&mut stream)?;
        (96 as u16).write(&mut stream)?;

        //----
        // Header stuff
        {
        let mut track_stream = stream::VecByteStream::new(Vec::new());
        stream.write('M' as u8)?;
        stream.write('T' as u8)?;
        stream.write('r' as u8)?;
        stream.write('k' as u8)?;

        events::TimeSignature::new(0, self.time_signature.0, self.time_signature.1).on_tick(&mut 0).write(&mut track_stream)?;

        events::Tempo::new(0, self.tempo).on_tick(&mut 0).write(&mut track_stream)?; 
        events::KeySignature::new(0, self.key_signature.0, self.key_signature.1).on_tick(&mut 0).write(&mut track_stream)?;

        events::TimeSignature::new(4*96, 3, 4).on_tick(&mut 0).write(&mut track_stream)?;

        util::VarLen::new(0).write(&mut track_stream)?;
        track_stream.write(0xFF)?; // EOT
        track_stream.write(0x2F)?;
        track_stream.write(0)?;

        (track_stream.size() as u32).write(&mut stream)?; // Track size
        track_stream.into_filestream(&mut stream)?;
        }
        //----

        for (i, track) in self.tracks.iter().enumerate() {
            println!("Writing track {}", i);
            let mut track_stream = stream::VecByteStream::new(Vec::new());
            let channel = i as u8;
            stream.write('M' as u8)?;
            stream.write('T' as u8)?;
            stream.write('r' as u8)?;
            stream.write('k' as u8)?;

            util::VarLen::new(0).write(&mut track_stream)?;
            track_stream.write(0xC0 | channel)?; // Program change
            track_stream.write(track.instrument as u8)?;
            
            for i in 0..track.i {
                let note : &Note = track.notes.get(&i).unwrap();
                let mut tick = 0;

                let mut velocity = note.vel;
                if note.vel == dynamics::AUTO {
                    velocity = track.dynamics;
                }
                
                for pitch in &note.notes {
                    events::NoteOn::new(0, velocity, *pitch, channel).on_tick(&mut tick).write(&mut track_stream)?;
                }

                for pitch in &note.notes {
                    events::NoteOff::new(note.duration, *pitch, channel).on_tick(&mut tick).write(&mut track_stream)?;
                }
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
