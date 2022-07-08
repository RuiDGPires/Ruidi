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

    pub fn write(&self) -> &[u8] {
        &[]
    }
}


