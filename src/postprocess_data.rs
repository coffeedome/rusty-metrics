use std::collections::HashMap;

pub fn post_process(input_vector: Vec<HashMap<String, i32>>){

    //Generate final mapping
    let mut final_map = HashMap::new();
    for mapping in input_vector.iter() {
        final_map.extend(mapping);
    }

    //Generate sorted vector:
    let mut final_map_vec: Vec<(&String, &i32)> = final_map.into_iter().collect();
    final_map_vec.sort_by(|a, b| b.1.cmp(&a.1));

    //Handle cases where result is less than 100:
    let final_map_vec_length = final_map_vec.len();

    if final_map_vec_length >= 100 {
        print_metrics(final_map_vec, 100);
    } else {
        print_metrics(final_map_vec, final_map_vec_length);
    }
    
}

//print vector
fn print_metrics(datavec: Vec<(&String, &i32)>, count: usize) {
    for (k,v) in &datavec[..count] {
        println!("{} - {}", k, v);
    }
}

// #[test]
// fn test_print_metrics() {
//     let datavec: Vec<&String, &i32> = [("this is a test", 20)];
//     print_metrics();
// }
