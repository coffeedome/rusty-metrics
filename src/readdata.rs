use regex::Regex;
use std::fs;

pub fn process_data(input: &str) {
    let cnt = fs::read_to_string(input).expect("Input not found");

    let rgx = Regex::new(r".*?\s.*?\s.*?\S*").unwrap();

    let cnt_fields: Vec<&str> = rgx.split(&cnt).collect();

    println!("{:?}", cnt_fields);
}
