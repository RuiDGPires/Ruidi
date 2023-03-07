use std::io::{Read, BufWriter, Write};
use std::fs::File;
mod tests;

pub trait Sourceable<I> {
    fn from_stream<T: InStream<I>>(stream: T) -> Result<Box<Self>, String>;
    fn to_stream<T: OutStream<I>>(&self, stream: T)  -> Result<(), String>;
}

pub trait InStream<T> {
    fn read(&mut self) -> Option<&T>;
    fn peek(&mut self) -> Option<&T>;
}

pub trait OutStream<T> {
    fn write(&mut self, val: T) -> Result<(), String>;
    fn flush(&mut self,) -> Result<(), String>;
}

pub struct FileByteInStream {
    contents: Vec<u8>,
    pos: usize, // this should be changed to use iterators
}

pub struct VecByteStream {
    contents: Vec<u8>,
    pos: usize, // this should be changed to use iterators
}

impl FileByteInStream {
    pub fn new(filename: String) -> Self {
        let mut contents = Vec::new();

        File::open(filename)
            .expect("An error occured while opening file")
            .read_to_end(&mut contents)
            .expect("An error occured while reading file")
            ;

        Self { contents: contents, pos: 0 }
    }

}

impl InStream<u8> for FileByteInStream {
    fn read(&mut self) -> Option<&u8> {
        let ret = self.contents.iter().skip(self.pos).next();
        self.pos += 1;
        ret
    }

    fn peek(&mut self) -> Option<&u8> {
        let ret = self.contents.iter().skip(self.pos).next();
        ret
    }
}

impl VecByteStream {
    pub fn new(vec: Vec<u8>) -> Self {
        Self { contents: vec, pos: 0 }
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.contents
    }

    pub fn into_stream<T: OutStream<u8>>(self, stream: &mut T) -> Result<(), String>{
        for item in self.contents {
            stream.write(item)?;
        }

        stream.flush()
    }

    pub fn size(&self) -> usize {
        self.contents.len()
    }

    pub fn clear(&mut self) -> () {
        self.contents.clear();
        self.pos = 0;
    }
}

impl InStream<u8> for VecByteStream {
    fn read(&mut self) -> Option<&u8> {
        let ret = self.contents.iter().skip(self.pos).next();
        self.pos += 1;
        ret
    }
    fn peek(&mut self) -> Option<&u8> {
        let ret = self.contents.iter().skip(self.pos).next();
        ret
    }
}

impl OutStream<u8> for VecByteStream {
    fn write(&mut self, val: u8) -> Result<(), String> {
        self.contents.push(val);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> { Ok(()) }
}

pub struct FileByteOutStream {
    file: BufWriter<File>,
}

impl FileByteOutStream {
    pub fn new(filename: String) -> Self {
        let file = File::create(filename).expect("Unable to create file");
        let file = BufWriter::new(file);

        
        FileByteOutStream{file: file}
    }
}

impl OutStream<u8> for FileByteOutStream {
    fn write(&mut self, val: u8) -> Result<(), String>{
        match self.file.write(&[val]) {
            Ok(_)  => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }

    fn flush(&mut self) -> Result<(), String> {
        match self.file.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}

impl Drop for FileByteOutStream {
    fn drop(&mut self) -> () {
        let _ = self.flush();
    }
}
