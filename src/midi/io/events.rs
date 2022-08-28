use super::util::Streamable;
use super::stream;
use super::util::VarLen;

#[derive(Copy, Clone)]
pub struct NoteOn {
    pub time: u64,
    pub vel: u8,
    pub note: u8,
    pub channel: u8,

    previous: u64,
}

#[derive(Copy, Clone)]
pub struct NoteOff {
    pub time: u64,
    pub note: u8,
    pub channel: u8,

    previous: u64,
}

#[derive(Copy, Clone)]
pub struct Tempo {
    pub time: u64,
    pub bpm: u32,

    previous: u64
}

#[derive(Copy, Clone)]
pub struct TimeSignature {
    pub time: u64,
    pub numerator: u8,
    pub denominator: u8,

    previous: u64
}

#[derive(Copy, Clone)]
pub struct KeySignature {
    pub time: u64,
    pub accidentals: u8,
    pub minor: bool,

    previous: u64,
}

#[derive(Copy, Clone)]
pub struct ControlChange {
    pub time: u64,
    pub channel: u8,
    pub id: u8,
    pub val: u8,

    previous: u64,
}

impl NoteOn {
    pub const CODE: u8 = 0x90;

    pub fn with_time(&mut self, time: u64) -> &Self {
        self.time = time;
        self
    }

    pub fn new(time: u64, vel: u8, note: u8, channel: u8) -> Self {
        Self{time: time, vel: vel, note: note, channel: channel, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u64) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl NoteOff {
    pub const CODE: u8 = 0x80;

    pub fn with_time(&mut self, time: u64) -> &Self {
        self.time = time;
        self
    }

    pub fn new(time: u64, note: u8, channel: u8) -> Self {
        Self{time: time, note: note, channel: channel, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u64) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl Tempo {
    pub const CODE: u16 = 0x5103;

    pub fn with_time(&mut self, time: u64) -> &Self {
        self.time = time;
        self
    }

    pub fn new(time: u64, bpm: u32) -> Self{
        Self{time: time, bpm: bpm, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u64) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl TimeSignature {
    pub const CODE: u32 = 0x5804;

    pub fn with_time(&mut self, time: u64) -> &Self {
        self.time = time;
        self
    }

    pub fn new(time: u64, numerator: u8, denominator: u8) -> Self{
        Self{time: time, numerator: numerator, denominator: denominator, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u64) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }

    fn inv(val: u8) -> Result<u8, String>{
        match val {
            16 => Ok(4),
            8 => Ok(3),
            4 => Ok(2),
            2 => Ok(1),
            1 => Ok(0),
            _ => Err(String::from("Invalid Time Signature"))
        } 
    }

    fn pow(val: u8) -> Result<u8, String>{
        match val {
            4 => Ok(16),
            3 => Ok(8),
            2 => Ok(4),
            1 => Ok(2),
            0 => Ok(1),
            _ => Err(String::from("Invalid Time Signature"))
        } 
    }
}

impl KeySignature {
    pub const CODE: u32 = 0x5902;

    pub fn with_time(&mut self, time: u64) -> &Self {
        self.time = time;
        self
    }

    pub fn new(time: u64, accidentals: u8, minor: bool) -> Self{
        Self{time: time, accidentals: accidentals, minor: minor, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u64) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl ControlChange {
    pub const CODE: u8 = 0xB0;

    pub fn with_time(&mut self, time: u64) -> &Self {
        self.time = time;
        self
    }

    pub fn new(time: u64, id: u8, val: u8, channel: u8) -> Self{
        Self{time: time, id: id, val: val, channel: channel, previous: 0}
    }

    pub fn on_tick(&mut self, time: &mut u64) -> &Self {
        self.previous = *time;
        *time = self.time;

        self
    }
}

impl Streamable<u8> for NoteOn {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let channel: u8 = stream.read().expect("Unexpected EOF") & 0x0F;
        let note:    u8 = *stream.read().expect("Unexpected EOF");
        let vel:     u8 = *stream.read().expect("Unexpected EOF");
          
        Ok(Box::new(Self::new(0, vel, note, channel)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new((self.time - self.previous) as u32).write(stream)?;

        stream.write(Self::CODE | self.channel)?;
        stream.write(self.note)?;
        stream.write(self.vel)?;
        Ok(()) 
    }
}

impl Streamable<u8> for NoteOff{
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let channel: u8 = stream.read().expect("Unexpected EOF") & 0x0F;
        let note:    u8 = *stream.read().expect("Unexpected EOF");
        let _:     u8 = *stream.read().expect("Unexpected EOF");
          
        Ok(Box::new(Self::new(0, note, channel)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new((self.time - self.previous) as u32).write(stream)?;
        stream.write(Self::CODE | self.channel)?;
        stream.write(self.note)?; // Note off
        stream.write(0x0)?;
        Ok(())
    }
}

impl Streamable<u8> for Tempo {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let mut tempo: u32 = *stream.read().expect("Unexpected EOF") as u32;
        tempo = (tempo << 8) | *stream.read().expect("Unexpected EOF") as u32;
        tempo = (tempo << 8) | *stream.read().expect("Unexpected EOF") as u32;
          
        let bpm : u32 = 60_000_000 / tempo;

        Ok(Box::new(Self::new(0, bpm)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new((self.time - self.previous) as u32).write(stream)?;

        stream.write(0xFF)?;
        (Self::CODE as u16).write(stream)?;

        let tempo: u32 = 60_000_000 / self.bpm;

        for i in 0..3 {
            stream.write((tempo >> (2 - i)*8) as u8)?;
        }
        Ok(()) 
    }
}

impl Streamable<u8> for TimeSignature {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let num:       u8 = *stream.read().expect("Unexpected EOF");
        let den:       u8 = *stream.read().expect("Unexpected EOF");

        let _:         u8 = *stream.read().expect("Unexpected EOF");
        let _:         u8 = *stream.read().expect("Unexpected EOF");

        Ok(Box::new(Self::new(0, num, Self::pow(den)?)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new((self.time - self.previous) as u32).write(stream)?;

        stream.write(0xFF)?; // Time signature
        (Self::CODE as u16).write(stream)?;
        stream.write(self.numerator)?;
        stream.write(Self::inv(self.denominator)?)?;
        stream.write(0x18)?;
        stream.write(0x08)?;

        Ok(()) 
    }
}

impl Streamable<u8> for KeySignature {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let sf:         u8 = *stream.read().expect("Unexpected EOF");
        let mi:         u8 = *stream.read().expect("Unexpected EOF");

        Ok(Box::new(Self::new(0, sf, mi != 0)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new((self.time - self.previous) as u32).write(stream)?;

        stream.write(0xFF)?; // Time signature
        (Self::CODE as u16).write(stream)?;
        stream.write(self.accidentals)?;
        stream.write(self.minor as u8)?;

        Ok(()) 
    }
}

impl Streamable<u8> for ControlChange {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        // TODO : Missing running status
        let channel:        u8 = *stream.read().expect("Unexpected EOF") & 0x0F;
        let id:        u8 = *stream.read().expect("Unexpected EOF");
        let val:       u8 = *stream.read().expect("Unexpected EOF");

        Ok(Box::new(Self::new(0, id, val, channel)))
    }

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        VarLen::new((self.time - self.previous) as u32).write(stream)?;

        stream.write(Self::CODE | self.channel)?; // Time signature
        stream.write(self.id  & 0x7F)?;
        stream.write(self.val & 0x7F)?;

        Ok(()) 
    }
}
