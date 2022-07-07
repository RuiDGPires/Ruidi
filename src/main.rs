mod midiobj;

fn main() {
    let obj: midiobj::MidiObj = midiobj::new_midi_object(); 
    println!("{:?}", obj);
}
