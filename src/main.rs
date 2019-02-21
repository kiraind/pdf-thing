use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

fn main() {
    let path = Path::new("test.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            why.description()
        ),
        Ok(file) => file,
    };

    let mut input_buffer = Vec::new();
    file.read_to_end(&mut input_buffer).unwrap();

    let mut output_buffer = Vec::new();

    let mut included = String::new();
    let mut in_file_name = false;

    for (i, byte) in input_buffer.iter().enumerate() {
        if *byte > 127 {
            println!("Warning: Invalid character 0x{:02X}", byte);
        }

        if *byte == ('%' as u8) && input_buffer[i+1] == ('{' as u8) {
            in_file_name = true;
        }

        if !in_file_name {
            output_buffer.push(*byte);
            // print!("{}", *byte as char);
        } else {
            included.push(*byte as char);

            if *byte == ('}' as u8) {
                in_file_name = false;

                let filename = included.trim_matches(|c| c == '%' || c == '{' || c == '}');

                println!("Including file: '{}'", filename );

                // Open the path in read-only mode, returns `io::Result<File>`
                let mut file = match File::open(filename) {
                    Err(why) => panic!(
                        "couldn't open {}: {}",
                        filename,
                        why.description()
                    ),
                    Ok(file) => file,
                };

                let mut included_buffer = Vec::new();
                file.read_to_end(&mut included_buffer).unwrap();

                output_buffer.append(&mut included_buffer);
            }
        }

        // println!("{:02X}", byte);
        
    }

    // println!("Resulting file:");
    // for byte in output_buffer {
    //     print!("{}", byte as char);
    // }

    let mut output_file = File::create("output.txt").unwrap();

    output_file.write_all(&output_buffer).unwrap();

    println!();
}