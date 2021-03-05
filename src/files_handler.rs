mod readdata;

use std::collections::HashMap;
use std::fs;

pub fn handle_files(datadirectory: &str, word_sequence: i32) {
    //Mutable global vector:
    let mut global_vec = Vec::new();

    //Scan for files in databucket directory:
    let paths = fs::read_dir(datadirectory).unwrap();

    //Loop through files in directory:
    for path in paths {
        //Push resulting hashmap onto vector
        global_vec.push(
            //fn- input:file str; output:hashmap
            readdata::process_data(path.unwrap().path().to_str().unwrap(), word_sequence),
        );
    }

    //Assert final vector is not empty:
    assert!(!global_vec.is_empty());

    //Explicit re-scan of directory to validate post-run integrity:
    let integrity_val = fs::read_dir(datadirectory).unwrap().count();
    assert!(global_vec.len() == integrity_val, true);

    //Generate final mapping
    let mut final_map = HashMap::new();
    for mapping in global_vec.iter() {
        final_map.extend(mapping);
    }

    //Generate sorted vector:
    let mut final_map_vec: Vec<(&String, &i32)> = final_map.into_iter().collect();
    final_map_vec.sort_by(|a, b| b.1.cmp(&a.1));

    print_metrics(final_map_vec, 100);
}

//print vector
fn print_metrics(datavec: Vec<(&String, &i32)>, count: usize) {
    for (k,v) in &datavec[..count] {
        println!("{} - {}", k, v);
    }
}
