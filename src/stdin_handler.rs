use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_stdin() {
    //Stdin to buffer:
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Unable to read stdin to string");

    //Generate buffer file:
    let mut buffer_file =
        File::create("databucket/buffer_file.txt").expect("Unable to create file");
    buffer_file
        .write_all(&buffer.as_bytes())
        .expect("Unable to write to buffer file");
}
