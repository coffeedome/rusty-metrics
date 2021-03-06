use std::collections::HashMap;
use std::fs;
use crate::process_data::process_data;

pub fn handle_files(datadirectory: &str, word_sequence: i32) -> Vec<HashMap<String, i32>> {
    //Mutable global vector:
    let mut global_vec = Vec::new();

    //Scan for files in databucket directory:
    let paths = fs::read_dir(datadirectory).unwrap();

    //Loop through files in directory:
    for path in paths {
        //Push resulting hashmap onto vector
        global_vec.push(
            //fn- input:file str; output:hashmap
            process_data(path.unwrap().path().to_str().unwrap(), word_sequence),
        );
    }

    return global_vec;
}