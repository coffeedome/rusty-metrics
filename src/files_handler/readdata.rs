use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn process_data(input: &str) -> HashMap<String, i32> {
    // //PRE-PROCESS:::convert all to lower case:
    let cnt_str = preprocess_data(input);

    //PRE-PROCESS:::remove all line-breaks:
    let data_no_special_chars = preprocess_data_regex(cnt_str, "[$&+,:;=?@#|'<>.^*()%!-]", "");
    let data_ready = preprocess_data_regex(data_no_special_chars, "\n", " ");

    //PROCESS:::capture all 3-string sequences:
    return capture_word_sequence(data_ready);
}

//Function all uppers to lower case
fn preprocess_data(input: &str) -> String {
    let cnt_raw = fs::read_to_string(input)
        .unwrap()
        .to_lowercase()
        .to_string();

    return cnt_raw;
}

//Function cleans up string data based on regex match
fn preprocess_data_regex(data: String, regex_string: &str, replacement: &str) -> String {
    let regex_metachar = regex::escape(regex_string);
    let rgx_special_chars = Regex::new(&regex_metachar)
        .unwrap()
        .replace_all(&data, replacement)
        .to_string();
    return rgx_special_chars;
}

//Generate hashmap <sequence, count> //TODO: parametrize num spaces
fn capture_word_sequence(regex_string: String) -> HashMap<String, i32> {
    let num_space_regex = Regex::new(r"((?:\S+\s){2}\S+)\s").unwrap();

    let mut map = HashMap::new();
    for n in num_space_regex.captures_iter(&regex_string) {
        let count = map.entry(n[1].to_string()).or_insert(0);
        *count += 1;
    }

    return map;
}
