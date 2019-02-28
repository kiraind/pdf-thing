#[macro_use]
extern crate serde_derive;
extern crate printpdf;

use printpdf::*;

use serde_json::Result;

use std::fs::File;
use std::io::BufWriter;

use std::error::Error;
use std::io::prelude::*;
// use std::io;

mod edf;
use edf::document::Document;

const FONT_BASELINE_SHIFTS: [f64; 2] = [ 0.86, 0.86 ];

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
        
    if let Err(err) = res {
        println!("JSON parsing error {}", err);
        panic!();
    }

    let doc = res.unwrap();

    println!("title:        '{}'", doc.meta.title);
    println!("author:       '{}'", doc.meta.author);
    println!("lang:         '{}'", doc.meta.lang);
    println!("generator:    '{}'", doc.meta.generator);
    println!("pages:         {}", doc.pages.len());

    // generating pdf

    // create doc & first page
    let (pdf_doc, page1, layer1) = PdfDocument::new(doc.meta.title, Mm(210.0), Mm(297.0), "Layer 1".to_string());
    let current_layer = pdf_doc.get_page(page1).get_layer(layer1);

    // adding fonts
    let mut fonts = vec![];

    let mut font_reader1 = std::io::Cursor::new(include_bytes!("../fonts/Montserrat-SemiBold.ttf").as_ref());
    let font1 = pdf_doc.add_external_font(&mut font_reader1).unwrap();
    fonts.push(&font1);

    let mut font_reader2 = std::io::Cursor::new(include_bytes!("../fonts/OpenSans-Regular.ttf").as_ref());
    let font2 = pdf_doc.add_external_font(&mut font_reader2).unwrap();
    fonts.push(&font2);

    // write text

    let page = &doc.pages[0];

    

    for fr in page.fragments.iter() {
        for atomic in fr.atomics.iter() {
            let x = fr.x + atomic.x;
            let y = fr.y + atomic.y;

            let mut classes = fr.classes.clone();
            classes.append( &mut atomic.classes.clone() );

            let mut font_size = 12;
            let mut font_id = 0;

            for class in classes {
                match class.as_str() {
                    "main" => {
                        font_size = doc.style.mainFontSize;
                        font_id = 1;
                    },

                    "heading1" => {
                        font_size = doc.style.h1_fontSize;
                        font_id = 0;
                    },
                    "heading2" => {
                        font_size = doc.style.h2_fontSize;
                        font_id = 0;
                    },
                    "heading3" => {
                        font_size = doc.style.h3_fontSize;
                        font_id = 0;
                    },
                    "heading4" => {
                        font_size = doc.style.h4_fontSize;
                        font_id = 0;
                    },
                    "heading5" => {
                        font_size = doc.style.h5_fontSize;
                        font_id = 0;
                    },
                    "heading6" => {
                        font_size = doc.style.h6_fontSize;
                        font_id = 0;
                    },

                    _ => {}
                }
            }

            current_layer.use_text(
                atomic.text.chars.clone(),
                font_size as i64,
                Mm(x),
                Mm(297.0) - Mm(y) - Pt(font_size as f64 * FONT_BASELINE_SHIFTS[font_id]).into(),
                fonts[ font_id ],
            );
        }
    }

    // saving pdf

    pdf_doc.save(&mut BufWriter::new(File::create("test_conv.pdf").unwrap())).unwrap();
}