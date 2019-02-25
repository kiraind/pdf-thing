use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;

struct AtomicText {
    text: String,
    x: f64,
    y: f64,
    width: f64,
    classes: Vec<String>
}

fn main() {
    let filename = "example.edf.json";

    let mut file = match File::open(&filename) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!(
            "couldn't open {}: {}",
            filename,
            why.description()
        ),
        Ok(file) => file,
    };

    let mut input_buffer = String::new();
    file.read_to_string(&mut input_buffer);

    let mut bytes_iter = input_buffer.chars();

    while let Some(ch) = bytes_iter.next() {
        print!("{}", ch);
    }
}