use midi::{io::stream::{FileByteOutStream, OutStream}};

fn main() {
    let mut out_stream = FileByteOutStream::new(String::from("/tmp/a.txt"));
    
    out_stream.write(0).expect("Unable to write file");
    out_stream.write(1).expect("Unable to write file");
}
