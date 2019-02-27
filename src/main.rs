use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
// use std::io;

mod edf;
use edf::document::Document;

fn main() {
    // let doc = Document::new();

    // return;

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

    let mut input_json = String::new();
    let encoding_parsing_err = file.read_to_string(&mut input_json).is_err();

    if encoding_parsing_err {
        println!("Encoding error");
        return;
    }

    let doc = Document::from_json(&input_json);

    println!("title:        '{}'", doc.metadata.title);
    println!("author:       '{}'", doc.metadata.author);
    println!("lang:         '{}'", doc.metadata.lang);
    println!("generator:    '{}'", doc.metadata.generator);
    println!("pages:         {}", doc.pages.len());
    println!("page format:  '{}'", doc.pages[0].format);
    println!("page fragms:   {}", doc.pages[0].fragments.len());

    // let mut chars_iter = input_json.chars();

    // while let Some(ch) = chars_iter.next() {
    //     print!("{}", ch);
    // }
}