use super::util::Streamable;
use super::stream;
use super::util::VarLen;
use std::cell::RefCell;


thread_local!(static PREVIOUS_TICK: RefCell<u32> = RefCell::new(0));

#[derive(Copy, Clone)]
pub struct NoteOn {
    pub time: u32,
    pub vel: u8,
    pub note: u8,
    pub channel: u8,

    previous: u32,
}

#[derive(Copy, Clone)]
pub struct NoteOff {
    pub time:  u32,
    pub note: u8,
    pub channel: u8,

    previous: u32,
}

#[derive(Copy, Clone)]
pub struct Tempo {
    pub time: u32,
    pub bpm: u32,

    previous: u32
}

pub struct TimeSignature {}
pub struct KeySignature {}

impl NoteOn {
    pub fn new(time: u32, vel: u8, note: u8, channel: u8) -> Self {
        Self{time: time, vel: vel, note: note, channel: channel, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u32) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl NoteOff {
    pub fn new(time: u32, note: u8, channel: u8) -> Self {
        Self{time: time, note: note, channel: channel, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u32) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl Tempo {
    pub fn new(time: u32, bpm: u32) -> Self{
        Self{time: time, bpm: bpm, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u32) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl Streamable<u8> for NoteOn {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let time:   u32 = (*VarLen::read(stream)?).val;
        let channel: u8 = stream.read().expect("Unexpected EOF") & 0x0F;
        let note:    u8 = *stream.read().expect("Unexpected EOF");
        let vel:     u8 = *stream.read().expect("Unexpected EOF");
          
        Ok(Box::new(Self::new(time, vel, note, channel)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new(self.time - self.previous).write(stream)?;

        stream.write(0x90 | self.channel)?;
        stream.write(self.note)?;
        stream.write(self.vel)?;
        Ok(()) 
    }
}

impl Streamable<u8> for NoteOff{
    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new(self.time - self.previous).write(stream)?;
        stream.write(0x80 | self.channel)?;
        stream.write(self.note)?; // Note off
        stream.write(0x0)?;
        Ok(())
    }

    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let time:   u32 = (*VarLen::read(stream)?).val;
        let channel: u8 = stream.read().expect("Unexpected EOF") & 0x0F;
        let note:    u8 = *stream.read().expect("Unexpected EOF");
        let _:     u8 = *stream.read().expect("Unexpected EOF");
          
        Ok(Box::new(Self::new(time, note, channel)))
    }
}

impl Streamable<u8> for Tempo {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let time:      u32 = (*VarLen::read(stream)?).val;
        let _:         u8 = *stream.read().expect("Unexpected EOF");
        let _:         u8 = *stream.read().expect("Unexpected EOF");
        let _:         u8 = *stream.read().expect("Unexpected EOF");
        let mut tempo: u32 = *stream.read().expect("Unexpected EOF") as u32;
        tempo = (tempo << 8) | *stream.read().expect("Unexpected EOF") as u32;
        tempo = (tempo << 8) | *stream.read().expect("Unexpected EOF") as u32;
          
        let bpm : u32 = 60_000_000 / tempo;

        Ok(Box::new(Self::new(time, bpm)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new(self.time - self.previous).write(stream)?;

        (0xFF_51 as u16).write(stream)?; // Tempo
        stream.write(0x03)?;

        let tempo: u32 = 60_000_000 / self.bpm;

        for i in 0..3 {
            stream.write((tempo >> (2 - i)*8) as u8)?;
        }
        Ok(()) 
    }
}
