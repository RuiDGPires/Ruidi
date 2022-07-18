use super::stream;

pub struct VarLen {
    pub val: u32,
    pub size: u8,
}

impl Streamable<u8> for VarLen {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        let mut val: u32 = 0;
        let mut size: u8 = 0;

        loop {
            match stream.read() {
                Some(v) => {
                    size += 1;
                    let tmp: u8 = v & 0x7F; 
                    val = (val << 7) | tmp as u32;
                    if v & 0x80 == 0 {break;}
                },
                None => {return Err(String::from("Unexpected end of file"));}
            }
        }
        Ok(Box::new(VarLen{val: val, size: size}))
    }
    
    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String>{
        let mut bytes: Vec<u8> = Vec::new(); 
        let mut val = self.val.clone();

        if val == 0 {
            bytes.push(0x00);
        } else {
            let mut first = true;
            while val != 0 {
                let byte: u8 = (val & 0x7F) as u8;

                if first {
                    bytes.insert(0, byte);
                    first = false;
                } else {
                    bytes.insert(0, byte | 0x80);
                }

                val >>= 7;
            }
        }

        for byte in bytes {
            stream.write(byte)?;
        }

        Ok(())
    }
}

impl VarLen {
    pub fn new(val: u32) -> Self {
        VarLen{val: val, size: 0}
    }
}

pub fn check_str<T: stream::InStream<u8>>(stream: &mut T, string: &str) -> bool {
    for c in string.chars() {
        if stream.read() != Some(&(c as u8)) {
            return false;
        }
    }
    true
}

pub trait Streamable<T> {
    fn read<S: stream::InStream<T>>(stream: &mut S) -> Result<Box<Self>, String>;
    fn write<S: stream::OutStream<T>>(self, stream: &mut S) -> Result<(), String>;
}



impl Streamable<u8> for u32 {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        let mut val: Self = 0;

        for i in 0..4 {
            match stream.read() {
                Some(v) => {val |= (*v as Self) << (3 - i)*8;}
                None   => return Err(String::from("Unexpected end of file")),
            }
        }

        Ok(Box::new(val))
    } 

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        for i in 0..4 {
            stream.write((self >> (3 - i)*8) as u8 & 0xFF)?;
        }
        
        Ok(())
    }
}

impl Streamable<u8> for u16 {
    fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
        let mut val: Self = 0;

        for i in 0..2 {
            match stream.read() {
                Some(v) => {val |= (*v as Self) << (1 - i)*8;}
                None   => return Err(String::from("Unexpected end of file")),
            }
        }

        Ok(Box::new(val))
    } 

    fn write<T: stream::OutStream<u8>>(self, stream: &mut T) -> Result<(), String> {
        for i in 0..2 {
            stream.write((self >> (1 - i)*8) as u8 & 0xFF)?;
        }
        
        Ok(())
    }
}
