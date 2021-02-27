use regex::Regex;
use std::fs;

pub fn process_data(input: &str) {
    //input should be a chunk:
    
    let cnt = fs::read_to_string(input).expect("Input not found");

    //PRE-PROCESS:::replace all line breaks with spaces:
    let rgx_preprocess = Regex::new(r"\n").unwrap();
    let rgx_preprocess_output = rgx_preprocess.replace_all(&cnt, " ");

    //PROCESS:::capture all 3-string sequences: 
    let rgx = Regex::new(r"((?:\S+\s){2}\S+)\s").unwrap();
    
    //iterate over capture groups: https://docs.rs/regex/1.4.3/regex/#example-iterating-over-capture-groups
    for n in rgx.captures_iter(&rgx_preprocess_output) {
        println!("{:?}", &n[1]);
    }
}
