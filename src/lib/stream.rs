use std::io::Read;

pub trait InStream<T> {
    fn read(&mut self) -> Option<&T>;
}

trait OutStream<T> {
    fn write(&self, val: T) -> ();
    fn flush(&self,) -> ();
}

pub struct FileByteInStream {
    pub contents: Vec<u8>,
    pub pos: usize, // this should be changed to use iterators
}

impl FileByteInStream {
    fn new(filename: String) -> Self {
        let mut contents = Vec::new();

        std::fs::File::open(filename)
            .expect("An error occured while opening file")
            .read_to_end(&mut contents)
            .expect("An error occured while reading file")
            ;

        Self { contents: contents, pos: 0 }
    }
}

impl InStream< u8> for FileByteInStream {
    fn read(&mut self) -> Option<&u8> {
        let ret = self.contents.iter().skip(self.pos).next();
        self.pos += 1;
        ret
    }
}

struct FileByteOutStream{
}

impl OutStream<String> for FileByteOutStream {
    fn write(&self, val: String) -> (){

    }

    fn flush(&self) -> () {

    }

}

impl FileByteOutStream {
    fn drop(&self) -> () {
        self.flush();
    }
}
