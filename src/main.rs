#[macro_use]
extern crate serde_derive;
use serde_json::Result;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
// use std::io;

mod edf;
use edf::document::Document;

use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u32)]
enum Gender {
    Male = 0,
    Female = 1,
}
impl PartialEq for Gender {
    fn eq(&self, other: &Gender) -> bool {
        match (self, other) {
            (&Gender::Female, &Gender::Female) => true,
            (&Gender::Male, &Gender::Male) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Location {
    country: String,
    zipcode: String,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    location: Location,
    phones: Vec<String>,
    gender: Gender,
}

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

    let res: Result<Document> = serde_json::from_str(&input_json);

    if let Ok(doc) = res {
        println!("title:        '{}'", doc.meta.title);
        println!("author:       '{}'", doc.meta.author);
        println!("lang:         '{}'", doc.meta.lang);
        println!("generator:    '{}'", doc.meta.generator);
        println!("pages:         {}", doc.pages.len());
    } else if let Err(err) = res {
        println!("JSON parsing error {}", err);
    } 
}