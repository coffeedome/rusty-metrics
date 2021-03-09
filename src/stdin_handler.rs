use crate::process_data::process_data;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn handle_stdin(word_sequence: i32) -> Vec<HashMap<String, i32>> {
    //Stdin to buffer:
    let mut global_vec = Vec::new();

    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    global_vec.push(process_data(
        input.unwrap().unwrap().as_str(),
        word_sequence,
    ));

    return global_vec;
}
