use super::stream;

pub struct VarLen {
    pub val: u32,
    pub size: u8,
}

impl VarLen {
    pub fn read<T: stream::InStream<u8>>(stream: &mut T) -> Result<Box<Self>, String> {
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
    
    // TODO -> Change to stream
    pub fn write(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new(); 
        let mut val = self.val.clone();

        if val == 0 {
            bytes.push(0x00);
        }else{
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
        bytes
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

pub fn get_u32<T: stream::InStream<u8>>(stream: &mut T) -> Result<u32, String> {
    let mut val: u32 = 0;

    for i in 0..4 {
        match stream.read() {
            Some(v) => {val |= (*v as u32) << (3 - i)*8;}
            None   => return Err(String::from("Unexpected end of file")),
        }
    }

    Ok(val)
} 

pub fn get_u16<T: stream::InStream<u8>>(stream: &mut T) -> Result<u16, String> {
    let mut val: u16 = 0;

    for i in 0..2 {
        match stream.read() {
            Some(v) => {val |= (*v as u16) << (1 - i)*8;}
            None   => return Err(String::from("Unexpected end of file")),
        }
    }

    Ok(val)
} 
