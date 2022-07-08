pub struct VarLen {
    pub val: u32,
}

impl VarLen {
    pub fn read(slice: &[u8]) -> VarLen {
        let mut val: u32 = 0;
        for &item in slice {
            let tmp: u8 = item & 0x7F; 
            val = (val << 7) | tmp as u32;
            if item & 0x80 == 0 {break;}
        }
        VarLen{val: val}
    }

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


